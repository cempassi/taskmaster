use std::collections::HashMap;
use std::convert::TryFrom;
use std::sync::mpsc::{channel, Receiver, Sender};

use super::worker;

use super::{
    task::{ConfigFile, Task},
    watcher::Watcher,
    worker::{Action, Status},
    Message,
};

#[derive(Debug)]
pub struct State {
    pub tasks: HashMap<String, Task>,
    pub workers: HashMap<String, (Sender<Action>, Receiver<Status>)>,
}

impl State {
    pub fn new() -> Self {
        State {
            tasks: HashMap::new(),
            workers: HashMap::new(),
        }
    }

    pub fn reload(&mut self, watcher: &Watcher) {
        let configfile: ConfigFile = ConfigFile::try_from(watcher).unwrap();

        for (name, task) in configfile {
            log::debug!("parsed task: {}: {:?}", name, task);

            //Si la tache existe deja
            if let Some(t) = self.tasks.get(&name) {
                //Si la tache a ete modifiee
                if t != &task {
                    log::debug!("task {} as been changed", name.clone());
                    //Si la tache est deja en cours de lancement, la relancer, sinon
                    //simplement changer la configuration

                    if let Some(w) = self.workers.get(&name) {
                        log::debug!("asking to reload running process for {}", name.clone());
                        w.0.send(Action::Reload(task.clone())).unwrap();
                    }
                    self.tasks.insert(name.clone(), task);
                    //Replace in hashmap and relaunch
                }
            } else {
                if task.autostart {
                    log::debug!("asking to start {}", name.clone());
                    watcher.send(Message::Start(name.clone()))
                }
                self.tasks.insert(name.clone(), task);
            }
        }
    }

    pub fn start(&mut self, name: &str) {
        log::debug!("starting task {}", name);
        if let Some(t) = self.tasks.get(name) {
            let (send_action, receive_action) = channel::<Action>();
            let (send_status, receive_status) = channel::<Status>();
            self.workers
                .insert(name.to_string(), (send_action, receive_status));
            worker::run(name, t.clone(), send_status, receive_action);
        } else {
            log::debug!("Task '{}' not found", name);
        }
    }

    pub fn stop(&mut self, name: &str) {
        log::debug!("stopping task {}", name);
        if let Some(worker) = self.workers.get(name) {
            worker.0.send(Action::Stop).unwrap();
        } else {
            log::warn!("task {} is not running", name);
        }
    }

    pub fn list(&mut self, chan: &Sender<String>) {
        log::debug!("setting list");
        chan.send("\nAvailable jobs:\n".to_string()).unwrap();
        for task in self.tasks.values() {
            chan.send(format!("{}", task)).unwrap();
            chan.send("\n----------\n".to_string()).unwrap();
        }
    }

    pub fn status(&self, taskname: &str, response: &Sender<String>) {
        log::debug!("retrieving status of {}", taskname);
        let status = self
            .workers
            .get(taskname)
            .map_or(Status::NotStarted, |worker| {
                worker.0.send(Action::Status);
                worker.1.recv().unwrap_or(Status::Unknown)
            });
        response
            .send(format!("status of {}: {}", taskname, status))
            .unwrap();
    }
}
