use super::{inter::Inter, manager::WaitChildren, task::Task};
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
    namespace: String,
    child: Child,

    stopdelay: time::Duration,
    stopsignal: Signal,
}

impl RunningChild {
    fn try_wait(&mut self) -> Result<Option<ExitStatus>, std::io::Error> {
        self.child.try_wait()
    }

    #[allow(clippy::cast_possible_wrap)]
    fn stop(self) -> Result<StoppingChild, nix::Error> {
        let pid = self.child.id() as i32;

        kill(Pid::from_raw(pid), self.stopsignal)?;
        Ok(StoppingChild::new(
            self.namespace,
            self.child,
            time::Instant::now(),
            self.stopdelay,
        ))
    }
}

impl From<WaitChildren> for Vec<RunningChild> {
    fn from(data: WaitChildren) -> Vec<RunningChild> {
        let mut res: Vec<RunningChild> = Vec::new();

        for child in data.children {
            res.push(RunningChild {
                namespace: data.namespace.clone(),
                child,
                stopdelay: data.stopdelay,
                stopsignal: data.stopsignal,
            })
        }

        res
    }
}

#[derive(Debug)]
struct StoppingChild {
    namespace: String,
    child: Child,
    signaled_at: time::Instant,
    timeout: time::Duration,
}

impl StoppingChild {
    fn new(
        namespace: String,
        child: Child,
        instant: time::Instant,
        timeout: time::Duration,
    ) -> StoppingChild {
        StoppingChild {
            namespace,
            child,
            signaled_at: instant,
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
    namespace: String,
    child: Child,
    status: ExitStatus,
}

impl FinishedChild {
    fn new(namespace: String, child: Child, status: ExitStatus) -> FinishedChild {
        FinishedChild {
            namespace,
            child,
            status,
        }
    }
}

impl From<FinishedChild> for super::inter::Inter {
    fn from(finished: FinishedChild) -> Self {
        Self::ChildHasExited(finished.namespace, finished.child.id(), finished.status)
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
        if current_status == Status::Inactive
            || current_status == Status::Finished
            || current_status == Status::Failed
        {
            self.start_raw();
        } else {
            log::warn!("[{}] already started", self.id);
        }
    }

    fn start_raw(&mut self) {
        log::debug!("[{}] starting ...", self.id);
        let mut children = self.task.run();

        self.children_pid = children.iter_mut().map(|chld| chld.id()).collect();
        self.sender
            .send(Inter::ChildrenToWait(WaitChildren::new(
                self.id.clone(),
                children,
                time::Duration::from_secs(self.task.stopdelay.into()),
                self.task.stopsignal,
            )))
            .unwrap();
        self.change_state(Status::Active);
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

    pub fn ev_child_has_exited(&mut self, pid: u32, status: ExitStatus) -> bool {
        if self.children_pid.iter().any(|&chld_pid| chld_pid == pid) {
            self.children_pid.retain(|&chld_pid| chld_pid != pid);
            self.handle_finished_child(status);
            if self.children_pid.is_empty() {
                self.update_finished_task_status();
            }
            true
        } else {
            log::error!("[{}] pid {} is not registred to this monitor", self.id, pid);
            false
        }
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
                self.finished
                    .push(FinishedChild::new(e.namespace, e.child, st));
            } else {
                i += 1;
            }
        }

        Ok(())
    }

    // cycle_stopping check for signaled children if they've stop
    fn cycle_stopping(&mut self) -> Result<(), std::io::Error> {
        let mut killed: Vec<(String, Child)> = Vec::new();
        let mut i = 0;

        while i != self.stopping.len() {
            let chld = &mut self.stopping[i];
            let timeout = chld.timeout;
            let during = chld.signaled_at.elapsed();

            if let Some(st) = chld.try_wait()? {
                let e = self.stopping.remove(i);
                self.finished
                    .push(FinishedChild::new(e.namespace, e.child, st));
            } else if during > timeout {
                chld.kill()?;
                let e = self.stopping.remove(i);
                killed.push((e.namespace, e.child));
            } else {
                i += 1;
            }
        }

        while !killed.is_empty() {
            let mut chld = killed.remove(0);
            let st = chld.1.wait()?;
            self.finished.push(FinishedChild::new(chld.0, chld.1, st));
        }
        Ok(())
    }

    fn cycle_finished(&mut self, sender: &Sender<Inter>) {
        while !self.finished.is_empty() {
            let e = self.finished.remove(0);
            sender.send(e.into()).unwrap();
        }
    }

    pub fn add_children_to_wait(&mut self, to_wait: WaitChildren) {
        self.running.append(&mut to_wait.into());
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
