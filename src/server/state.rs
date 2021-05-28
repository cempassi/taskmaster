use std::{
    collections::HashMap,
    convert::TryFrom,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::Sender,
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time,
};

use super::{
    communication::Com,
    inter::Inter,
    monitor::Monitor,
    task::{ConfigFile, Task},
    watcher::Watcher,
};

#[derive(Debug)]
pub struct State {
    pub monitors: Arc<Mutex<HashMap<String, Monitor>>>,
    sender: Sender<Inter>,
    response: Sender<Com>,
    thread: Option<JoinHandle<()>>,
    waiter_running: Arc<AtomicBool>,
}

impl State {
    pub fn new(sender: Sender<Inter>, response: Sender<Com>) -> Self {
        State {
            monitors: Arc::new(Mutex::new(HashMap::new())),
            sender,
            response,
            thread: None,
            waiter_running: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn reload(&mut self, watcher: &Watcher) {
        let configfile: ConfigFile = ConfigFile::try_from(watcher).unwrap();

        for (name, task) in configfile {
            log::debug!("parsed task: {}: {:?}", name, task);

            if self.monitors.lock().unwrap().get(&name).is_some() {
                self.reload_task(&name, task);
            } else {
                self.add_task(&name, task);
            }
        }
    }

    fn reload_task(&mut self, name: &str, task: Task) {
        let mut monitors = self.monitors.lock().unwrap();
        let mon = monitors.get_mut(name).unwrap();

        if mon.get_task() != &task {
            mon.reload(task);
        }
    }

    fn add_task(&mut self, name: &str, task: Task) {
        self.monitors.lock().unwrap().insert(
            name.to_string(),
            Monitor::new(name.to_string(), task, self.sender.clone()),
        );
    }

    pub fn start(&mut self, name: &str) {
        log::debug!("starting task {}", name);
        if let Some(mon) = self.monitors.lock().unwrap().get_mut(name) {
            mon.start();
        } else {
            log::error!("task {} doesn't exist", name);
            return;
        }
        if !self.waiter_running.load(Ordering::SeqCst) {
            self.spawn_waiting_thread();
        }
    }

    pub fn list(&mut self) {
        log::debug!("setting list");
        self.response
            .send(Com::Msg("\nAvailable jobs:\n".to_string()))
            .unwrap();
        for mon in self.monitors.lock().unwrap().values() {
            self.response
                .send(Com::Msg(format!("{}", mon.get_task())))
                .unwrap();
            self.response
                .send(Com::Msg("\n----------\n".to_string()))
                .unwrap();
        }
    }

    pub fn status(&self, taskname: &str) {
        log::debug!("retrieving status of {}", taskname);
        let status = self
            .monitors
            .lock()
            .unwrap()
            .get(taskname)
            .unwrap()
            .status();
        self.response
            .send(Com::Msg(format!("status of {}: {}", taskname, status)))
            .unwrap();
    }

    pub fn stop(&mut self, namespace: &str) {
        let mut process_manager = self.monitors.lock().unwrap();

        if let Some(manager) = process_manager.get_mut(namespace) {
            manager.stop()
        } else {
            log::warn!("unknown namespace {}", namespace);
        }
    }

    fn spawn_waiting_thread(&mut self) {
        let process_manager_mut = self.monitors.clone();
        let running_state = self.waiter_running.clone();
        let sender = self.sender.clone();

        running_state.store(true, Ordering::SeqCst);
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
            log::debug!("waiter thread finished !");
            running_state.store(false, Ordering::SeqCst);
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
