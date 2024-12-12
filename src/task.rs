use chrono::{NaiveDate, Local};


/* *
 * Define a task entity by a 
 * struct based method because
 * Rust does not support classes 
 */
pub struct Task {
    pub id : usize, 
    pub description : String, 
    pub completed : bool, 
    pub assign_date : NaiveDate,
    pub complete_date : Option<NaiveDate>
}

/**
 * Now associate methods with the Task struct
 */
impl Task {
    pub fn new(id : usize, description : String) -> Self {
        Self {
            id, 
            description, 
            completed: false,
            assign_date : Local::now().date_naive(), 
            complete_date : None 
        }
    }

    pub fn mark_complete(&mut self) {
        self.completed = true;
        self.complete_date = Some(Local::now().date_naive());
    }

    pub fn print(&mut self) {
        println!("--------------------------");
        println!("|\tTask ID : {}", self.id);
        println!("|\tDescription : {}", self.description);
        println!("|\tAssigned On : {}", self.assign_date);
        println!("|\tCompleted? {}", if self.completed {"Yes"} else {"No"});
        if self.completed {
            println!("\tCompleted On : {:?}", self.complete_date);
        }
        println!("--------------------------");
    }

}