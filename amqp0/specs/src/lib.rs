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

extern crate phf;

use phf::OrderedMap;
use std::hash::{Hash, Hasher};

#[cfg(feature = "amqp0-build-specs")]
include!(concat!(env!("OUT_DIR"), "/mod.rs"));
#[cfg(not(feature = "amqp0-build-specs"))]
include!(concat!("../pregen/mod.rs"));

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Version {
    minor: u16,
    revision: u16,
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

use phf::PhfHash;

impl PartialEq for Spec {
    fn eq(&self, other: &Spec) -> bool {

        fn is_map_eq<K, V>(map1: &OrderedMap<K, V>, map2: &OrderedMap<K, V>) -> bool
            where K: Eq + PhfHash,
                  V: Eq
        {
            map1.entries().all(|(k, v1)| {
                match map2.get(k) {
                    Some(v2) if v1 == v2 => true,
                    _ => false,
                }
            })
        }

        self.name == other.name
            && self.version == other.version
            && is_map_eq(self.classes, other.classes)
            && is_map_eq(self.constants, other.constants)
            && is_map_eq(self.domains, other.domains)
            && is_map_eq(self.frame_types, other.frame_types)
            && is_map_eq(self.response_codes, other.response_codes)
    }
}
impl Eq for Spec {}

impl Hash for Spec {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        for (name, class) in self.classes {
            name.hash(state);
            class.hash(state);
        }
        for (name, value) in self.constants {
            name.hash(state);
            value.hash(state);
        }
        for (name, mapping) in self.domains {
            name.hash(state);
            mapping.hash(state);
        }
        for (name, value) in self.frame_types {
            name.hash(state);
            value.hash(state);
        }
        for (name, response_code) in self.response_codes {
            name.hash(state);
            response_code.hash(state);
        }
        self.version.hash(state)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Class {
    name: &'static str,
    fields: &'static [ClassField],
    index: u16,
    methods: &'static [ClassMethod],
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
pub struct ClassField {
    name: &'static str,
    domain: &'static str,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ClassMethodField {
    name: &'static str,
    domain: &'static str,
    assertions: &'static [ClassMethodFieldAssertion],
    is_reserved: bool,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum ClassMethodFieldAssertion {
    Null,
    NotNull,
    ChannelMax,
    NotZero,
    Enum(&'static [&'static str]),
    Length(usize),
    Regexp(&'static str),
    Syntax(&'static str),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Constant {
    name: &'static str,
    value: u32,
    class: Option<&'static str>,
}

impl Spec {
    pub fn name(&self) -> &'static str {
        self.name
    }
    pub fn classes(&self) -> &'static OrderedMap<&'static str, Class> {
        self.classes
    }
    pub fn domains(&self) -> &'static OrderedMap<&'static str, &'static str> {
        self.domains
    }
    pub fn frame_types(&self) -> &'static OrderedMap<&'static str, Constant> {
        self.frame_types
    }
    pub fn response_codes(&self) -> &'static OrderedMap<&'static str, Constant> {
        self.response_codes
    }
    pub fn version(&self) -> &Version {
        &self.version
    }
}

impl Class {
    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn fields(&self) -> &'static [ClassField] {
        self.fields
    }

    pub fn index(&self) -> u16 {
        self.index
    }

    pub fn methods(&self) -> &'static [ClassMethod] {
        self.methods
    }
}

impl ClassField {
    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn domain(&self) -> &'static str {
        self.domain
    }
}

impl ClassMethod {
    pub fn index(&self) -> u16 {
        self.index
    }
    pub fn name(&self) -> &str {
        self.name
    }
    pub fn fields(&self) -> &'static [ClassMethodField] {
        self.fields
    }
    pub fn has_content(&self) -> bool {
        self.has_content
    }
}

impl ClassMethodField {
    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn domain(&self) -> &'static str {
        self.domain
    }

    pub fn assertions(&self) -> &'static [ClassMethodFieldAssertion] {
        self.assertions
    }

    pub fn is_reserved(&self) -> bool {
        self.is_reserved
    }
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