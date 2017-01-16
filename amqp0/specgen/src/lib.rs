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
extern crate log;
extern crate phf_codegen;
extern crate xml;

mod version;
mod parser;
mod codegen;

use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;
use xml::reader::EventReader;

use codegen::FormatRustCode;
use parser::Parser as SpecParser;

pub use parser::Error as ParseError;
pub use version::Version;

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
    methods: Vec<ClassMethod<'a>>,
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
pub struct ClassMethod<'a> {
    name: Cow<'a, str>,
    index: Cow<'a, str>,
    chassis: HashMap<String, String>,
    response: Option<Cow<'a, str>>,
    fields: Vec<ClassMethodField<'a>>,
    is_synchronous: bool,
    has_content: bool,
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

    pub fn parse_xml_path<P>(path: P) -> Result<Self, ParseError>
        where P: AsRef<Path>
    {
        let path = path.as_ref();
        let file = try!(File::open(&path));
        let file = BufReader::new(file);

        let mut parser = SpecParser::new();

        for event in EventReader::new(file) {
            let event = try!(event);
            parser = try!(parser.parse(&event));
        }

        Ok(try!(parser.into_spec()))
    }

    pub fn write_generated<W>(&self, name: &str, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        let (ungrouped, frame_types, response_codes) = {
            let (mut ungrouped, mut frame_types, mut response_codes) = (vec![], vec![], vec![]);
            #[allow(match_same_arms)]
            for constant in self.constants().values() {
                match constant.name().replace(" ", "-").as_str() {
                    "frame-min-size" | "frame-end" => &mut ungrouped,
                    name if name.starts_with("frame-")
                        && name != "frame-error" => &mut frame_types,
                    _ if constant.value().len() == 3 => &mut response_codes,
                    _ => &mut ungrouped,
                }.push((constant.name().replace(" ", "-"), constant));
            };
            (ungrouped, frame_types, response_codes)
        };

        try!(writeln!(writer, "::Spec {{"));
        try!(writeln!(writer, "name: {},", name.format_rust()));
        try!(writeln!(writer, "classes: {},", self.classes().format_rust()));
        try!(writeln!(writer, "constants: {},", codegen::format_to_map(ungrouped.into_iter())));
        try!(writeln!(writer, "domains: {},", self.domains().format_rust()));
        try!(writeln!(writer, "frame_types: {},", codegen::format_to_map(frame_types.into_iter())));
        try!(writeln!(writer, "response_codes: {},", codegen::format_to_map(response_codes.into_iter())));
        try!(writeln!(writer, "version: {},", self.version().format_rust()));
        try!(writeln!(writer, "}}"));

        Ok(())
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

    pub fn methods(&self) -> &[ClassMethod<'a>] {
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
        &*self.name
    }

    pub fn value(&self) -> &str {
        &*self.value
    }

    pub fn class(&self) -> Option<&str> {
        self.class.as_ref().map(|c| &**c)
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

impl<'a> ClassMethod<'a> {
    pub fn new<N, I>(name: N, index: I, is_synchronous: bool, has_content: bool) -> Self
        where N: Into<Cow<'a, str>>,
              I: Into<Cow<'a, str>>
    {
        let name = name.into();
        trace!("Method: {}", &name);
        ClassMethod {
            name: name,
            index: index.into(),
            chassis: HashMap::new(),
            response: None,
            fields: vec![],
            is_synchronous: is_synchronous,
            has_content: has_content,
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
        self.response.as_ref().map(|s| &**s)
    }
    pub fn is_synchronous(&self) -> bool {
        self.is_synchronous
    }
    pub fn has_content(&self) -> bool {
        self.has_content
    }
}
