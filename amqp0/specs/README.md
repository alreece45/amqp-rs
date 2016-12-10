AMQP Specifications
===================

Exposes a Rust interface for the AMQP specifications.

Pregenerated Files
==================

To save some processing power, and the need for dependencies, a 
pre-generated file is used by default. To generate from the XML files:

 * Testing/Usage from another crate: enable the amqp-build-specs feature
 * Committing/preparing to release: amqp-pregen-specs

Contributing/Developing
=======================

Most of the work is generated from the amqpspecgen crate.

When developing or preparing a contribution. You may update the 
can regenerate a file to use with the features above. The build feature
is recommended, as pregen overrides the original files and formats it --
the formatting takes a significant amount of time.

License 
=======
## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.