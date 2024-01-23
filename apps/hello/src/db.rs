pub fn connect() -> rusqlite::Connection {
    let path = path();

    rusqlite::Connection::open(&path).unwrap()
}

pub fn path() -> String {
    std::env::var("DB_PATH").unwrap_or("storage/db.sqlite".to_string())
}

#[derive(Debug)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub content: Option<String>,
}