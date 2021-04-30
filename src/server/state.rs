use std::collections::HashMap;
use std::convert::TryFrom;
use std::sync::mpsc::{channel, Sender};

use super::worker;

use super::{
    default,
    reader::{ConfigFile, ReadTask},
    task::Task,
    watcher::Watcher,
    worker::Action,
    Message,
};

#[derive(Debug)]
pub struct State {
    pub tasks: HashMap<String, ReadTask>,
    pub workers: HashMap<String, Sender<Action>>,
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

        for task in configfile.task {
            println!("parsed task: {}", task);

            //Si la tache existe deja
            if let Some(t) = self.tasks.get(&task.name) {
                //Si la tache a ete modifiee
                if t != &task {
                    //Si la tache est deja en cours de lancement, la relancer, sinon
                    //simplement changer la configuration
                    if let Some(w) = self.workers.get(&task.name) {
                        w.send(Action::Reload(task.clone())).unwrap();
                    }
                    self.tasks.insert(task.name.clone(), task);
                    //Replace in hashmap and relaunch
                }
            } else {
                if task.autostart.unwrap_or(default::AUTOSTART) {
                    watcher.send(Message::Start(task.name.clone()))
                }
                self.tasks.insert(task.name.clone(), task);
            }
        }
    }

    pub fn start(&mut self, name: &str) {
        let task = Task::try_from(self.tasks.get(name).unwrap()).unwrap();
        let (sender, receiver) = channel::<Action>();
        self.workers.insert(name.to_string(), sender.clone());
        worker::run(task, sender, receiver);
    }

    pub fn stop(&mut self, name: &str) {
        if let Some(worker) = self.workers.get(name) {
            worker.send(Action::Stop).unwrap();
        } else {
            println!("Task is not running");
        }
    }

    pub fn list(&mut self, chan: &Sender<String>) {
        chan.send("\nAvailable jobs:\n".to_string()).unwrap();
        for task in self.tasks.values() {
            chan.send(format!("{}", task)).unwrap();
            chan.send("\n----------\n".to_string()).unwrap();
        }
    }
}
