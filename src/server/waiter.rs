use super::inter::Inter;
use nix::{
    sys::{
        signal::Signal,
        wait::{waitpid, WaitStatus},
    },
    unistd::Pid,
};
use odds::vec::VecExt;
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

struct WaitChildren {
    namespace: String,
    children: Vec<Child>,
    stopdelay: time::Duration,
    stopsignal: Signal,
}

struct RunningChild {
    child: Child,

    stopdelay: time::Duration,
    stopsignal: Signal,
}

impl From<WaitChildren> for Vec<RunningChild> {
    fn from(data: WaitChildren) -> Vec<RunningChild> {
        let mut res: Vec<RunningChild> = Vec::new();

        for child in data.children {
            res.push(RunningChild {
                child,
                stopdelay: data.stopdelay,
                stopsignal: data.stopsignal,
            })
        }

        res
    }
}

struct StoppingChild {
    child: Child,
    signaled_at: time::Instant,
    timeout: time::Duration,
}

impl StoppingChild {
    fn new(child: Child, instant: time::Instant, timeout: time::Duration) -> StoppingChild {
        StoppingChild {
            child,
            signaled_at: instant,
            timeout,
        }
    }
}

struct FinishedChild {
    child: Child,
    status: ExitStatus,
}

impl FinishedChild {
    fn new(child: Child, status: ExitStatus) -> FinishedChild {
        FinishedChild { child, status }
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

    fn cycle(&mut self) {
        self.cycle_running().unwrap();
    }

    // cycle_running check for Child that has terminated
    fn cycle_running(&mut self) -> Result<(), std::io::Error> {
        let mut i = 0;

        while i != self.running.len() {
            if let Some(st) = self.running[i].child.try_wait()? {
                let e = self.running.remove(i);
                self.finished.push(FinishedChild::new(e.child, st));
            } else {
                i += 1;
            }
        }

        Ok(())
    }

    fn cycle_stopping(&mut self) -> Result<(), std::io::Error> {
        let mut killed: Vec<Child> = Vec::new();
        let mut i = 0;

        while i != self.stopping.len() {
            let chld = &mut self.stopping[i];
            let timeout = chld.timeout;
            let during = chld.signaled_at.elapsed();

            if let Some(st) = chld.child.try_wait()? {
                let e = self.stopping.remove(i);
                self.finished.push(FinishedChild::new(e.child, st));
            } else if during > timeout {
                chld.child.kill()?;
                let e = self.stopping.remove(i).child;
                killed.push(e);
            } else {
                i += 1;
            }
        }
        Ok(())
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
