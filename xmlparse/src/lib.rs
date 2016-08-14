#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use] extern crate log;

extern crate xml;

pub mod amqp0;
