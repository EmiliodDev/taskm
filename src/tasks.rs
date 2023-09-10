#[derive(Debug, PartialEq)]
pub struct Task {
    id: u16,
    title: String,
    description: String,
    status: bool,
}

impl Task { // create a new task
   pub fn new(id: u16, title: String, description: String, status: bool) -> Task {
        Task {
            id,
            title,
            description,
            status,
        }
    }
}

pub struct TaskManager {
    pub tasks: Vec<Task>,
}

impl TaskManager {
    pub fn add(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn check(&mut self, index: usize, new_status: bool) {
        if !self.tasks.is_empty() {
            self.tasks[index].status = new_status;
        }
    }

    pub fn list(&self) {
        if !self.tasks.is_empty() {
            for task in &self.tasks {
                println!("{:?}", task);
            }
        } else {
            println!("Task list empty.");
        }
    }
    
    pub fn delete(&mut self, index: usize) {
        if !self.tasks.is_empty() {
            self.tasks.remove(index);
        } else {
            println!("Task list is already empty.");
        }
    }
}
