use std::collections::HashMap;
use std::convert::TryFrom;

use crate::{reader::ConfigFile,/* worker::Worker*/};
use crate::watcher::Watcher;
use crate::reader::ReadTask;

#[derive(Debug)]
pub struct Config {
    pub tasks: HashMap<String, ReadTask>,
    //pub workers: HashMap<String, Worker>
}

impl Config {
    pub fn new() -> Self {
        Config{
            tasks: HashMap::new(),
     //       workers: HashMap::new()
        }
    }

    pub fn reload(&mut self, watcher: &Watcher) {
        let configfile: ConfigFile = ConfigFile::try_from(watcher).unwrap();
        for task in configfile.task {
            if let Some(t) =  self.tasks.get(&task.name) {
                if t != &task {
         //           if let Some(w) = self.workers.get_mut(&task.name){
        //                w.reload(&task);
          //          }
                    self.tasks.insert(task.name.clone(), task);
                    //Replace in hashmap and relaunch
                }
            } else {
                self.tasks.insert(task.name.clone(), task);
                //Launch and insert in hashmap
            }
        }
    }
}
