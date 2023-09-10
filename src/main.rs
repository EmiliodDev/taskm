pub mod tasks;

use tasks::TaskManager;
use tasks::Task;

fn main() {
    println!("Todo list");

    let mut task_manager = TaskManager {
        tasks: vec![],
    };

    let task1 = Task::new(1, "Task 1".to_string(), "This is the task 1".to_string(), false);

    task_manager.add(task1);

    task_manager.check(0, true);

    task_manager.list();  // print the all the tasks
                        
    task_manager.delete(0);

    task_manager.list();
}
