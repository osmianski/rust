use crate::basics::Result;
use crate::http;
use lib::cli::Console;

pub fn run(console: &mut Console) -> Result<()> {
    let port = std::env::var("APP_PORT").unwrap_or("8000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    console.writeln(format!("HTTP server is running on {}", &addr).as_str())?;

    let listener = std::net::TcpListener::bind(addr)?;

    for stream in listener.incoming() {
        http::handle_connection(console, &stream?)?;
    }

    Ok(())
}
