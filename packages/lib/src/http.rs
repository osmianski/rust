use std::io;
use std::io::prelude::*;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    MethodExpected,
    UriExpected,
    ParameterExpected(&'static str),
}

pub struct Request {
    pub method: String,
    pub uri: String,
    pub parameters: Parameters,
}

enum State {
    Method,
    Uri,
    Slash,
    Parameter(usize),
    WildcardParameter(usize),
}

pub type Parameters = std::collections::HashMap<String, String>;

impl Request {
    pub fn new(method: String, uri: String) -> Request {
        Request {
            method,
            uri,
            parameters: Parameters::new(),
        }
    }

    pub fn receive_headers(stream: &std::net::TcpStream) -> Result<Request, Error> {
        // Collect the request (which end with an empty line) into a `Vec` - an expandable array
        // of `String`s in the heap.
        let http_request: Vec<_> = std::io::BufReader::new(stream)
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        // Extract the request method and URI from the first line of the request.
        let mut request_line = http_request[0].split(' ');
        let method = String::from(request_line.next().ok_or(Error::MethodExpected)?);
        let uri = String::from(request_line.next().ok_or(Error::UriExpected)?);

        Ok(Request::new(method, uri))
    }

    pub fn is(&mut self, route: &str) -> bool {
        let mut state = State::Method;
        let mut offset = 0;
        let mut route_offset = 0;
        let method_bytes = self.method.as_bytes();
        let uri_bytes = self.uri.as_bytes();
        let route_bytes = route.as_bytes();

        self.parameters.clear();

        for c in route.bytes() {
            match state {
                State::Method => match c {
                    b' ' => {
                        state = State::Uri;
                    }
                    _ => {
                        if route_offset >= method_bytes.len() || c != method_bytes[route_offset] {
                            return false;
                        }
                    }
                },
                State::Uri => {
                    if offset >= uri_bytes.len() || c != uri_bytes[offset] {
                        return false;
                    }

                    match c {
                        b'/' => {
                            state = State::Slash;
                        }
                        _ => {}
                    }

                    offset += 1;
                }
                State::Slash => match c {
                    b'{' => {
                        state = State::Parameter(route_offset + 1);
                    }
                    _ => {
                        if offset >= uri_bytes.len() || c != uri_bytes[offset] {
                            return false;
                        }

                        state = State::Uri;
                        offset += 1;
                    }
                },
                State::Parameter(parameter_offset) => match c {
                    b'*' => {
                        state = State::WildcardParameter(parameter_offset);
                    }
                    b'}' => {
                        let mut next_offset = offset;
                        while next_offset < uri_bytes.len() && uri_bytes[next_offset] != b'/' {
                            next_offset += 1;
                        }

                        self.parameters.insert(
                            String::from_utf8_lossy(&route_bytes[parameter_offset..route_offset])
                                .to_string(),
                            String::from_utf8_lossy(&uri_bytes[offset..next_offset]).to_string(),
                        );

                        state = State::Uri;
                        offset = next_offset;
                    }
                    _ => {}
                },
                State::WildcardParameter(parameter_offset) => match c {
                    b'}' => {
                        if route_offset + 1 != route.len() {
                            return false;
                        }

                        self.parameters.insert(
                            String::from_utf8_lossy(
                                &route_bytes[parameter_offset..route_offset - 1],
                            )
                            .to_string(),
                            String::from_utf8_lossy(&uri_bytes[offset..]).to_string(),
                        );

                        state = State::Uri;
                        offset = uri_bytes.len();
                    }
                    _ => {
                        return false;
                    }
                },
            }

            route_offset += 1;
        }

        match state {
            State::Uri | State::Slash => offset == uri_bytes.len(),
            _ => false,
        }
    }
}

pub struct Response {
    pub status: u16,
    pub status_text: String,
    pub body: String,
}

impl Response {
    pub fn not_found() -> Response {
        Response {
            status: 404,
            status_text: "Not found".to_string(),
            body: "Not found".to_string(),
        }
    }

    pub fn plain_text(text: String) -> Response {
        Response {
            status: 200,
            status_text: "OK".to_string(),
            body: text,
        }
    }

    pub fn send(&self, mut stream: &std::net::TcpStream) -> Result<(), io::Error> {
        stream.write_all(
            format!(
                "HTTP/1.1 {status} {status_text}r\n{headers}\r\n{body}\r\n",
                status = self.status,
                status_text = self.status_text,
                headers = "",
                body = self.body,
            )
            .as_bytes(),
        )?;

        Ok(())
    }
}

pub struct Fake {
    response: Response,
}

impl Fake {
    pub fn new(response: Response) -> Fake {
        Fake { response }
    }

    pub fn see(&self, s: &str) -> bool {
        self.response.body.contains(s)
    }
}
