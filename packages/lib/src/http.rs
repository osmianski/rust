use std::io;
use std::io::prelude::*;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    MethodExpected,
    UriExpected,
    ParameterExpected(&'static str),
    HeaderExpected,
}

pub struct Request {
    pub method: String,
    pub uri: String,
    pub parameters: Parameters,
    pub headers: Headers,
}

enum State {
    Method,
    Uri,
    Slash,
    Parameter(usize),
    WildcardParameter(usize),
}

pub type Parameters = std::collections::HashMap<String, String>;
pub type Headers = std::collections::HashMap<String, String>;

impl Request {
    pub fn new(method: String, uri: String) -> Request {
        Request {
            method,
            uri,
            parameters: Parameters::new(),
            headers: Headers::new(),
        }
    }

    pub fn receive_headers(stream: &std::net::TcpStream) -> Result<Request, Error> {
        let print_headers = false;

        enum State {
            StartLine,
            Headers(Request),
        }
        let lines = std::io::BufReader::new(stream).lines();
        let mut state = State::StartLine;

        if print_headers {
            println!("");
        }

        for line in lines {
            match line {
                Ok(line) => {
                    if line.is_empty() {
                        break;
                    }

                    if print_headers {
                        println!("{}", line);
                    }

                    match state {
                        State::StartLine => {
                            let mut parts = line.split_whitespace();
                            let method = parts.next().ok_or(Error::MethodExpected)?;
                            let uri = parts.next().ok_or(Error::UriExpected)?;
                            let request = Request::new(method.to_string(), uri.to_string());

                            state = State::Headers(request);
                        }

                        State::Headers(mut request) => {
                            let pos = line.find(':').ok_or(Error::HeaderExpected)?;

                            request.headers.insert(
                                line[..pos].trim().to_lowercase().to_string(),
                                line[pos + 1..].trim().to_string(),
                            );

                            state = State::Headers(request);
                        }
                    }
                }
                Err(e) => {
                    return Err(Error::Io(e));
                }
            }
        }

        if print_headers {
            println!("");
        }

        match state {
            State::StartLine => Err(Error::MethodExpected),
            State::Headers(request) => Ok(request),
        }
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
    pub headers: Headers,
}

impl Response {
    pub fn new(status: u16, status_text: String, body: String) -> Response {
        let mut response = Response {
            status,
            status_text,
            body,
            headers: Headers::new(),
        };

        response.header("Cache-Control".to_string(), "no-cache, private".to_string());
        response.header("X-Powered-By".to_string(), "simplicity".to_string());

        response
    }

    pub fn new_from_str(status: u16, status_text: &str, body: &str) -> Response {
        Response::new(status, status_text.to_string(), body.to_string())
    }

    pub fn not_found() -> Response {
        let mut response = Response::new_from_str(404, "Not found", "Not found");

        response.header("Content-Type".to_string(), "text/plain; charset=UTF-8".to_string());

        response
    }

    pub fn plain_text(text: String) -> Response {
        let mut response = Response::new(200, "OK".to_string(), text);

        response.header("Content-Type".to_string(), "text/plain; charset=UTF-8".to_string());

        response
    }

    pub fn html(html: String) -> Response {
        let mut response = Response::new(200, "OK".to_string(), html);

        response.header("Content-Type".to_string(), "text/html; charset=UTF-8".to_string());

        response
    }

    pub fn json(json: String) -> Response {
        let mut response = Response::new(200, "OK".to_string(), json);

        response.header("Content-Type".to_string(), "application/json".to_string());

        response
    }

    pub fn header(&mut self, name: String, value: String) {
        self.headers.insert(name, value);
    }

    pub fn send(&self, mut stream: &std::net::TcpStream) -> Result<(), io::Error> {
        let mut s = String::new();

        s.push_str("HTTP/1.1 ");
        s.push_str(&self.status.to_string());
        s.push_str(" ");
        s.push_str(&self.status_text);
        s.push_str("\r\n");

        for (name, value) in &self.headers {
            s.push_str(name);
            s.push_str(": ");
            s.push_str(value);
            s.push_str("\r\n");
        }

        s.push_str("\r\n");
        s.push_str(&self.body);
        s.push_str("\r\n");
        
        stream.write_all(s.as_bytes())?;

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
