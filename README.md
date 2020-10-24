# Tide Fluent Routes &emsp; [![Build Status](https://github.com/mendelt/tide-fluent-routes/workflows/Build/badge.svg)](https://github.com/mendelt/tide-fluent-routes/actions?query=workflow%3ABuild+event%3Apush+branch%3Amaster)

Tide Fluent Routes is a fluent api to define routes for the Tide HTTP framework.
At the moment it supports setting up paths, you can integrate middleware at any place in the
route-tree and you can integrate endpoints.
Some things that are possible with Tide-native routes are not (yet) possible;
- Tide prefix routes are not implemented
- you can not nest Tide servers

To use this you can import Tide Fluent Routes with `use tide_fluent_routes::prelude::*` it
introduces the `register` extension method on the `Tide::Server` to register routes from a
RouteBuilder.
A RouteBuilder can be initialized using the `route()` method.
You can register simple endpoints like this;
```rust
use tide_fluent_routes::prelude::*;

let mut server = tide::Server::new();

server.register(
   root()
       .get(endpoint)
       .post(endpoint));
```
Fluent Routes follows conventions from Tide. All HTTP verbs are supported the same way. Paths
can be extended using `at` but this method takes a router closure that allows building the route
as a tree.
A complete route tree can be defined like this;
```rust

server.register(
    root()
        .get(endpoint)
        .post(endpoint)
        .at("api/v1", |route| route
            .get(endpoint)
            .post(endpoint)
        )
        .at("api/v2", |route| route
            .get(endpoint)
            .post(endpoint)
        )
);
```
This eliminates the need to introduce variables for partial pieces of your route tree.

Including routes defined in other functions also looks very natural, this makes it easy
to compose large route trees from smaller trees defined elsewhere;
```rust

fn v1_routes(routes: RouteSegment<()>) -> RouteSegment<()> {
    routes
        .at("articles", |route| route
            .get(endpoint)
            .post(endpoint)
            .at(":id", |route| route
                .get(endpoint)
                .put(endpoint)
                .delete(endpoint)
            )
        )
}

fn v2_routes(routes: RouteSegment<()>) -> RouteSegment<()> {
    routes
        .at("articles", |route| route
            .get(endpoint))
}

server.register(
    root()
        .get(endpoint)
        .post(endpoint)
        .at("api/v1", v1_routes)
        .at("api/v2", v2_routes));
```

With vanilla Tide routes it can be hard to see what middleware is active for what
endpoints.
Adding middleware to a tree is easy, and its very clear where the middleware is applied;
```rust
server.register(
    root()
        .get(endpoint)
        .post(endpoint)
        .at("api/v1", |route| route
            .with(dummy_middleware, |route| route
                .get(endpoint)
            )
            .post(endpoint)
        )
        .at("api/v2", |route| route
            .get(endpoint)
            .get(endpoint)
        ),
);
```

Serving directories is possible using `serve_dir`, this works the same as with normal Tide routes,
fluent routes adds the `serve_file` convenience method for serving single files.
```rust
use tide_fluent_routes::prelude::*;
use tide_fluent_routes::fs::ServeFs;

let mut server = tide::Server::new();

server.register(
    root()
        .serve_file("files/index.html").unwrap()
        .at("img", |r| r
            .serve_dir("files/images").unwrap()
        )
);
```

*version: 0.1.3*
## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
