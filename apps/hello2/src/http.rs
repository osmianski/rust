mod home;

use crate::basics::Result;
use lib::{cli::Console, http::{Request, Response}};
use std::net::TcpStream;

pub fn handle_connection(console: &mut Console, stream: &TcpStream) -> Result<()> {
    if let Err(err) = handle_connection_but_not_errors(console, &stream) {
        let response = Response {
            status: 500,
            status_text: "Server error".to_string(),
            body: format!("{:?}", err),
        };
        
        let _ = response.send(&stream);
    }

    Ok(())
}

fn handle_connection_but_not_errors(console: &mut Console, stream: &TcpStream) -> Result<()> {
    let mut request = Request::receive_headers(&stream)?;

    console.write(format!("{} {}", request.method, request.uri).as_str())?;

    let response = handle(&mut request)?;

    response.send(&stream)?;

    console.writeln(format!(" -> {}", response.status).as_str())?;

    Ok(())
}

fn handle(request: &mut Request) -> Result<Response> {
    route(request)
}

fn route(request: &mut Request) -> Result<Response> {
    if request.is("GET /") {
        home::show(request)
    }
    else {
        Ok(Response::not_found())
    }
}