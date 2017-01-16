// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![cfg_attr(not(feature="clippy"), allow(unknown_lints))]

extern crate byteorder;
extern crate bit_vec;

#[macro_use]
extern crate cfg_if;

#[macro_use]
mod macros;

pub mod field;
mod encodable;

// default: pregen/mod.rs
// build    OUT_DIR/mod.rs
// pregen:  pregen/mod.rs

cfg_if! {
    if #[cfg(all(feature="amqp0-build-primitives", not(feature="amqp0-pregen-primitives")))] {
        include!(concat!(env!("OUT_DIR"), "/mod.rs"));
    }
    else {
        include!(concat!("../pregen/mod.rs"));
    }
}

pub use self::encodable::Encodable;

pub trait ProtocolFramePayload<'a>: Encodable {
    type Method: ProtocolMethod<'a>;

    fn as_method(&self) -> Option<&Self::Method>;
}

pub trait ProtocolMethod<'a> {
    type Start: ProtocolMethodPayload + 'a;
    fn as_start(&self) -> Option<&Self::Start>;
}

pub trait ProtocolMethodPayload: Encodable {
    fn class(&self) -> Class;
    fn class_name(&self) -> &'static str;
    fn class_id(&self) -> u16;
    fn method_name(&self) -> &'static str;
    fn method_id(&self) -> u16;
}
