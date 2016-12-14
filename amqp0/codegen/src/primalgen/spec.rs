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
use super::{MethodModuleWriter, HeadersStructWriter};

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
        try!(self.write_method_enum(writer));
        try!(self.write_header_enum(writer));
        try!(self.write_frame_enum(writer));

        let mut class_methods = Vec::with_capacity(self.spec.classes().len());

        try!(writeln!(writer, "\n// Class Modules"));
        for class in self.spec.classes().values() {
            let module_name = class.name().to_snake_case();
            try!(writeln!(writer, "pub mod {} {{", module_name));

            let property_writer = HeadersStructWriter::new(class, &self.domain_mapper);
            try!(property_writer.write_to(writer));

            let method_writers = class.methods().iter()
                .map(|method| MethodModuleWriter::new(class, method, &self.domain_mapper))
                .collect::<Vec<_>>();

            for method_writer in &method_writers {
                try!(method_writer.write_rust_to(writer));
            }

            let methods = method_writers.iter()
                .map(|w| (w.struct_name().to_owned(), w.has_lifetimes()))
                .collect::<Vec<_>>();

            let has_lifetimes = methods.iter().any(|&(_, has_lifetimes)| has_lifetimes);
            let lifetimes = if has_lifetimes { "<'a>" } else {""};

            try!(writeln!(writer, "pub enum Method{} {{", lifetimes));
            for (name, has_lifetimes) in methods {
                let lifetimes = if has_lifetimes { "<'a>" } else {""};
                try!(writeln!(writer, "{0}({0}{1}),", name, lifetimes));
            }
            try!(writeln!(writer, "}} // enum Method\n"));
            try!(writeln!(writer, "}} // mod {}\n", module_name));

            let has_methods = !class.methods().is_empty();
            let pascal_case = class.name().to_pascal_case();
            class_methods.push((pascal_case, module_name, has_lifetimes, has_methods));
        }

        let has_lifetimes = class_methods.iter().map(|c| c.3).any(|c| c);
        let lifetimes = if has_lifetimes { "<'a>" } else { "" };

        try!(writeln!(writer, "\n// Class methods"));
        for &(ref pascal_case, ref module, has_lifetimes, has_methods) in &class_methods {
            if has_methods {
                let lifetimes = if has_lifetimes { "<'a>" } else { "" };
                try!(writeln!(writer, "type {0}Method{2} = {1}::Method{2};", pascal_case, module, lifetimes));
            }
        }

        try!(writeln!(writer, "\npub enum Method{} {{", lifetimes));
        for (pascal_case, _, has_lifetimes, has_methods) in class_methods {
            if has_methods {
                let lifetimes = if has_lifetimes { "<'a>" } else { "" };
                try!(writeln!(writer, "{0}({0}Method{1}),", pascal_case, lifetimes));
            }
            else {
                try!(writeln!(writer, "{},", pascal_case));
            }
        }
        try!(writeln!(writer, "}} // enum Method"));

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

    pub fn write_method_enum<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {

        Ok(())
    }

    pub fn write_header_enum<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        try!(writeln!(writer, "\npub enum Header<'a> {{"));
        for class in self.spec.classes().values() {
            let pascal_case = class.name().to_pascal_case();
            if class.fields().is_empty() {
                try!(writeln!(writer, "{},", pascal_case));
            }
            else {
                let snake_case = class.name().to_snake_case();
                try!(writeln!(writer, "{}({}::Headers<'a>),", pascal_case, snake_case));
            }
        }
        try!(writeln!(writer, "}} // enum Header"));

        Ok(())
    }

    pub fn write_frame_enum<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        let frame_types = self.spec.frame_types();
        if frame_types.is_empty() {
            return Ok(())
        }

        try!(writeln!(writer, "\npub enum Frame<'a> {{"));
        for frame_type in frame_types.keys() {
            let name_start = if frame_type.starts_with("frame-") { 6 } else { 0 };
            let pascal_case = (&frame_type[name_start..]).to_pascal_case();

            try!(match pascal_case.as_str() {
                "Method" | "OobMethod" => writeln!(writer, "{}(Method<'a>),", pascal_case),
                "Header" | "OobHeader" => writeln!(writer, "{}(Header<'a>),", pascal_case),
                "Body" | "OobBody" => writeln!(writer, "{}(&'a [u8]),", pascal_case),
                _ => writeln!(writer, "{},", pascal_case),
            });

        }
        try!(writeln!(writer, "}} // enum Frame"));

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