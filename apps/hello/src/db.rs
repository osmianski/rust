pub fn connect() -> Result<rusqlite::Connection, crate::Error> {
    let path = path();

    Ok(rusqlite::Connection::open(&path)?)
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
