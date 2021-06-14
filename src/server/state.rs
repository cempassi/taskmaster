use std::{
    collections::HashMap,
    convert::TryFrom,
    marker,
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
    formatter::Formatter,
    inter::Inter,
    monitor::Monitor,
    task::{ConfigFile, Task},
    watcher::Watcher,
};

#[derive(Debug)]
pub struct State<F>
where
    F: Formatter,
{
    pub monitors: Arc<Mutex<HashMap<String, Monitor>>>,
    sender: Sender<Inter>,
    response: Sender<Com>,
    thread: Option<JoinHandle<()>>,
    waiter_running: Arc<AtomicBool>,
    _marker: marker::PhantomData<F>,
}

impl<F: Formatter> State<F> {
    pub fn new(sender: Sender<Inter>, response: Sender<Com>) -> Self {
        Self {
            monitors: Arc::new(Mutex::new(HashMap::new())),
            sender,
            response,
            thread: None,
            waiter_running: Arc::new(AtomicBool::new(false)),
            _marker: marker::PhantomData,
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
        let mon = Monitor::new(name.to_string(), task);
        if mon.is_running() {
            self.start_waiting_thread_if_needed();
        }
        self.monitors.lock().unwrap().insert(name.to_string(), mon);
    }

    fn unknown_taskid(&self, taskid: &str) {
        log::error!("task {} doesn't exist", taskid);
        F::send_error(&self.response, format!("unknown taskid {}", taskid)).unwrap();
    }

    pub fn start(&mut self, name: &str) {
        log::debug!("starting task {}", name);
        if let Some(mon) = self.monitors.lock().unwrap().get_mut(name) {
            mon.start();
        } else {
            self.unknown_taskid(name);
        }
        self.start_waiting_thread_if_needed();
    }

    fn start_waiting_thread_if_needed(&mut self) {
        if !self.waiter_running.load(Ordering::SeqCst) {
            self.spawn_waiting_thread();
        }
    }

    pub fn info(&mut self, name: &str) {
        log::debug!("Get info on task {}", name);
        if let Some(mon) = self.monitors.lock().unwrap().get_mut(name) {
            F::send_task(&self.response, name, &mon.get_task()).unwrap();
        } else {
            self.unknown_taskid(name);
        }
    }

    pub fn list(&mut self) {
        log::debug!("setting list");
        F::send_tasks(
            &self.response,
            &mut self
                .monitors
                .lock()
                .unwrap()
                .iter()
                .map(|(k, v)| (k.clone(), v.get_task().clone())),
        )
        .unwrap();
    }

    pub fn status(&self, taskname: &str) {
        log::debug!("retrieving status of {}", taskname);
        if let Some(manager) = self.monitors.lock().unwrap().get(taskname) {
            let status = manager.status();

            F::send_status(&self.response, taskname, status).unwrap();
        } else {
            self.unknown_taskid(taskname);
        }
    }

    pub fn stop(&mut self, taskid: &str) {
        let mut process_manager = self.monitors.lock().unwrap();

        if let Some(manager) = process_manager.get_mut(taskid) {
            manager.stop();
        } else {
            self.unknown_taskid(taskid);
        }
    }

    pub fn restart(&mut self, taskid: &str) {
        let mut process_manager = self.monitors.lock().unwrap();

        if let Some(manager) = process_manager.get_mut(taskid) {
            manager.restart()
        } else {
            self.unknown_taskid(taskid);
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
                let working_manager = process_manager
                    .values_mut()
                    .filter(|manager| manager.is_running());
                let mut working_manager_count = 0;
                let mut finished_manager_count = 0;

                working_manager.for_each(|manager| {
                    working_manager_count += 1;
                    manager.cycle(&sender);
                    if manager.has_finished() {
                        finished_manager_count += 1;
                    }
                });
                if working_manager_count == finished_manager_count {
                    break;
                }
                drop(process_manager);
                thread::sleep(time::Duration::from_millis(500));
            }
            log::debug!("waiter thread finished !");
            running_state.store(false, Ordering::SeqCst);
        }));
    }
}
