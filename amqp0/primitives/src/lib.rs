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
extern crate cfg_if;

#[macro_use]
mod macros;

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

pub mod field;

#[derive(Clone)]
pub struct Frame<P> {
    channel: u16,
    payload: P
}

impl<'a, P> Frame<P>
    where P: ProtocolFramePayload<'a>
{
    pub fn channel(&self) -> u16 {
        self.channel
    }
    pub fn payload(&self) -> &P {
        &self.payload
    }
}

impl<'a, P> Encodable for Frame<P>
    where P: ProtocolFramePayload<'a>
{
    fn encoded_size(&self) -> usize {
        4 + self.payload.encoded_size()
    }
}

use std::borrow::Cow;

impl<'a> Encodable for Cow<'a, [u8]> {
    fn encoded_size(&self) -> usize {
        self.len()
    }
}
impl<'a> Encodable for Cow<'a, str> {
    fn encoded_size(&self) -> usize {
        self.len()
    }
}

pub trait Protocol<'a> {
    type Frame: 'a;

    fn protocol_header() -> &'static [u8];
}

pub trait ProtocolFramePayload<'a>: Encodable {
    type Method: ProtocolMethod<'a>;

    fn as_method(&self) -> Option<&Self::Method>;
}

pub trait ProtocolMethod<'a> {
    type Start: ProtocolMethodPayload + 'a;
    fn as_start(&self) -> Option<&Self::Start>;
}

pub trait ProtocolMethodPayload: Encodable {
    fn class_id(&self) -> u16;
    fn method_id(&self) -> u16;
}

pub trait Encodable {
    fn encoded_size(&self) -> usize;
}

