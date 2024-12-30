mod test;
mod todo;
mod storage;

use todo::Task;
use storage::{save_tasks, view_tasks};


fn main() {
    let mut task = Task::new("Bring milk".to_string());
    let task2 = Task::new("Bring chocolate".to_string());

    Task::mark_completed(&mut task);

    let mut tasks: Vec<Task> = Vec::new();

    save_tasks(&mut tasks, task);
    save_tasks(&mut tasks, task2);
    view_tasks(tasks);
   }
