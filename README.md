#amqp-rs

[![Build Status](https://travis-ci.org/alreece45/amqp-rs.svg?branch=master)](https://travis-ci.org/alreece45/amqp-rs)

Various modules for AMQP.

This library does not yet interact with clients or servers.

Still in early stages: **experimental** and under development.

| Crate | Description | Docs |
| --- | --- | --- |
amqp0-primitives | Basic structures for sending and receiving AMQP messages | [Docs](https://alreece45.github.io/amqp-rs/amqp0_primitives/index.html)
amqp0-parser-nom | Nom parsing to AMQP primitives | [Docs](https://alreece45.github.io/amqp-rs/amqp0_parser_nom/index.html)
amqp0-specs      | API for accessing spec information (methods, fields, etc) | [Docs](https://alreece45.github.io/amqp-rs/amqp0_specs/index.html)
amqp0-specgen    | Code to parse the AMQP specifications and create amqp0-specs |
amqp0-codegen    | Code Generation for other crates (primitives, parsers, etc) |

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.