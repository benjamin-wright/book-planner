#[macro_export]
macro_rules! server {
    ($method:expr, $body:expr) => {
        use std::net::SocketAddr;

        use hyper::server::conn::Http;
        use hyper::service::service_fn;
        use tokio::net::TcpListener;
        use hyper::{Request, Body, Response};

        async fn handler(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
            println!("request: {:?}!", req.method());
            $body
        }

        #[tokio::main(flavor = "current_thread")]
        async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
            let listener = TcpListener::bind(addr).await?;
            println!("Listening on http://{}", addr);

            loop {
                let result = listener.accept().await;
                let stream = match result {
                    Ok((stream, _)) => stream,
                    Err(err) => {
                        return Err(err.into());
                    }
                };

                tokio::task::spawn(async move {
                    if let Err(err) = Http::new().serve_connection(stream, service_fn(handler)).await {
                        println!("Error serving connection: {:?}", err);
                    }
                });
            }
        }
    };
}

