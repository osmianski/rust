mod home;

use crate::basics::Result;
use lib::cli::Console;
use lib::http::Request;
use lib::http::Response;
use std::net::TcpStream;

pub fn handle_connection(console: &mut Console, stream: &TcpStream) -> Result<()> {
    if let Err(err) = handle_connection_but_not_errors(console, &stream) {
        let response = Response::new(500, "Server error".to_string(), format!("{:?}", err));

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

#[cfg(test)]
pub mod fake {
    use lib::http::{Fake, Request};

    pub fn get(uri: &'static str) -> Fake {
        let mut request = Request::new("GET".to_string(), uri.to_string());

        Fake::new(super::handle(&mut request).unwrap())
    }
}

fn handle(request: &mut Request) -> Result<Response> {
    route(request)
}

fn route(request: &mut Request) -> Result<Response> {
    if request.is("GET /") {
        home::show::handle(request)
    } else {
        Ok(Response::not_found())
    }
}
