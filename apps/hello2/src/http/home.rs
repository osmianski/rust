use crate::basics::Result;
use lib::http::{Request, Response};

pub fn show(_request: &Request) -> Result<Response> {
    Ok(Response::plain_text("Hello".to_string()))
}
