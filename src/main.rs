mod task;

use std::io::{self, Write};

use task::Task;



fn main() {
    
    // Database 
    let mut tasks : Vec<Task> = Vec::new();
    let mut next_id = 1;

    // Take user input
    loop {
        println!("\nToDo App");

        print!("Your choice (add/print/delete/completed):");
        io::stdout().flush().unwrap();

        // Take the choice
        let mut choice : String = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        // Switch case
        match choice.trim() {
            "add" => {
                print!("Enter task description:");
                io::stdout().flush().unwrap();

                let mut description : String = String::new();
                io::stdin().read_line(&mut description).unwrap();

                let new_task : Task = Task::new(next_id, description.trim().to_string());
                tasks.push(new_task);

                next_id += 1;
                println!("Task added to the DB! Assigned Task ID : {}", next_id - 1);
            }

            "print" => {
                if tasks.is_empty() {
                    println!("No tasks found!")
                } else {
                    println!(
                        "{:<5} | {:<30} | {:<10}",
                        "ID", "Description", "Completed"
                    );
                    println!("{}", "-".repeat(80));
                    
                    // Print each task
                    for task in &tasks {
                        println!(
                                "{:<5} | {:<30} | {:<10}",
                                    task.id,
                                    task.description,
                                    if task.completed { "Yes" } else { "No" });
                    }
                }
            }

            "completed" => {
                print!("Enter task ID:");
                io::stdout().flush().unwrap();

                let mut id : String = String::new();
                io::stdin().read_line(&mut id).unwrap();

                // Parse to integer 
                if let Ok(id) = id.trim().parse::<usize>() {
                    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
                        task.mark_complete();
                        println!("Task completed!");
                    } else {
                        println!("Task not found!");
                    }
                } else {
                    println!("Invalid ID type!");
                }
            }

            

            "exit" => {
                break;
            }

            _ => {
                
            }
        }
    }

    println!("App closed...") 

}