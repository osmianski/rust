pub mod show {
    use crate::basics::Result;
    use lib::http::{Request, Response};
    
    pub fn handle(_request: &Request) -> Result<Response> {
        Ok(Response::plain_text("Hello".to_string()))
    }
    
    #[cfg(test)]
    mod tests {
        use crate::http::fake;
        use lib::env;
    
        #[test]
        fn it_shows_hello() {
            env::load(".env.testing");
    
            assert!(fake::get("/").see("Hello"));
        }
    }    
}
