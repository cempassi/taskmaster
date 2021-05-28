use super::{inter::Inter, task::Task};
use nix::{
    self,
    sys::signal::{kill, Signal},
    unistd::Pid,
};
use serde::Serialize;
use std::{
    fmt::{self, Debug, Display, Formatter},
    process::{Child, ExitStatus},
    sync::mpsc::Sender,
    time,
};

#[derive(Copy, Clone, Serialize, PartialEq)]
pub enum Status {
    Inactive,
    Active,
    Reloading,
    Failing,
    Finished,
    Failed,
    Stopping,
    Stopped,
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            Status::Inactive => "inactive",
            Status::Active => "active",
            Status::Reloading => "reloading",
            Status::Failing => "failing",
            Status::Finished => "finished",
            Status::Failed => "failed",
            Status::Stopping => "stopping",
            Status::Stopped => "stopped",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
struct RunningChild {
    child: Child,

    started_at: time::Instant,
    startup_time: time::Duration,

    stopsignal: Signal,
    stopdelay: time::Duration,
}

impl RunningChild {
    fn new(
        child: Child,
        started_at: time::Instant,
        startup_time: time::Duration,
        stopsignal: Signal,
        stopdelay: time::Duration,
    ) -> Self {
        Self {
            child,
            started_at,
            startup_time,
            stopsignal,
            stopdelay,
        }
    }

    fn try_wait(&mut self) -> Result<Option<ExitStatus>, std::io::Error> {
        self.child.try_wait()
    }

    #[allow(clippy::cast_possible_wrap)]
    fn stop(self) -> Result<StoppingChild, nix::Error> {
        let pid = self.child.id() as i32;

        kill(Pid::from_raw(pid), self.stopsignal)?;
        Ok(StoppingChild::new(
            self.child,
            time::Instant::now(),
            self.stopdelay,
        ))
    }
}

#[derive(Debug)]
struct StoppingChild {
    child: Child,
    stopped_at: time::Instant,
    timeout: time::Duration,
}

impl StoppingChild {
    fn new(child: Child, stopped_at: time::Instant, timeout: time::Duration) -> StoppingChild {
        StoppingChild {
            child,
            stopped_at,
            timeout,
        }
    }

    fn try_wait(&mut self) -> Result<Option<ExitStatus>, std::io::Error> {
        self.child.try_wait()
    }

    fn kill(&mut self) -> Result<(), std::io::Error> {
        self.child.kill()
    }
}

#[derive(Debug)]
struct FinishedChild {
    child: Child,
    status: ExitStatus,
}

impl FinishedChild {
    fn new(child: Child, status: ExitStatus) -> FinishedChild {
        FinishedChild { child, status }
    }
}

#[derive(Serialize)]
pub struct Monitor {
    id: String,
    task: Task,

    #[serde(skip)]
    children_pid: Vec<u32>,

    #[serde(skip)]
    state: Status,

    #[serde(skip)]
    sender: Sender<Inter>,

    #[serde(skip)]
    running: Vec<RunningChild>,

    #[serde(skip)]
    stopping: Vec<StoppingChild>,

    #[serde(skip)]
    finished: Vec<FinishedChild>,
}

// impl Drop for Monitor {
//     fn drop(&mut self) {
//         for child in &mut self.children {
//             child.kill().expect("cannot kill children");
//         }
//         self.children.clear();
//     }
// }

impl Monitor {
    // Only create Monitoring struct
    pub fn new_only(id: String, task: Task, sender: Sender<Inter>) -> Self {
        Monitor {
            id,
            task,
            children_pid: Vec::new(),
            state: Status::Inactive,
            sender,
            running: Vec::new(),
            stopping: Vec::new(),
            finished: Vec::new(),
        }
    }

    // Create new Monitoring struct and start the task if required
    pub fn new(id: String, task: Task, sender: Sender<Inter>) -> Self {
        let mut mon = Monitor::new_only(id, task, sender);
        if mon.task.autostart {
            mon.start();
        }
        mon
    }

    fn change_state(&mut self, status: Status) {
        self.state = status;
    }

    pub fn start(&mut self) {
        let current_status = self.status();
        if startable_state(current_status) {
            self.start_raw();
        } else {
            log::warn!("[{}] already started", self.id);
        }
    }

    fn start_raw(&mut self) {
        log::debug!("[{}] starting ...", self.id);
        let mut running_children = self.spaw_children();

        self.running.append(&mut running_children);
        self.change_state(Status::Active);
    }

