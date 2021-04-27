use std::collections::VecDeque;

pub struct History {
    queue: VecDeque<String>,
    pos: usize,
}

pub enum Direction {
    Next,
    Previous,
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

    pub fn get(&mut self, direction: &Direction) -> Option<String> {
        match direction {
            Direction::Next => self.pos -= 1,
            Direction::Previous => self.pos += 1,
        };
        if let Some(line) = self.queue.get(self.pos) {
            return Some(line.clone());
        }
        None
    }

    pub fn print(&self) {
        self.queue.iter().for_each(|line| println!("{}", line));
    }
}
