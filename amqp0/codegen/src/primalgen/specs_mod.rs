// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;
use std::io;
use inflections::Inflect;

use CodeGenerator;
use common::{Specs, Spec};

pub struct SpecsModuleWriter<'a> {
    specs: Specs<'a>,
}

impl<'a> CodeGenerator for SpecsModuleWriter<'a> {
    fn write_rust_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        // ensure that class ids remain consistent across the specs
        self.specs.assert_name_indexes_consistent();

        try!(writeln!(writer, ""));
        for spec in self.specs.iter() {
            try!(writeln!(writer, "pub mod {};", spec.mod_name()));
        }
        try!(writeln!(writer, ""));

        try!(self.write_frame_types(writer));
        try!(self.write_classes(writer));
        try!(self.write_methods(writer));
        try!(self.write_specs(writer));

        Ok(())
    }
}

impl<'a> SpecsModuleWriter<'a> {
    pub fn new<S>(specs: S) -> Self
        where S: Into<Cow<'a, [Spec]>>
    {
        SpecsModuleWriter {
            specs: Specs::new(specs)
        }
    }

    pub fn write_classes<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        try!(writeln!(writer, "//"));
        try!(writeln!(writer, "// Index values for classes shared among multiple specs"));
        try!(writeln!(writer, "//"));
        try!(writeln!(writer, "// Sometimes, the index value is repeated in different classes, but these are not reused"));
        try!(writeln!(writer, "// within a single protocol"));
        try!(writeln!(writer, "//"));
        try!(writeln!(writer, "// Classes are currently only considered common if they are used in more than one"));
        try!(writeln!(writer, "// spec. This behavior *may* change in the future as more specs are added."));
        try!(writeln!(writer, "//"));

        let common_classes = {
            let mut classes = self.specs.common_classes().into_iter().collect::<Vec<_>>();
            classes.sort_by(|&(_, a), &(_, b)| a.cmp(&b));
            classes
        };

        for (class_name, index) in common_classes {
            let constant_class = class_name.to_constant_case();
            try!(writeln!(writer, "pub const CLASS_{}: u16 = {};", constant_class, index));
        }
        try!(writeln!(writer, ""));

        Ok(())
    }

    pub fn write_frame_types<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        try!(writeln!(writer, "//"));
        try!(writeln!(writer, "// Frame types ids shared among multiple specs"));
        try!(writeln!(writer, "//"));

        let common_frame_types = {
            let mut frame_types = self.specs.common_frame_types().into_iter().collect::<Vec<_>>();
            frame_types.sort_by(|&(_, a), &(_, b)| a.cmp(&b));
            frame_types
        };

        for (class_name, index) in common_frame_types {
            let constant_class = class_name.to_constant_case();
            try!(writeln!(writer, "pub const {}: u8 = {};", constant_class, index));
        }
        try!(writeln!(writer, ""));

        Ok(())
    }

    pub fn write_methods<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        try!(writeln!(writer, "//"));
        try!(writeln!(writer, "// Index values for methods common among the different specs"));
        try!(writeln!(writer, "//"));
        try!(writeln!(writer, "// Methods are only considered common when:"));
        try!(writeln!(writer, "//"));
        try!(writeln!(writer, "//   * The index value is consistent across all of the specs"));
        try!(writeln!(writer, "//   * The method is used in more than one primalgen.spec"));
        try!(writeln!(writer, "//"));
        try!(writeln!(writer, "// This may change in the future-- in that case, methods *may* be removed, or"));
        try!(writeln!(writer, "// one of the requirements may be relaxed."));
        try!(writeln!(writer, "//"));

        let common_methods = {
            let mut methods = self.specs.common_methods().into_iter().collect::<Vec<_>>();
            methods.sort_by(|&(a, _), &(b, _)| a.cmp(b));
            methods
        };

        for (class_name, methods) in common_methods {
            if methods.is_empty() {
                continue
            };

            let methods = {
                let mut methods = methods.into_iter().collect::<Vec<_>>();
                methods.sort_by(|&(_, a), &(_, b)| a.cmp(&b));
                methods
            };

            let constant_class = class_name.to_constant_case();

            for (method_name, index) in methods {
                let constant_method = method_name.to_constant_case();

                if constant_method != "_" {
                    try!(writeln!(writer, "pub const METHOD_{}_{}: u16 = {};", constant_class, constant_method, index));
                }
            }
            try!(writeln!(writer, ""));
        }

        Ok(())
    }

    pub fn write_specs<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        try!(writeln!(writer, "//"));
        try!(writeln!(writer, "// Index values for methods common among the different specs"));
        try!(writeln!(writer, "//"));
        try!(writeln!(writer, "// Methods are only considered common when:"));
        try!(writeln!(writer, "//"));
        try!(writeln!(writer, "//   * The index value is consistent across all of the specs"));
        try!(writeln!(writer, "//   * The method is used in more than one primalgen.spec"));
        try!(writeln!(writer, "//"));
        try!(writeln!(writer, "// This may change in the future-- in that case, methods *may* be removed, or"));
        try!(writeln!(writer, "// one of the requirements may be relaxed."));
        try!(writeln!(writer, "//"));

        for spec in self.specs.iter() {
            let (minor, revision) = {
                let version = spec.version();
                (version.minor(), version.revision())
            };
            let struct_name = format!("{}{}_{}", spec.name().to_pascal_case(), minor, revision);

            try!(writeln!(writer, "\n#[allow(non_camel_case_types)]"));
            try!(writeln!(writer, "\n#[derive(Debug, Clone, PartialEq)]"));

            try!(writeln!(writer, "pub struct {};", struct_name));

            // impl Protocol
            try!(writeln!(writer, "impl<'a> ::Protocol<'a> for {} {{", struct_name));

            // Protocol::Frame
            try!(writeln!(writer, "type Frame = {}::Frame<'a>;", spec.mod_name()));

            // Protocol::protocol_header
            try!(writeln!(writer, "fn protocol_header() -> &'static [u8] {{"));
            let (minor, revision) = (spec.version().minor(), spec.version().revision());
            try!(writeln!(writer, "b\"AMQP\\x00\\x00\\x{:02x}\\x{:02x}\"", minor, revision));
            try!(writeln!(writer, "}} // fn protocol_header() "));

            try!(writeln!(writer, "}} // impl ::Protocol<'a> for {}", struct_name));
        }

        Ok(())
    }
}