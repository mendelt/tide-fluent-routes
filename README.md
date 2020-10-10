# Tide Fluent Routes &emsp; [![Build Status](https://github.com/mendelt/tide-fluent-routes/workflows/Build/badge.svg)](https://github.com/mendelt/tide-fluent-routes/actions?query=workflow%3ABuild+event%3Apush+branch%3Amaster)

Tide Fluent Routes is a fluent api to define routes for the Tide HTTP framework.
At the moment it supports setting up paths, you can integrate middleware at any place in the
route-tree and you can integrate endpoints.
Some things that are possible with Tide-native routes are not (yet) possible;
- middleware does not work for now, there is support missing for this in Tide
- Tide prefix routes are not implemented
- you can not nest Tide servers
- serving directories is not possible but a version of this is planned

To use this you can import Tide Fluent Routes with `use tide_fluent_routes::prelude::* it
introduces the `register` extension method on the `Tide::Server to register routes from a
RouteBuilder.
A RouteBuilder can be initialized using the `route()` method.
You can register simple endpoints like this;
```rust
use tide_fluent_routes::*;

let mut server = tide::Server::new();

server.register(
   root()
       .get(endpoint)
       .post(endpoint),
);
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
        ),
);
```
This eliminates the need to introduce variables for partial pieces of your route tree.

Another problem with Tide routes is that middleware that is only active for certain routes can
be difficult to maintain. Adding middleware to a tree is easy, and its very clear where the
middleware is applied and where not; (this is still a prototype and middleware is not actually
added right now)
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

*version: 0.1.0*
## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
