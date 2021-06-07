use super::{
    inter::Inter,
    relaunch::Relaunch,
    task::{get_current_timestamp, Task},
};
use nix::{
    self,
    sys::signal::{kill, Signal},
    unistd::Pid,
};
use serde::Serialize;
use std::{
    fmt::{self, Debug, Display, Formatter},
    process::{Child, Command, ExitStatus},
    sync::mpsc::Sender,
    time,
};

#[derive(Copy, Clone, Serialize, PartialEq, Debug)]
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
            self.started_at,
            self.startup_time,
            time::Instant::now(),
            self.stopdelay,
        ))
    }
}

#[derive(Debug)]
struct StoppingChild {
    child: Child,
    started_at: time::Instant,
    startup_time: time::Duration,
    stopped_at: time::Instant,
    timeout: time::Duration,
}

impl StoppingChild {
    fn new(
        child: Child,
        started_at: time::Instant,
        startup_time: time::Duration,
        stopped_at: time::Instant,
        timeout: time::Duration,
    ) -> StoppingChild {
        StoppingChild {
            child,
            started_at,
            startup_time,
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
    execution_time: time::Duration,
    startup_time: time::Duration,
}

impl FinishedChild {
    fn new(
        child: Child,
        status: ExitStatus,
        execution_time: time::Duration,
        startup_time: time::Duration,
    ) -> FinishedChild {
        FinishedChild {
            child,
            status,
            execution_time,
            startup_time,
        }
    }
}

#[derive(Serialize)]
pub struct Monitor {
    id: String,
    task: Task,
    retry_count: u32,
    spawned_children: u32,

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
            retry_count: 0,
            spawned_children: 0,
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
        log::info!("[{}] starting ...", self.id);
        self.retry_count = 0;
        let mut running_children = self.spawn_children();

        self.running.append(&mut running_children);
        self.change_state(Status::Active);
    }

    fn spawn_children(&mut self) -> Vec<RunningChild> {
        let timestamp = get_current_timestamp();
        let num_process = self.task.numprocess;
        let mut running_children = Vec::new();

        for _ in 0..num_process {
            let id = self.increase_spawned_children_counter();
            let mut command = self.task.get_command(id, timestamp);

            let running_child = spawn_child(
                &mut command,
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
            log::info!("[{}] reloading ...", self.id);

            let need_to_start = task.autostart || !self.running.is_empty();

            self.change_state(Status::Reloading);
            if !self.running.is_empty() {
                self.stop();
            }
            self.task = task;

            if need_to_start {
                self.start();
            } else {
                // FIXME!: What if the monitor was started ?
                self.change_state(Status::Inactive);
            }
        }
    }

    pub fn stop(&mut self) {
        log::info!("[{}] stopping ...", self.id);
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

    pub fn is_running(&self) -> bool {
        !self.running.is_empty() || !self.stopping.is_empty()
    }

    pub fn has_finished(&self) -> bool {
        self.running.is_empty() && self.stopping.is_empty()
    }

    pub fn cycle(&mut self, sender: &Sender<Inter>) {
        log::debug!(
            "[{}] doing cycle: {} running, {} stopping ",
            self.id,
            self.running.len(),
            self.stopping.len()
        );

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
                self.add_finished_child(e.child, st, e.started_at.elapsed(), e.startup_time);
            } else {
                i += 1;
            }
        }

        Ok(())
    }

    // cycle_stopping check for signaled children if they've stop
    fn cycle_stopping(&mut self) -> Result<(), std::io::Error> {
        let mut killed: Vec<StoppingChild> = Vec::new();
        let mut i = 0;

        while i != self.stopping.len() {
            let chld = &mut self.stopping[i];
            let timeout = chld.timeout;
            let during = chld.stopped_at.elapsed();

            if let Some(st) = chld.try_wait()? {
                let e = self.stopping.remove(i);
                self.add_finished_child(e.child, st, e.started_at.elapsed(), e.startup_time);
            } else if during > timeout {
                chld.kill()?;
                let e = self.stopping.remove(i);
                killed.push(e);
            } else {
                i += 1;
            }
        }

        while !killed.is_empty() {
            let mut chld = killed.remove(0);
            let st = chld.child.wait()?;
            self.add_finished_child(chld.child, st, chld.started_at.elapsed(), chld.startup_time);
        }
        Ok(())
    }

    fn add_finished_child(
        &mut self,
        child: Child,
        status: ExitStatus,
        execution_time: time::Duration,
        startup_time: time::Duration,
    ) {
        log::debug!(
            "[{}] child-{} exited with {} after {}s",
            self.id,
            child.id(),
            status,
            execution_time.as_secs()
        );
        self.finished.push(FinishedChild::new(
            child,
            status,
            execution_time,
            startup_time,
        ))
    }

    fn cycle_finished(&mut self, _sender: &Sender<Inter>) {
        log::debug!(
            "[{}] end of cycle: {} finished",
            self.id,
            self.finished.len()
        );
        while !self.finished.is_empty() {
            let e = self.finished.remove(0);
            let status = self.check_finished_child(&e);
            match status {
                Status::Failed => {
                    if self.state != Status::Stopping {
                        self.change_state(Status::Failing)
                    }
                }
                Status::Finished => {}
                _ => panic!("unexpected status for finished child !"),
            }
            if self.should_process_restarted(status) {
                self.restart_task()
            }
        }
        if self.running.is_empty() {
            self.change_state(finished_state(self.state));
        }
    }

    fn should_process_restarted(&self, status: Status) -> bool {
        (self.retry_count < self.task.retry)
            && ((status == Status::Failed && self.task.restart == Relaunch::OnError)
                || (status == Status::Finished && self.task.restart == Relaunch::Always))
    }

    fn check_finished_child(&self, child: &FinishedChild) -> Status {
        child.status.code().map_or_else(
            || {
                log::debug!("[{}] unexpected exit status {}", self.id, child.status);
                Status::Failed
            },
            |code| {
                if self.unexpected_exit_code(code) {
                    log::warn!(
                        "[{}] child exited with unexpeced status code {}",
                        self.id,
                        code
                    );
                    Status::Failed
                } else if child.execution_time < child.startup_time {
                    log::debug!("[{}] child finished too early", self.id);
                    Status::Failed
                } else {
                    Status::Finished
                }
            },
        )
    }

    fn unexpected_exit_code(&self, code: i32) -> bool {
        !self.task.exitcodes.iter().any(|&wanted| wanted == code)
    }

    fn restart_task(&mut self) {
        let timestamp = get_current_timestamp();

        if self.retry_count < self.task.retry {
            let id = self.increase_spawned_children_counter();
            let mut command = self.task.get_command(id, timestamp);
            self.retry_count += 1;

            let running_child = spawn_child(
                &mut command,
                time::Duration::from_secs(self.task.successdelay.into()),
                self.task.stopsignal,
                time::Duration::from_secs(self.task.stopdelay.into()),
            );
            log::info!("[{}] retry process", self.id);
            self.running.push(running_child);
        } else {
            log::warn!("[{}] max retries limit", self.id);
        }
    }

    fn increase_spawned_children_counter(&mut self) -> u32 {
        let current_value = self.spawned_children;

        self.spawned_children += 1;
        current_value
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

fn running_state(state: Status) -> bool {
    state == Status::Active
        || state == Status::Reloading
        || state == Status::Failing
        || state == Status::Stopping
}

fn finished_state(state: Status) -> Status {
    match state {
        Status::Stopping | Status::Stopped => Status::Stopped,
        Status::Failing | Status::Failed => Status::Failed,
        _ => Status::Finished,
    }
}

fn spawn_child(
    command: &mut Command,
    startup_time: time::Duration,
    stopsignal: Signal,
    stopdelay: time::Duration,
) -> RunningChild {
    let child = command.spawn().expect("Cannot start child");
    RunningChild::new(
        child,
        time::Instant::now(),
        startup_time,
        stopsignal,
        stopdelay,
    )
}

#[cfg(test)]
mod monitor_suite {
    use super::{finished_state, running_state, startable_state, Status};

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

    #[test]
    fn test_running_state() {
        assert_eq!(running_state(Status::Active), true);
        assert_eq!(running_state(Status::Reloading), true);
        assert_eq!(running_state(Status::Failing), true);
        assert_eq!(running_state(Status::Stopping), true);

        assert_eq!(running_state(Status::Finished), false);
        assert_eq!(running_state(Status::Inactive), false);
        assert_eq!(running_state(Status::Failed), false);
        assert_eq!(running_state(Status::Stopped), false);
    }

    #[test]
    fn test_finished_state() {
        assert_eq!(finished_state(Status::Active), Status::Finished);
        assert_eq!(finished_state(Status::Reloading), Status::Finished);
        assert_eq!(finished_state(Status::Finished), Status::Finished);
        assert_eq!(finished_state(Status::Inactive), Status::Finished);

        assert_eq!(finished_state(Status::Failing), Status::Failed);
        assert_eq!(finished_state(Status::Failed), Status::Failed);

        assert_eq!(finished_state(Status::Stopping), Status::Stopped);
        assert_eq!(finished_state(Status::Stopped), Status::Stopped);
    }
}
