use super::inter::Inter;
use nix::{
    self,
    sys::signal::{kill, Signal},
    unistd::Pid,
};
use std::{
    collections::BTreeMap,
    process::{Child, ExitStatus},
    sync::{mpsc::Sender, Arc, Mutex},
    thread::{self, JoinHandle},
    time,
};

#[derive(Debug)]
pub struct WaitChildren {
    namespace: String,
    children: Vec<Child>,
    stopdelay: time::Duration,
    stopsignal: Signal,
}

impl WaitChildren {
    pub fn new(
        namespace: String,
        children: Vec<Child>,
        stopdelay: time::Duration,
        stopsignal: Signal,
    ) -> Self {
        Self {
            namespace,
            children,
            stopdelay,
            stopsignal,
        }
    }
}

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
pub struct FinishedChild {
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
struct ManageChildren {
    running: Vec<RunningChild>,
    stopping: Vec<StoppingChild>,
    finished: Vec<FinishedChild>,
}

impl ManageChildren {
    fn new(children: Vec<RunningChild>) -> ManageChildren {
        ManageChildren {
            running: children,
            stopping: Vec::new(),
            finished: Vec::new(),
        }
    }

    fn has_finished(&self) -> bool {
        self.running.is_empty() && self.stopping.is_empty()
    }

    fn stop(&mut self) {
        while !self.running.is_empty() {
            let chld = self.running.remove(0);
            let stopping_child = chld.stop().unwrap();
            self.stopping.push(stopping_child);
        }
        self.running.clear();
    }

    fn cycle(&mut self, sender: &Sender<Inter>) {
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
}

pub struct Waiter {
    thread: Option<JoinHandle<()>>,
    sender: Sender<Inter>,
    process_manager: Arc<Mutex<BTreeMap<String, ManageChildren>>>,
}

impl Waiter {
    pub fn new(sender: Sender<Inter>) -> Self {
        Self {
            thread: None,
            sender,
            process_manager: Arc::new(Mutex::new(BTreeMap::new())),
        }
    }

    pub fn wait_children(&mut self, children_to_wait: WaitChildren) {
        let namespace = children_to_wait.namespace.clone();
        let mut process_manager = self.process_manager.lock().unwrap();

        if let Some(manager) = process_manager.get_mut(&namespace) {
            let mut running_process = children_to_wait.into();
            manager.running.append(&mut running_process);
        } else {
            let manager = ManageChildren::new(children_to_wait.into());
            process_manager.insert(namespace.clone(), manager);
            if process_manager.len() == 1 {
                drop(process_manager);
                self.spawn_waiting_thread()
            }
        }
    }

    pub fn stop(&mut self, namespace: &str) {
        let mut process_manager = self.process_manager.lock().unwrap();

        if let Some(manager) = process_manager.get_mut(namespace) {
            manager.stop()
        } else {
            log::warn!("unknown namespace {}", namespace);
        }
    }

    fn spawn_waiting_thread(&mut self) {
        let process_manager_mut = self.process_manager.clone();
        let sender = self.sender.clone();

        self.thread = Some(thread::spawn(move || {
            log::debug!("waiter thread spawned !");
            loop {
                let mut process_manager = process_manager_mut.lock().unwrap();
                let mut finished_manager: Vec<String> = Vec::new();

                for (key, manager) in process_manager.iter_mut() {
                    manager.cycle(&sender);
                    if manager.has_finished() {
                        finished_manager.push(key.clone());
                    }
                }
                for key in finished_manager {
                    process_manager.remove(&key);
                }
                if process_manager.len() == 0 {
                    break;
                }
                drop(process_manager);
                thread::sleep(time::Duration::from_millis(500));
            }
            sender.send(Inter::NoMoreChildrenToWait).unwrap();
            log::debug!("waiter thread finished !");
        }));
    }

    pub fn done_wait_children(&mut self) {
        if let Some(thread) = self.thread.take() {
            thread.join().expect("cannot join waiting thread");
        } else {
            log::error!("waiter: no thread to join as being asked !");
        }
    }
}
