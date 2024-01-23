mod db;
mod env;
mod http;

fn main() {
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
    if command_name.eq("list") {
        list();
    } 
    else if command_name.eq("serve") {
        serve();
    } 
    else if command_name.eq("migrate") {
        migrate();
    } 
    else {
        println!("Could not find command {}", command_name);
        std::process::exit(1);
    }
}

fn list() {
    println!("{:20}{}", "list", "List available commands");
    println!("{:20}{}", "serve", "Run HTTP server");
}

fn serve() {
    let port = std::env::var("APP_PORT").unwrap_or("8021".to_string());
    let addr = format!("0.0.0.0:{}", port);

    println!("HTTP server is running on {}", &addr);

    let listener = std::net::TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        http::handle_connection(stream.unwrap());
    }
}

fn migrate() {
    let path_str = db::path();
    let path = std::path::Path::new(&path_str);

    if path.exists() {
        std::fs::remove_file(&path_str).unwrap();
    }

    if let Some(parent_path) = path.parent() {
        std::fs::create_dir_all(parent_path).unwrap();
    }

    std::fs::File::create(&path_str).unwrap();

    let db = db::connect();

    db.execute("CREATE TABLE posts (
        id INTEGER PRIMARY KEY,
        title TEXT NOT NULL,
        content TEXT
    )", ()).unwrap();

    db.execute("INSERT INTO posts (title) VALUES (?1)", (
        "Hello world!".to_string(),
    )).unwrap();

    db.execute("INSERT INTO posts (title) VALUES (?1)", (
        "Hello again!".to_string(),
    )).unwrap();

    println!("posts table migrated");
}