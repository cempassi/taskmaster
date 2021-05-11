use std::collections::HashMap;
use std::convert::TryFrom;
use std::sync::mpsc::Sender;

use super::{
    monitor::Monitor,
    task::{ConfigFile, Task},
    watcher::Watcher,
};

#[derive(Debug)]
pub struct State {
    pub tasks: HashMap<String, Task>,
    pub monitors: HashMap<String, Monitor>,
}

impl State {
    pub fn new() -> Self {
        State {
            tasks: HashMap::new(),
            monitors: HashMap::new(),
        }
    }

    pub fn reload(&mut self, watcher: &Watcher) {
        let configfile: ConfigFile = ConfigFile::try_from(watcher).unwrap();

        for (name, task) in configfile {
            log::debug!("parsed task: {}: {:?}", name, task);

            if self.tasks.get(&name).is_some() {
                self.reload_task(&name, &task);
            } else {
                self.add_task(&name, &task);
            }
        }
    }

    fn reload_task(&mut self, name: &str, task: &ReadTask) {
        let current_task = self.tasks.get(name).unwrap();
        if current_task != task {
            self.tasks.insert(name.to_string(), task.clone());
            let mon = self.monitors.get_mut(name).unwrap();
            mon.reload(Task::try_from(task).unwrap());
        }
    }

    fn add_task(&mut self, name: &str, task: &ReadTask) {
        self.tasks.insert(name.to_string(), task.clone());
        self.monitors.insert(
            name.to_string(),
            Monitor::new(Task::try_from(task).unwrap()),
        );
    }

    pub fn start(&mut self, name: &str) {
        log::debug!("starting task {}", name);
        // FIXME: we don't need to start a monitor that is already started
        if let Some(mon) = self.monitors.get_mut(name) {
            mon.start();
        } else {
            log::error!("task {} doesn't exist", name);
        }
        unimplemented!();
    }

    pub fn stop(&mut self, name: &str) {
        log::debug!("stopping task {}", name);
        // FIXME: we can't stop a monitors that is not running
        if let Some(mon) = self.monitors.get_mut(name) {
            mon.stop();
        } else {
            log::error!("task {} doesn't exist", name);
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
        let status = self.monitors.get(taskname).unwrap().status();
        response
            .send(format!("status of {}: {}", taskname, status))
            .unwrap();
    }
}
