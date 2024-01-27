#[derive(Debug)]
pub enum Error {
    Db(rusqlite::Error),
    Http(lib::http::Error),
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl From<rusqlite::Error> for Error {
    fn from(e: rusqlite::Error) -> Self {
        Self::Db(e)
    }
}

impl From<lib::http::Error> for Error {
    fn from(e: lib::http::Error) -> Self {
        match e {
            lib::http::Error::Io(e) => Self::Io(e),
            _ => Self::Http(e),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e)
    }
}


pub type Result<T> = std::result::Result<T, Error>;
