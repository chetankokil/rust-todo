use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    description: String,
    completed: bool,
}

impl Task {
    pub fn new(description: String) -> Self {
        Self { description, completed: false }
    }

    pub fn mark_completed(&mut self) {
        self.completed = true;
    }
}
