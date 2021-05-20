use super::inter::Inter;
use nix::{
    self,
    sys::{
        signal::{kill, Signal},
        wait::{waitpid, WaitStatus},
    },
    unistd::Pid,
};
use std::{
    collections::BTreeMap,
    convert::TryFrom,
    process::{Child, ExitStatus},
    sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc::Sender,
        Arc,
    },
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
    namespace: String,

    running: Vec<RunningChild>,
    stopping: Vec<StoppingChild>,
    finished: Vec<FinishedChild>,
}

impl ManageChildren {
    fn new(namespace: String) -> ManageChildren {
        ManageChildren {
            namespace,
            running: Vec::new(),
            stopping: Vec::new(),
            finished: Vec::new(),
        }
    }

    fn new_with_running_children(namespace: String, children: Vec<RunningChild>) -> ManageChildren {
        ManageChildren {
            namespace,
            running: children,
            stopping: Vec::new(),
            finished: Vec::new(),
        }
    }

    fn cycle(&mut self, sender: &Sender<Inter>) {
        self.cycle_running().unwrap();
        self.cycle_stopping().unwrap();
        self.cycle_finished(sender);
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

        while killed.len() > 0 {
            let chld = killed.remove(0);
            let st = chld.1.wait()?;
            self.finished.push(FinishedChild::new(chld.0, chld.1, st));
        }
        Ok(())
    }

    fn cycle_finished(&mut self, sender: &Sender<Inter>) {
        while self.finished.len() > 0 {
            let e = self.finished.remove(0);
            sender.send(e.into()).unwrap();
        }
    }
}

pub struct Waiter {
    counter: Arc<AtomicUsize>,
    thread: Option<JoinHandle<()>>,
    sender: Sender<Inter>,
}

impl Waiter {
    pub fn new(sender: Sender<Inter>) -> Self {
        Self {
            counter: Arc::new(AtomicUsize::new(0)),
            thread: None,
            sender,
        }
    }

    pub fn wait_children(&mut self, children_to_wait: usize) {
        self.counter.fetch_add(children_to_wait, Ordering::SeqCst);

        if self.thread.is_none() {
            self.spawn_waiting_thread();
        } else {
            log::debug!("waiting thread already running");
        }
    }

    fn spawn_waiting_thread(&mut self) {
        let sender = self.sender.clone();
        let counter = self.counter.clone();

        self.thread = Some(thread::spawn(move || {
            log::debug!("thread to wait subprocess spawned !");
            while counter.load(Ordering::SeqCst) > 0 {
                match waitpid(Pid::from_raw(-1), None) {
                    Ok(status) => match status {
                        WaitStatus::Exited(pid, _) | WaitStatus::Signaled(pid, _, _) => {
                            log::debug!("a process has exited {:?}", status);
                            sender.send(Inter::ChildHasExited(pid, status)).unwrap();
                            counter.fetch_sub(1, Ordering::SeqCst);
                        }
                        _ => {
                            log::warn!("exit status {:?} not handled", status);
                        }
                    },
                    Err(error) => {
                        log::error!("error while using waitpid: {}", error);
                        panic!("error while waiting for subproccess");
                    }
                }
            }
            sender.send(Inter::NoMoreChildrenToWait).unwrap();
            log::debug!("wait counter at zero, finished waiting for subprocess");
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
