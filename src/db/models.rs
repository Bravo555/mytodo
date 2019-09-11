use super::schema::task;

#[derive(Insertable, Debug)]
#[table_name = "task"]
pub struct NewTask<'a> {
    pub title: &'a str,
    pub done: bool,
}

#[derive(Queryable, Debug)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub done: bool,
}
