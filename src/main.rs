use std::{env, fs};
use std::fs::File;
use std::io::{self, Write};
use serde::{Serialize, Deserialize};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run -- [add|list|done] <task>");
        return;
    }

    let mut tasks = load_tasks();

    let command = &args[1];

    if command == "add" {
        let description = args[2..].join(" ");
        let task = Task::new(description);

        tasks.push(task);
        save_task(&tasks);

        println!("Task added.");
    } else if command == "list" {
        if tasks.is_empty() {
            println!("No tasks found.");
        } else {
            for (i, task) in tasks.iter().enumerate() {
                let status = if task.done {"✅"} else {"❌"};

                println!("{}: [{}] {}", i+1, status, task.description);
            }
        }   
    } else if command == "done" {
        if args.len() < 3 {
            println!("Please provide the task number to mark as done");
            return;
        }

        let index: usize = match args[2].parse::<usize>() {
            Ok(n) => n - 1,
            Err(_) => {
                println!("Invalid task number");
                return;
            }
        };

        if index >= tasks.len() {
            println!("Task number out of range");
            return;
        }

        tasks[index].done = true;
        save_task(&tasks);
        println!("Task {} marked as done ✅", index+1);

    } else if command == "delete" {
        if args.len() < 3 {
            println!("Please provide taskk number to delete");
            return;
        }

        let index: usize = match args[2].parse::<usize>() {
            Ok(n) => n - 1,
            Err(_) => {
                println!("Invalid task number");
                return;
            }
        };

        if index >= tasks.len() {
            println!("Task number out of range");
            return;
        }

        let removed = tasks.remove(index);
        save_task(&tasks);
        println!("Deleted task: {}", removed.description);

    } else if command == "clear" {
        print!("Are you sure you want to delete all tasks? (Y/N): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let input = input.trim().to_lowercase();

        if input == "y" || input == "yes" {
            tasks.clear();
            save_task(&tasks);
            println!("All tasks deleted. Clean state 🌱");
        } else {
            println!("Aborted. Your tasks are safe.🔒");
        }
    } else {
        println!("Unknown Command");
    }
}

fn save_task(tasks: &Vec<Task>) {
    let json = serde_json::to_string(tasks).expect("Failed to serialize tasks");
    let mut file = File::create("tasks.json").expect("Could not create file");
    
    file.write_all(json.as_bytes()).expect("Could not write to file")
}

fn load_tasks() -> Vec<Task> {
    let data = fs::read_to_string("tasks.json").unwrap_or(String::from("[]"));

    serde_json::from_str(&data).unwrap_or_else(|_|Vec::new())
}

#[derive(Serialize, Deserialize)]
struct Task {
    description: String,
    done: bool,
}

impl Task {
    fn new(description: String) -> Task {
        Task { 
            description,
            done: false,
        }
    }
}