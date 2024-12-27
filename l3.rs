use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

#[derive(Debug)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

impl Task {
    fn new(id: usize, description: String) -> Self {
        Self {
            id,
            description,
            completed: false,
        }
    }

    fn mark_completed(&mut self) {
        self.completed = true;
    }
}

struct TodoList {
    tasks: Vec<Task>,
    next_id: usize,
}

impl TodoList {
    fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, description: String) {
        let task = Task::new(self.next_id, description);
        self.tasks.push(task);
        self.next_id += 1;
    }

    fn remove_task(&mut self, id: usize) {
        self.tasks.retain(|task| task.id != id);
    }

    fn edit_task(&mut self, id: usize, new_description: String) {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {
            task.description = new_description;
        }
    }

    fn mark_task_completed(&mut self, id: usize) {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {
            task.mark_completed();
        }
    }

    fn save_to_file(&self, file_path: &str) -> io::Result<()> {
        let mut file = File::create(file_path)?;
        for task in &self.tasks {
            writeln!(
                file,
                "{}|{}|{}",
                task.id, task.description, task.completed
            )?;
        }
        Ok(())
    }

    fn load_from_file(&mut self, file_path: &str) -> io::Result<()> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        self.tasks.clear();
        self.next_id = 1;

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() == 3 {
                if let (Ok(id), Some(description), Ok(completed)) = (
                    parts[0].parse(),
                    Some(parts[1].to_string()),
                    parts[2].parse(),
                ) {
                    let task = Task {
                        id,
                        description,
                        completed,
                    };
                    self.tasks.push(task);
                    self.next_id = self.next_id.max(id + 1);
                }
            }
        }

        Ok(())
    }

    fn list_tasks(&self) {
        for task in &self.tasks {
            println!(
                "[{}] {} - {}",
                if task.completed { "x" } else { " " },
                task.id,
                task.description
            );
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new();

    loop {
        println!("\nTodo List Menu:");
        println!("1. Add task");
        println!("2. Remove task");
        println!("3. Edit task");
        println!("4. Mark task as completed");
        println!("5. List tasks");
        println!("6. Save tasks to file");
        println!("7. Load tasks from file");
        println!("8. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        match choice.trim().parse::<u8>() {
            Ok(1) => {
                println!("Enter task description:");
                let mut description = String::new();
                io::stdin().read_line(&mut description).unwrap();
                todo_list.add_task(description.trim().to_string());
            }
            Ok(2) => {
                println!("Enter task ID to remove:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    todo_list.remove_task(id);
                }
            }
            Ok(3) => {
                println!("Enter task ID to edit:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                println!("Enter new description:");
                let mut description = String::new();
                io::stdin().read_line(&mut description).unwrap();
                if let Ok(id) = id.trim().parse() {
                    todo_list.edit_task(id, description.trim().to_string());
                }
            }
            Ok(4) => {
                println!("Enter task ID to mark as completed:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    todo_list.mark_task_completed(id);
                }
            }
            Ok(5) => {
                todo_list.list_tasks();
            }
            Ok(6) => {
                println!("Enter file path to save tasks:");
                let mut file_path = String::new();
                io::stdin().read_line(&mut file_path).unwrap();
                if let Err(e) = todo_list.save_to_file(file_path.trim()) {
                    println!("Failed to save tasks: {}", e);
                }
            }
            Ok(7) => {
                println!("Enter file path to load tasks:");
                let mut file_path = String::new();
                io::stdin().read_line(&mut file_path).unwrap();
                if let Err(e) = todo_list.load_from_file(file_path.trim()) {
                    println!("Failed to load tasks: {}", e);
                }
            }
            Ok(8) => {
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}