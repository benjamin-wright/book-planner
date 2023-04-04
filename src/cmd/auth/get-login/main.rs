use book_planner::server;

server!("GET", {
    let response = Response::builder()
        .status(200)
        .body(Body::from("Hi from this one"))
        .unwrap();

    Ok(response)
});
