use std::io::prelude::*;

#[derive(Debug)]
pub enum Error {
    MethodExpected,
    UriExpected,
    ParameterExpected(&'static str),
}

struct Request {
    method: String,
    uri: String,
    parameters: Parameters,
}

enum State {
    Method,
    Uri,
    Slash,
    Parameter(usize),
    WildcardParameter(usize),
}

type Parameters = std::collections::HashMap<String, String>;

impl Request {
    fn new(method: String, uri: String) -> Request {
        Request {
            method,
            uri,
            parameters: Parameters::new(),
        }
    }

    pub fn receive(stream: &std::net::TcpStream) -> Result<Request, crate::Error> {
        // Collect the request (which end with an empty line) into a `Vec` - an expandable array
        // of `String`s in the heap.
        let http_request: Vec<_> = std::io::BufReader::new(stream)
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        // Extract the request method and URI from the first line of the request.
        let mut request_line = http_request[0].split(' ');
        let method = String::from(request_line.next()
            .ok_or(Error::MethodExpected)?);
        let uri = String::from(request_line.next()
            .ok_or(Error::UriExpected)?);

        let request = Request::new(method, uri);

        print!("{method} {uri}", method = request.method, uri = request.uri);

        Ok(request)
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
                State::Method => {
                    match c {
                        b' ' => {
                            state = State::Uri;
                        },
                        _ => {
                            if route_offset >= method_bytes.len() || c != method_bytes[route_offset] {
                                return false;
                            }
                        },
                    }
                },
                State::Uri => {
                    if offset >= uri_bytes.len() || c != uri_bytes[offset] {
                        return false;
                    }
        
                    match c {
                        b'/' => {
                            state = State::Slash;
                        },
                        _ => {},
                    }

                    offset += 1;
                },
                State::Slash => {
                    match c {
                        b'{' => {
                            state = State::Parameter(route_offset + 1);
                        },
                        _ => {
                            if offset >= uri_bytes.len() || c != uri_bytes[offset] {
                                return false;
                            }

                            state = State::Uri;
                            offset += 1;
                        },
                    }
                },
                State::Parameter(parameter_offset) => {
                    match c {
                        b'*' => {
                            state = State::WildcardParameter(parameter_offset);
                        },
                        b'}' => {
                            let mut next_offset = offset;
                            while next_offset < uri_bytes.len() && uri_bytes[next_offset] != b'/' {
                                next_offset += 1;
                            }

                            self.parameters.insert(
                                String::from_utf8_lossy(&route_bytes[parameter_offset..route_offset]).to_string(),
                                String::from_utf8_lossy(&uri_bytes[offset..next_offset]).to_string(),
                            );

                            state = State::Uri;
                            offset = next_offset;
                        },
                        _ => {},
                    }
                },
                State::WildcardParameter(parameter_offset) => {
                    match c {
                        b'}' => {
                            if route_offset + 1 != route.len() {
                                return false;
                            }

                            self.parameters.insert(
                                String::from_utf8_lossy(&route_bytes[parameter_offset..route_offset - 1]).to_string(),
                                String::from_utf8_lossy(&uri_bytes[offset..]).to_string(),
                            );

                            state = State::Uri;
                            offset = uri_bytes.len();
                        },
                        _ => {
                            return false;
                        },
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

struct Response<'a> {
    status: u16,
    status_text: String,
    headers: &'a str,
    body: String,
}

impl Response<'_> {
    pub fn error<'a>(err: crate::Error) -> Response<'a> {
        Response {
            status: 500,
            status_text: "Server error".to_string(),
            headers: "",
            body: format!("{:?}", err),
        }
    }

    pub fn not_found<'a>() -> Response<'a> {
        Response {
            status: 404,
            status_text: "Not found".to_string(),
            headers: "",
            body: "Not found".to_string(),
        }
    }

    pub fn plain_text<'a>(text: String) -> Response<'a> {
        Response {
            status: 200,
            status_text: "OK".to_string(),
            headers: "",
            body: text,
        }
    }

    pub fn send(&self, mut stream: &std::net::TcpStream) -> Result<(), crate::Error> {
        stream.write_all(format!(
            "HTTP/1.1 {status} {status_text}r\n{headers}\r\n{body}\r\n", 
            status = self.status,
            status_text = self.status_text,
            headers = self.headers,
            body = self.body,
        ).as_bytes())?;
    
        println!(" -> {status}", status = self.status);

        Ok(())
            
    }
}

pub fn handle_connection(stream: std::net::TcpStream) -> Result<(), crate::Error> {
    let mut request = Request::receive(&stream)?;

    let response = handle_errors(route(&mut request));

    response.send(&stream)
}

fn handle_errors(result: Result<Response, crate::Error>) -> Response {
    match result {
        Ok(response) => response,
        Err(err) => Response::error(err),
    }
}

fn route(request: &mut Request) -> Result<Response, crate::Error> {
    if request.is("GET /") {
        home_show(request)
    } 
    else if request.is("GET /hello/{name}") {
        hello_show(request)
    } 
    else if request.is("GET /hello/{name*}") {
        hi_show(request)
    } 
    else if request.is("GET /posts") {
        posts_index(request)
    } 
    else {
        Ok(Response::not_found())
    }
}

fn home_show(_request: &Request) -> Result<Response, crate::Error> {
    Ok(Response::plain_text("Hello".to_string()))
}

fn hello_show(request: &Request) -> Result<Response, crate::Error> {
    let text = format!(
        "Hello, {}", 
        request.parameters.get("name")
            .ok_or(Error::ParameterExpected("name"))?,
    );

    Ok(Response::plain_text(text))
}

fn hi_show(request: &Request) -> Result<Response, crate::Error> {
    let text = format!(
        "Hi, {}", 
        request.parameters.get("name")
            .ok_or(Error::ParameterExpected("name"))?
    );

    Ok(Response::plain_text(text))
}

fn posts_index(_request: &Request) -> Result<Response, crate::Error> {
    let db = crate::db::connect()?;

    let mut stmt = db.prepare("SELECT * FROM posts")?;

    let posts = stmt.query_map([], |row| {
        Ok(crate::db::Post {
            id: row.get("id")?,
            title: row.get("title")?,
            content: row.get("content")?,
        })
    })?;

    let mut result = String::new();
    for post in posts {
        result.push_str(&format!("{:?}\n", post?));
    }

    Ok(Response::plain_text(result))
}