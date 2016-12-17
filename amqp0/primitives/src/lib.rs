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

#[macro_use]
mod macros;

#[cfg(not(feature = "amqp0-build-primitives"))]
include!(concat!("../pregen/mod.rs"));
#[cfg(feature = "amqp0-build-primitives")]
include!(concat!(env!("OUT_DIR"), "/mod.rs"));

pub mod field;

pub trait Protocol<'a> {
    type Frame: 'a;

    fn protocol_header() -> &'static [u8];
}

pub trait ProtocolFramePayload<'a> {
    type Method: ProtocolMethod<'a>;

    fn as_method(&self) -> Option<&Self::Method>;
}

pub trait ProtocolMethod<'a> {
    type Start: ProtocolMethodPayload + 'a;
    fn as_start(&self) -> Option<&Self::Start>;
}

pub trait ProtocolMethodPayload {
    fn class_id(&self) -> u16;
    fn method_id(&self) -> u16;
    fn payload_size(&self) -> usize;
}

