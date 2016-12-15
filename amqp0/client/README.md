
AMQP client for Rust
=======================

Library to connect and use AMQP services.

Features
--------

 * Avoids heap allocations when parsing.
 * Keeps a pool of Tables and Lists to use -- to avoid allocating new ones for each message