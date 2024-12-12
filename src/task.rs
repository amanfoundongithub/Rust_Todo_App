
/**
 * Define a task entity by a 
 * struct based method because
 * Rust does not support classes 
 */
pub struct Task {
    pub id : usize, 
    pub description : String, 
    pub completed : bool, 
}

/**
 * Now associate methods with the Task struct
 */
impl Task {
    pub fn new(id : usize, description : String) -> Self {
        Self {
            id, 
            description, 
            completed: false
        }
    }

    pub fn mark_complete(&mut self) {
        self.completed = true;
    }

    pub fn print(&mut self) {
        println!("--------------------------");
        println!("|\tTask ID : {}", self.id);
        println!("|\tDescription : {}", self.description);
        println!("|\tCompleted? {}", if self.completed {"Yes"} else {"No"});
        println!("--------------------------");
    }

}