#![feature(async_closure)]

use book_planner::server;
use hyper::{Request, Body, Response};

async fn handler(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    println!("request: {:?}!", req.method());
    let response = Response::builder()
        .status(200)
        .body(Body::from("Hi from this one"))
        .unwrap();

    Ok(response)
}

server!("GET", handler);