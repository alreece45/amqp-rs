// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub mod codegen;
pub mod parser;

mod version;

use std::borrow::{Borrow, Cow};
use std::collections::{BTreeMap, HashMap};

pub use self::codegen::write_generated;
pub use self::parser::parse;
pub use self::version::Version;

#[derive(Debug)]
pub struct Specs<'a> {
    specs: Vec<Spec<'a>>,
}

#[derive(Debug)]
pub struct Spec<'a> {
    classes: BTreeMap<String, Class<'a>>,
    constants: BTreeMap<String, Constant<'a>>,
    domains: BTreeMap<String, Domain<'a>>,
    version: Version,
}

#[derive(Debug)]
pub enum Assertion {
    Null,
    NotNull,
    ChannelMax,
    NotZero,
    Enum(Vec<String>),
    Length(usize),
    Regexp(String),
    Syntax(String),
}

#[derive(Debug)]
pub struct Class<'a> {
    name: Cow<'a, str>,
    fields: Vec<ClassField<'a>>,
    index: Cow<'a, str>,
    methods: Vec<Method<'a>>,
}

#[derive(Debug)]
pub struct ClassField<'a> {
    name: Cow<'a, str>,
    domain: Cow<'a, str>,
}

#[derive(Debug)]
pub struct Constant<'a> {
    name: Cow<'a, str>,
    value: Cow<'a, str>,
    class: Option<Cow<'a, str>>,
}

#[derive(Debug)]
pub struct Domain<'a> {
    name: Cow<'a, str>,
    mapping: Cow<'a, str>,
}

#[derive(Debug)]
pub struct ClassMethodField<'a> {
    name: Cow<'a, str>,
    domain: Cow<'a, str>,
    assertions: Vec<Assertion>,
    is_reserved: bool,
}

#[derive(Debug)]
pub struct Method<'a> {
    name: Cow<'a, str>,
    index: Cow<'a, str>,
    chassis: HashMap<String, String>,
    response: Option<Cow<'a, str>>,
    fields: Vec<ClassMethodField<'a>>,
    is_synchronous: bool,
}

impl<'a> Spec<'a> {
    pub fn new(version: Version) -> Self {
        Spec {
            classes: BTreeMap::new(),
            constants: BTreeMap::new(),
            domains: BTreeMap::new(),
            version: version,
        }
    }

    pub fn classes(&self) -> &BTreeMap<String, Class<'a>> {
        &self.classes
    }
    pub fn constants(&self) -> &BTreeMap<String, Constant<'a>> {
        &self.constants
    }
    pub fn domains(&self) -> &BTreeMap<String, Domain<'a>> {
        &self.domains
    }
    pub fn version(&self) -> &Version {
        &self.version
    }
}

impl<'a> Class<'a> {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn fields(&self) -> &[ClassField<'a>] {
        &self.fields
    }

    pub fn index(&self) -> &str {
        &self.index
    }

    pub fn methods(&self) -> &[Method<'a>] {
        &self.methods
    }
}

impl<'a> ClassField<'a> {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn domain(&self) -> &str {
        &self.domain
    }
}

impl<'a> Constant<'a> {
    pub fn name(&self) -> &str {
        self.name.borrow()
    }

    pub fn value(&self) -> &str {
        self.value.borrow()
    }

    pub fn class(&self) -> Option<&str> {
        self.class.as_ref().map(|c| c.borrow())
    }
}

impl<'a> Domain<'a> {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn mapping(&self) -> &str {
        &self.mapping
    }
}

impl<'a> ClassMethodField<'a> {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn domain(&self) -> &str {
        &self.domain
    }

    pub fn assertions(&self) -> &[Assertion] {
        &self.assertions
    }

    pub fn is_reserved(&self) -> bool {
        self.is_reserved
    }
}

impl<'a> Method<'a> {
    pub fn new<N, I>(name: N, index: I, is_synchronous: bool) -> Self
        where N: Into<Cow<'a, str>>,
              I: Into<Cow<'a, str>>
    {
        let name = name.into();
        trace!("Method: {}", &name);
        Method {
            name: name,
            index: index.into(),
            chassis: HashMap::new(),
            response: None,
            fields: vec![],
            is_synchronous: is_synchronous,
        }
    }
    pub fn index(&self) -> &str {
        &self.index
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn chassis(&self) -> &HashMap<String, String> {
        &self.chassis
    }
    pub fn fields(&self) -> &[ClassMethodField<'a>] {
        &self.fields
    }
    pub fn response(&self) -> Option<&str> {
        self.response.as_ref().map(|s| s.borrow())
    }
    pub fn is_synchronous(&self) -> bool {
        self.is_synchronous
    }
}
