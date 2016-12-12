// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;

use inflections::Inflect;
use specs::Spec;

use CodeGenerator;
use common::spec_mod_name;
use common::domain::DomainMapper;
use super::{MethodModuleWriter, PropertiesStructWriter};

pub struct SpecModuleWriter<'a> {
    struct_name: String,
    mod_name: String,
    domain_mapper: DomainMapper<'a>,
    spec: &'a Spec,
}

impl<'a> CodeGenerator for SpecModuleWriter<'a> {
    fn write_rust_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        if self.spec.classes().is_empty() {
            return Ok(());
        }

        try!(writeln!(writer, "#![allow(too_many_arguments)]\n"));

        try!(self.write_class_constants(writer));
        try!(self.write_method_constants(writer));

        try!(writeln!(writer, "\n// Class Modules"));
        for class in self.spec.classes().values() {
            try!(writeln!(writer, "pub mod {} {{", class.name().to_camel_case()));

            let property_writer = PropertiesStructWriter::new(class, &self.domain_mapper);
            try!(property_writer.write_to(writer));

            for method in class.methods() {
                let method_writer = MethodModuleWriter::new(class, method, &self.domain_mapper);
                try!(method_writer.write_rust_to(writer));
            }
            try!(writeln!(writer, "}}"));
        }

        /*
        try!(writeln!(writer, "fn parse(class: u8, method: u8, bytes: &[u8]) -> u8 {{"));
        try!(writeln!(writer, "match (class, method) {{"));
        for class in self.primalgen.spec.classes().values() {
            for method in class.methods() {
                try!(writeln!(writer, "({}, {}) => {}::{}", class.index(), method.index()));
            }
        }
        try!(writeln!(writer, "}}"));
        try!(writeln!(writer, "}}"));
        */

        //try!(writeln!(writer, "}}")); // primalgen.spec mod

        try!(writeln!(writer, "\n#[allow(non_camel_case_types)]"));
        try!(writeln!(writer, "pub struct {};", self.struct_name));
        try!(writeln!(writer, "impl ::Spec for {} {{}}\n", self.struct_name));

        Ok(())
    }
}

impl<'a> SpecModuleWriter<'a> {
    pub fn new(spec: &'a Spec) -> Self {
        let (minor, revision) = {
            let version = spec.version();
            (version.minor(), version.revision())
        };
        let struct_name = format!("{}{}_{}", spec.name().to_pascal_case(), minor, revision);
        let mod_name = spec_mod_name(spec);

        SpecModuleWriter {
            struct_name: struct_name,
            mod_name: mod_name,
            domain_mapper: DomainMapper::new(spec.domains()),
            spec: spec,
        }
    }

    pub fn mod_name(&self) -> &str {
        &self.mod_name
    }

    pub fn write_class_constants<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        try!(writeln!(writer, "// Class Constants"));
        for class in self.spec.classes().values() {
            let constant_class = class.name().to_constant_case();
            try!(writeln!(writer, "pub const CLASS_{}: u16 = {};", constant_class, class.index()));
        }

        Ok(())
    }

    pub fn write_method_constants<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        try!(write!(writer, "\n// Class Methods"));
        for class in self.spec.classes().values() {
            try!(writeln!(writer, ""));
            let constant_class = class.name().to_constant_case();
            for method in class.methods() {
                let constant_method = method.name().to_constant_case();
                try!(writeln!(writer, "pub const METHOD_{}_{}: u16 = {};", constant_class, constant_method, method.index()));
            }
        }

        Ok(())
    }
}