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
        println!("Press 1 to add a task");


        print!("Your choice:");
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

            _ => {
                
            }
        }
    }

}