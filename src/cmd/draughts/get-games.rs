use book_planner::server;

server!("GET", {
    let response = Response::builder()
        .status(200)
        .body(Body::from("These are your games"))
        .unwrap();

    Ok(response)
});
