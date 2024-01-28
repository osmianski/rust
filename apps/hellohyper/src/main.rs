use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use lib::env;
use std::convert::Infallible;
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    env::load(".env");

    let port = std::env::var("APP_PORT").unwrap_or("8000".to_string());
    let addr = SocketAddr::from(([0, 0, 0, 0], port.parse::<u16>().unwrap()));
    let listener = TcpListener::bind(addr).await?;

    println!("HTTP server is running on {}", &addr);

    // We start a loop to continuously accept incoming connections
    loop {
        let (stream, _) = listener.accept().await?;

        // Use an adapter to access something implementing `tokio::io` traits as if they implement
        // `hyper::rt` IO traits.
        let io = TokioIo::new(stream);

        // Spawn a tokio task to serve multiple connections concurrently
        tokio::task::spawn(async move {
            // Finally, we bind the incoming connection to our `hello` service
            if let Err(err) = http1::Builder::new()
                // `service_fn` converts our function in a `Service`
                .serve_connection(io, service_fn(hello))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}

async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    #[test]
    fn it_casts_any_error_to_box_dyn_error() {
        if let Err(err) = outer() {
            println!("Error: {}", err);
            return;
        }

        assert!(false);
    }

    fn outer() -> Result<(), Box<dyn Error>> {
        inner()?;
        Ok(())
    }

    fn inner() -> Result<(), std::io::Error> {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "test"))
    }
}