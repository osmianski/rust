pub mod show {
    use crate::basics::Result;
    use crate::inertia;
    use lib::http::Request;
    use lib::http::Response;
    use serde_json::json;

    pub fn handle(request: &Request) -> Result<Response> {
        Ok(inertia::response(request, "Home", json!({
            "foo": "bar"
        }).to_string()))
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
