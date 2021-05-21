use std::collections::HashMap;
use std::convert::TryFrom;
use std::process::ExitStatus;
use std::sync::mpsc::Sender;

use super::{
    communication::Com,
    inter::Inter,
    monitor::Monitor,
    task::{ConfigFile, Task},
    watcher::Watcher,
};

#[derive(Debug)]
pub struct State {
    pub monitors: HashMap<String, Monitor>,
    sender: Sender<Inter>,
    response: Sender<Com>,
}

impl State {
    pub fn new(sender: Sender<Inter>, response: Sender<Com>) -> Self {
        State {
            monitors: HashMap::new(),
            sender,
            response,
        }
    }

    pub fn reload(&mut self, watcher: &Watcher) {
        let configfile: ConfigFile = ConfigFile::try_from(watcher).unwrap();

        for (name, task) in configfile {
            log::debug!("parsed task: {}: {:?}", name, task);

            if self.monitors.get(&name).is_some() {
                self.reload_task(&name, task);
            } else {
                self.add_task(&name, task);
            }
        }
    }

    fn reload_task(&mut self, name: &str, task: Task) {
        let mon = self.monitors.get_mut(name).unwrap();

        if mon.get_task() != &task {
            mon.reload(task);
        }
    }

    fn add_task(&mut self, name: &str, task: Task) {
        self.monitors.insert(
            name.to_string(),
            Monitor::new(name.to_string(), task, self.sender.clone()),
        );
    }

    pub fn start(&mut self, name: &str) {
        log::debug!("starting task {}", name);
        if let Some(mon) = self.monitors.get_mut(name) {
            mon.start();
        } else {
            log::error!("task {} doesn't exist", name);
        }
    }

    // pub fn stop(&mut self, name: &str) {
    //     log::debug!("stopping task {}", name);
    //     if let Some(mon) = self.monitors.get_mut(name) {
    //         mon.stop();
    //     } else {
    //         log::error!("task {} doesn't exist", name);
    //     }
    // }

    pub fn list(&mut self) {
        log::debug!("setting list");
        self.response
            .send(Com::Msg("\nAvailable jobs:\n".to_string()))
            .unwrap();
        for mon in self.monitors.values() {
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
        let status = self.monitors.get(taskname).unwrap().status();
        self.response
            .send(Com::Msg(format!("status of {}: {}", taskname, status)))
            .unwrap();
    }

    pub fn ev_child_has_exited(&mut self, namespace: &String, pid: u32, status: ExitStatus) {
        if let Some(monitor) = self.monitors.get_mut(namespace) {
            monitor.ev_child_has_exited(pid, status);
        } else {
            log::error!("no task named {}", namespace);
        }
    }
}
