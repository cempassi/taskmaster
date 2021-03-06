use std::collections::VecDeque;

pub struct History {
    queue: VecDeque<String>,
    pos: usize,
}

impl History {
    pub fn new() -> Self {
        History {
            queue: VecDeque::new(),
            pos: 0,
        }
    }

    pub fn push(&mut self, command: String) {
        self.queue.push_back(command);
    }

    pub fn get(&mut self, direction: i8) {
        if direction > 0 {
            self.pos += direction as usize;
        } else {
            self.pos -= (direction * -1) as usize;
        }
        self.queue.get(self.pos);
    }

    pub fn print(&self) { 
        self.queue.iter().for_each(|line| println!("{}", line));
    }
}
