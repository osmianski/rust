use lib::http::escape_html;
use lib::http::Request;
use lib::http::Response;

pub fn response(request: &Request, component: &str, props: String) -> Response {
    let url = &request.uri;
    let version = "".to_string();

    let mut data = format!("{{\"component\":\"{component}\",\"props\":{props},\"url\":\"{url}\",\"version\":\"{version}\"}}");

    if request.headers.get("x-inertia").is_some() {
        let mut response = Response::json(data);
        
        response.header("Vary".to_string(), "Accept".to_string());
        response.header("X-Inertia".to_string(), "true".to_string());

        return response;
    }

    data = escape_html(data);

    let vite_port = std::env::var("VITE_PORT").unwrap_or("5173".to_string());

    let vite = format!("
<script type=\"module\" src=\"http://127.0.0.1:{vite_port}/@vite/client\"></script>
<script type=\"module\" src=\"http://127.0.0.1:{vite_port}/js/app.js\"></script>
");

    Response::html(format!(
            "<!DOCTYPE html>
<html lang=\"en\">
    <head>
        <meta charset=\"utf-8\">
        <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">
        <link rel=\"stylesheet\" href=\"https://rsms.me/inter/inter.css\">
        {vite}
    </head>
    <body>
        <div id=\"app\" data-page=\"{data}\"></div>
    </body>
</html>
"))
}
