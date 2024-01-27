use lib::http::Response;

pub fn response(data: String) -> Response {
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
", vite = "", data = data))
}
