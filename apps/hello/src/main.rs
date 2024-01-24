mod db;
mod env;
mod http;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Db(rusqlite::Error),
    Http(crate::http::Error),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<rusqlite::Error> for Error {
    fn from(e: rusqlite::Error) -> Self {
        Self::Db(e)
    }
}

impl From<crate::http::Error> for Error {
    fn from(e: crate::http::Error) -> Self {
        Self::Http(e)
    }
}

fn main() -> Result<(), Error> {
    // Load the environment variables from the `.env` file. If the file does not exist,
    // the function loads nothing.
    env::load(".env");

    // `args`` is an iterator over the command line arguments. The first argument is always
    // the name of the executable. The second optional argument is the name of the command.
    let args = std::env::args();

    // `args` iterator returns `String` instances. `command_name` owns the second argument
    // if it exists, or is set to `"list"` string otherwise.
    let command_name = args.skip(1).next().unwrap_or("list".to_string());

    // Match the command name against the list of predefined commands and execute the
    // corresponding function. If the command is not found, print an error message and exit
    // with a non-zero exit code.
    match command_name.as_str() {
        "list" => Ok(list()),
        "serve" => serve(),
        "migrate" => migrate(),
        _ => {
            println!("Could not find command {}", command_name);
            std::process::exit(1);
        }
    }
}

fn list() {
    println!("{:20}{}", "list", "List available commands");
    println!("{:20}{}", "migrate", "Run database migrations");
    println!("{:20}{}", "serve", "Run HTTP server");
}

fn serve() -> Result<(), Error> {
    let port = std::env::var("APP_PORT").unwrap_or("8021".to_string());
    let addr = format!("0.0.0.0:{}", port);

    println!("HTTP server is running on {}", &addr);

    let listener = std::net::TcpListener::bind(addr)?;

    for stream in listener.incoming() {
        http::handle_connection(stream?)?;
    }

    Ok(())
}

fn migrate() -> Result<(), Error> {
    let path_str = db::path();
    let path = std::path::Path::new(&path_str);

    if path.exists() {
        std::fs::remove_file(&path_str)?;
    }

    if let Some(parent_path) = path.parent() {
        std::fs::create_dir_all(parent_path)?;
    }

    std::fs::File::create(&path_str)?;

    let db = db::connect()?;

    db.execute(
        "CREATE TABLE posts (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            content TEXT
        )",
        (),
    )?;

    db.execute(
        "INSERT INTO posts (title) VALUES (?1)",
        ("Hello world!".to_string(),),
    )?;

    db.execute(
        "INSERT INTO posts (title) VALUES (?1)",
        ("Hello again!".to_string(),),
    )?;

    println!("posts table migrated");

    Ok(())
}
