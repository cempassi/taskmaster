use serde::Deserialize;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs;
use std::io::prelude::*;
use std::os::unix::net::{UnixListener, UnixStream};
use std::sync::mpsc::{channel, Sender};
use std::thread;

use super::worker;

use super::{
    reader::{ConfigFile, ReadTask},
    task::Task,
    watcher::Watcher,
    worker::Action,
    Communication,
    Message,
};

#[derive(Debug)]
pub struct State {
    pub listener: UnixListener,
    pub tasks: HashMap<String, ReadTask>,
    pub workers: HashMap<String, Sender<Action>>,
}

impl Drop for State {
    fn drop(&mut self) {
        fs::remove_file("/tmp/taskmaster.sock").unwrap();
    }
}

impl State {
    pub fn new(socket: &str) -> Self {
        State {
            listener: UnixListener::bind(socket).unwrap(),
            tasks: HashMap::new(),
            workers: HashMap::new(),
        }
    }

    pub fn listen(&mut self, sender: Sender<Communication>) {
        let listener = self.listener.try_clone().unwrap();
        thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        let s = sender.clone();
                        thread::spawn(move || process_message(stream, s));
                    }
                    Err(err) => {
                        println!("Error: {}", err);
                        break;
                    }
                }
            }
        });
    }

    pub fn reload(&mut self, watcher: &Watcher) {
        let configfile: ConfigFile = ConfigFile::try_from(watcher).unwrap();

        for task in configfile.task {
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
                if task.autostart == true {
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
        worker::run(task, sender.clone(), receiver);
    }

    pub fn stop(&mut self, name: &str) {
        if let Some(worker) = self.workers.get(name) {
            worker.send(Action::Stop).unwrap();
        } else {
            println!("Task is not running");
        }
    }

    pub fn list(&mut self, chan: Sender<String>) {
        chan.send("\nAvailable jobs:\n".to_string()).unwrap();
        for (_, task) in &self.tasks {
            chan.send(format!("{}", task)).unwrap();
            chan.send("\n----------\n".to_string()).unwrap();
        }
    }
}

fn process_message(stream: UnixStream, sender: Sender<Communication>) {
    println!("Ready to recieve.");
    let mut response = stream.try_clone().expect("Couldn't clone socket");
    let mut de = serde_json::Deserializer::from_reader(stream);


    if let Ok(msg) = Message::deserialize(&mut de) {
        println!("Recieved {:?}", msg);
        let (snd, receiver) = channel();
        let com = Communication{ message: msg, channel: Some(snd)};
        sender.send(com).unwrap();
        for res in receiver.iter() {
            response.write_all(res.as_bytes()).unwrap();
        }
    }
    println!("End of transmission.");
}