    fn spaw_children(&self) -> Vec<RunningChild> {
        let mut command = self.task.get_command();
        let num_process = self.task.numprocess;
        let mut running_children = Vec::new();

        for _ in 0..num_process {
            let child = command.spawn().expect("Cannot start child");
            let running_child = RunningChild::new(
                child,
                time::Instant::now(),
                time::Duration::from_secs(self.task.successdelay.into()),
                self.task.stopsignal,
                time::Duration::from_secs(self.task.stopdelay.into()),
            );
            running_children.push(running_child);
        }
        running_children
    }

    pub fn status(&self) -> Status {
        self.state
    }

    pub fn reload(&mut self, task: Task) {
        if self.task != task {
            log::debug!("[{}] reloading ...", self.id);
            self.change_state(Status::Reloading);
            self.task = task;
            if self.task.autostart {
                self.start();
            } else {
                self.change_state(Status::Inactive);
            }
        }
    }

    pub fn stop(&mut self) {
        self.change_state(Status::Stopping);
        while !self.running.is_empty() {
            let chld = self.running.remove(0);
            let stopping_child = chld.stop().unwrap();
            self.stopping.push(stopping_child);
        }
        self.running.clear();
    }

    pub fn get_task(&self) -> &Task {
        &self.task
    }

    fn handle_finished_child(&mut self, status: ExitStatus) {
        if let Some(code) = status.code() {
            if !self.task.exitcodes.iter().any(|&wanted| wanted == code) {
                log::debug!(
                    "[{}] child exited with unexpeced status code {}",
                    self.id,
                    code
                );
                self.state = Status::Failing;
            }
        } else {
            log::warn!("[{}] unexpected exit status {:?}", self.id, status);
            self.state = Status::Failing;
        }
    }

    fn update_finished_task_status(&mut self) {
        if self.state == Status::Failing {
            self.state = Status::Failed;
        } else {
            self.state = Status::Finished;
        }
    }

    pub fn has_finished(&self) -> bool {
        self.running.is_empty() && self.stopping.is_empty()
    }

    pub fn cycle(&mut self, sender: &Sender<Inter>) {
        if !self.running.is_empty() {
            self.cycle_running().unwrap();
        }
        if !self.stopping.is_empty() {
            self.cycle_stopping().unwrap();
        }
        if !self.finished.is_empty() {
            self.cycle_finished(sender);
        }
    }

    // cycle_running check for Child that has terminated
    fn cycle_running(&mut self) -> Result<(), std::io::Error> {
        let mut i = 0;

        while i != self.running.len() {
            if let Some(st) = self.running[i].try_wait()? {
                let e = self.running.remove(i);
                self.finished.push(FinishedChild::new(e.child, st));
            } else {
                i += 1;
            }
        }

        Ok(())
    }

    // cycle_stopping check for signaled children if they've stop
    fn cycle_stopping(&mut self) -> Result<(), std::io::Error> {
        let mut killed: Vec<Child> = Vec::new();
        let mut i = 0;

        while i != self.stopping.len() {
            let chld = &mut self.stopping[i];
            let timeout = chld.timeout;
            let during = chld.stopped_at.elapsed();

            if let Some(st) = chld.try_wait()? {
                let e = self.stopping.remove(i);
                self.finished.push(FinishedChild::new(e.child, st));
            } else if during > timeout {
                chld.kill()?;
                let e = self.stopping.remove(i);
                killed.push(e.child);
            } else {
                i += 1;
            }
        }

        while !killed.is_empty() {
            let mut chld = killed.remove(0);
            let st = chld.wait()?;
            self.finished.push(FinishedChild::new(chld, st));
        }
        Ok(())
    }

    fn cycle_finished(&mut self, _sender: &Sender<Inter>) {
        // while !self.finished.is_empty() {
        //     let e = self.finished.remove(0);
        //     sender.send(e.into()).unwrap();
        // }
        unimplemented!();
    }
}

impl Debug for Monitor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Ok(s) = serde_json::to_string(self) {
            write!(f, "{}", s)
        } else {
            Err(fmt::Error)
        }
    }
}

fn startable_state(state: Status) -> bool {
    state == Status::Inactive
        || state == Status::Finished
        || state == Status::Failed
        || state == Status::Stopped
}

#[cfg(test)]
mod monitor_suite {
    use super::{startable_state, Status};

    #[test]
    fn test_startable_state() {
        assert_eq!(startable_state(Status::Active), false);
        assert_eq!(startable_state(Status::Reloading), false);
        assert_eq!(startable_state(Status::Failing), false);
        assert_eq!(startable_state(Status::Stopping), false);

        assert_eq!(startable_state(Status::Finished), true);
        assert_eq!(startable_state(Status::Inactive), true);
        assert_eq!(startable_state(Status::Failed), true);
        assert_eq!(startable_state(Status::Stopped), true);
    }
}
