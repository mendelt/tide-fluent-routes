//! Example application showing fluent routing

use tide::{Request, Result};
use tide_fluent_routes::prelude::*;

#[async_std::main]
async fn main() {
    let mut server = tide::Server::new();
    server
        .register(
            root()
                .get(endpoint)
                .post(endpoint)
                .at("api/v1", |route| route.get(endpoint).post(endpoint))
                .at("api/v2", |route| route.get(endpoint).post(endpoint)),
        )
        .expect("Error setting up routes");

    server.listen("127.0.0.1:8080").await.unwrap();
}

async fn endpoint(_: Request<()>) -> Result {
    todo!()
}
