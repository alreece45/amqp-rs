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
use super::{FrameEnumWriter, MethodModuleWriter, PropertiesStructWriter};

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

        let frame_writer = FrameEnumWriter::new(self.spec.frame_types());
        try!(frame_writer.write_rust_to(writer));

        try!(writeln!(writer, "\n// Class Modules"));
        for class in self.spec.classes().values() {
            let module_name = class.name().to_snake_case();
            try!(writeln!(writer, "pub mod {} {{", module_name));

            let property_writer = PropertiesStructWriter::new(class, &self.domain_mapper);
            try!(property_writer.write_to(writer));

            let method_writers = class.methods().iter()
                .map(|method| MethodModuleWriter::new(class, method, &self.domain_mapper))
                .collect::<Vec<_>>();

            for method_writer in &method_writers {
                try!(method_writer.write_rust_to(writer));
            }

            let enum_items = method_writers.iter()
                .map(|w| (w.struct_name().to_owned(), w.has_lifetimes()))
                .collect::<Vec<_>>();

            let has_lifetimes = enum_items.iter().any(|&(_, has_lifetimes)| has_lifetimes);
            let lifetimes = if has_lifetimes { "<'a>" } else {""};

            try!(writeln!(writer, "pub enum Method{} {{", lifetimes));
            for (name, has_lifetimes) in enum_items {
                let lifetimes = if has_lifetimes { "<'a>" } else {""};
                try!(writeln!(writer, "{0}({0}{1}),", name, lifetimes));
            }
            try!(writeln!(writer, "}} // enum Method\n"));
            try!(writeln!(writer, "}} // mod {}\n", module_name));
        }

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