use lib::http::escape_html;
use lib::http::Response;

pub fn response(component: &str, props: String) -> Response {
    let vite_port = std::env::var("VITE_PORT").unwrap_or("5173".to_string());

    let vite = format!("
<script type=\"module\" src=\"http://127.0.0.1:{vite_port}/@vite/client\"></script>
<script type=\"module\" src=\"http://127.0.0.1:{vite_port}/js/app.js\"></script>
");

    let data = escape_html(format!("{{\"component\":\"{component}\",\"props\":{props}}}"));

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
