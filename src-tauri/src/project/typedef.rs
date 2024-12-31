use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub color: String,
}

pub fn setup() {
    let connection = sqlite::open("database.db").unwrap();
    let sql = "
        CREATE TABLE IF NOT EXISTS project (
            id NUMBER PRIMARY KEY,
            name TEXT,
            color TEXT
        )
    ";
    let _ = connection.execute(sql);
}
