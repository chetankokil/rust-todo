use crate::todo::Task;

pub fn save_tasks(tasks: &mut Vec<Task>, task: Task) {
    tasks.push(task);
}

pub fn view_tasks(tasks: Vec<Task>) {
    for task in tasks {
        println!("{:?}", &task);
    }
}
