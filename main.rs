enum TaskState {
    Running,
    Stopped,
    Failed(String),
}

struct Task {
    id: u32,
    state: TaskState,
}

impl Task {
    fn restart(&mut self) {
        match self.state {
            TaskState::Failed(_) => {
                self.state = TaskState::Running;
                println!("Task {} started", &self.id);
            }
            TaskState::Stopped => {
                self.state = TaskState::Running;
                println!("Task {} started", &self.id);
            }
            _ => {
                println!("Task {} is already running", &self.id)
            }
        }
    }
}

fn print_state(state: &TaskState) {
    let text = match state {
        TaskState::Running => String::from("The task is running"),
        TaskState::Stopped => String::from("The task is not running"),
        TaskState::Failed(s) => format!("The task failed to start due to {}", s),
    };
    println!("{}", text);
}

fn is_running(state: &TaskState) -> bool {
    let result: bool;
    match state {
        TaskState::Running => result = true,
        _ => result = false,
    }
    return result;
}

fn main() {
    let running_task = TaskState::Running;
    let stopped_task = TaskState::Stopped;
    let failed_task = TaskState::Failed(String::from("A specific reason"));

    print_state(&running_task);
    print_state(&stopped_task);
    print_state(&failed_task);

    println!("Is running? : {}", is_running(&running_task));
    println!("Is running? : {}", is_running(&stopped_task));
    println!("Is running? : {}", is_running(&failed_task));

    let mut task = Task {
        id: 1,
        state: failed_task,
    };

    task.restart();
}

