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

#[cfg(feature = "amqp0-build-specs")]
include!(concat!(env!("OUT_DIR"), "/mod.rs"));
#[cfg(not(feature = "amqp0-build-specs"))]
include!(concat!("../pregen/mod.rs"));

extern crate phf;

mod class;
mod spec;

use phf::OrderedMap;

pub use self::class::ClassMethodFieldAssertion;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Constant {
    name: &'static str,
    value: u32,
    class: Option<&'static str>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Version {
    minor: u16,
    revision: u16,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Class {
    name: &'static str,
    fields: &'static [ClassField],
    index: u16,
    methods: &'static [ClassMethod],
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ClassField {
    name: &'static str,
    domain: &'static str,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ClassMethod {
    name: &'static str,
    index: u16,
    response: Option<&'static str>,
    fields: &'static [ClassMethodField],
    is_synchronous: bool,
    has_content: bool,

    chassis_server: Option<&'static str>,
    chassis_client: Option<&'static str>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ClassMethodField {
    name: &'static str,
    domain: &'static str,
    assertions: &'static [ClassMethodFieldAssertion],
    is_reserved: bool,
}

#[derive(Debug, Clone)]
pub struct Spec {
    name: &'static str,
    classes: &'static OrderedMap<&'static str, Class>,
    constants: &'static OrderedMap<&'static str, Constant>,
    domains: &'static OrderedMap<&'static str, &'static str>,
    frame_types: &'static OrderedMap<&'static str, Constant>,
    response_codes: &'static OrderedMap<&'static str, Constant>,
    version: Version,
}

impl Constant {
    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn value(&self) -> u32 {
        self.value
    }

    pub fn class(&self) -> Option<&'static str> {
        self.class
    }
}

impl Version {
    pub fn minor(&self) -> u16 {
        self.minor
    }

    pub fn revision(&self) -> u16 {
        self.revision
    }
}
