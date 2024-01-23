use std::io::prelude::*;

struct Request {
    method: String,
    uri: String,
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
    pub fn receive(stream: &std::net::TcpStream) -> Request {
        // Collect the request (which end with an empty line) into a `Vec` - an expandable array
        // of `String`s in the heap.
        let http_request: Vec<_> = std::io::BufReader::new(stream)
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        // Extract the request method and URI from the first line of the request.
        let mut request_line = http_request[0].split(' ');
        let method = String::from(request_line.next().unwrap());
        let uri = String::from(request_line.next().unwrap());

        let request = Request {
            method,
            uri,
        };

        print!("{method} {uri}", method = request.method, uri = request.uri);

        request
    }

    pub fn is(&self, route: &str, parameters: &mut Parameters) -> bool {
        let mut state = State::Method;
        let mut offset = 0;
        let mut route_offset = 0;
        let method_bytes = self.method.as_bytes();
        let uri_bytes = self.uri.as_bytes();
        let route_bytes = route.as_bytes();

        parameters.clear();

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

                            parameters.insert(
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

                            parameters.insert(
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
    status_text: &'a str,
    headers: &'a str,
    body: &'a str,
}

impl Response<'_> {
    pub fn not_found<'a>() -> Response<'a> {
        Response {
            status: 404,
            status_text: "Not found",
            headers: "",
            body: "Not found",
        }
    }

    pub fn plain_text<'a>(text: &'a str) -> Response<'a> {
        Response {
            status: 200,
            status_text: "Not found",
            headers: "",
            body: text,
        }
    }

    pub fn send(&self, mut stream: &std::net::TcpStream) {
        stream.write_all(format!(
            "HTTP/1.1 {status} {status_text}r\n{headers}\r\n{body}\r\n", 
            status = self.status,
            status_text = self.status_text,
            headers = self.headers,
            body = self.body,
        ).as_bytes()).unwrap();
    
        println!(" -> {status}", status = self.status);
            
    }
}

pub fn handle_connection(stream: std::net::TcpStream) {
    let request = Request::receive(&stream);
    let mut parameters = Parameters::new();

    if request.is("GET /", &mut parameters) {
        return Response::plain_text("Hello").send(&stream);
    }
    else if request.is("GET /hello/{name}", &mut parameters) {
        let text = format!("Hello, {}", parameters.get("name").unwrap());

        return Response::plain_text(text.as_str()).send(&stream);
    }
    else if request.is("GET /hello/{name*}", &mut parameters) {
        let text = format!("Hi, {}", parameters.get("name").unwrap());

        return Response::plain_text(text.as_str()).send(&stream);
    }

    Response::not_found().send(&stream);
}