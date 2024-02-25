use crate::activity::Activity;

pub struct Db {
    todolist: Vec<Activity>
}

impl Db {
    pub fn new() -> Self {
        Db {
            todolist: Vec::new()
        }
    }

    pub fn add(&mut self, activity: Activity) {
        self.todolist.push(activity);
    }

    pub fn complete_task(&mut self, activity_id: u32) {
        for activity in &mut self.todolist {
            if activity.id == activity_id {
                activity.completed = true
            }
        }
    }

    pub fn get_all(&self) {
        for activity in &self.todolist {
            println!("Activity {}: {} ({})", activity.id, activity.desc, activity.completed)
        }
    }
}