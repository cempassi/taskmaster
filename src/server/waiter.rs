use super::{
    inter::Inter,
    manager::{ManageChildren, WaitChildren},
};
use std::{
    collections::BTreeMap,
    sync::{mpsc::Sender, Arc, Mutex},
    thread::{self, JoinHandle},
    time,
};

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
            manager.add_children_to_wait(children_to_wait);
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
