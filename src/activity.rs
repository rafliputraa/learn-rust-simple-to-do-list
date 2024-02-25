#[derive(Clone)]
pub struct Activity {
    pub id: u32,
    pub desc: String,
    pub completed: bool,
}

impl Activity {
    pub fn new(id: u32, desc: String) -> Activity {
        Activity{
            id,
            desc,
            completed: false,
        }
    }
}