use crate::activity::Activity;
use crate::db::Db;

mod activity;
mod db;

fn main() {
    let mut todolistsdb = Db::new();
    let activity1 = Activity::new(1, "code rust 1 hour.".to_string());
    let activity2 = Activity::new(2, "code editing video for youtube.".to_string());
    let activity3 = Activity::new(3, "buy some food and beverages in groceries.".to_string());

    todolistsdb.add(activity1);
    todolistsdb.add(activity2.clone());
    todolistsdb.add(activity3);

    todolistsdb.complete_task(activity2.id);

    todolistsdb.get_all();


}
