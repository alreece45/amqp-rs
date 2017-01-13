// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod class_mod;
mod frame_payload_enum;
mod header_enum;
mod method_enum;

use std::iter::ExactSizeIterator;
use std::io;

use WriteRust;
use common::Spec;

use self::class_mod::ClassModuleWriter;
use self::frame_payload_enum::FramePayloadEnumWriter;
use self::header_enum::HeaderEnumWriter;
use self::method_enum::MethodEnumWriter;

pub struct RootModuleWriter<'a> {
    spec: &'a Spec,
}

impl<'a> WriteRust for RootModuleWriter<'a> {
    fn write_rust_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        if self.spec.classes().len() == 0 {
            return Ok(());
        }

        try!(writeln!(writer, "#![allow(too_many_arguments)]\n"));

        try!(self.write_class_constants(writer));
        try!(self.write_method_constants(writer));

        let header_enum = HeaderEnumWriter::new(&self.spec);
        try!(header_enum.write_rust_to(writer));
        try!(self.write_frame_struct(writer));

        let frame_payload_enum = FramePayloadEnumWriter::new(&self.spec);
        try!(frame_payload_enum.write_rust_to(writer));

        try!(writeln!(writer, "\n// Class Modules"));
        for class in self.spec.classes() {
            let module_writer = ClassModuleWriter::new(class);
            try!(module_writer.write_rust_to(writer));
        }

        // aliases
        try!(writeln!(writer, "\n// Class methods"));
        for class in self.spec.classes() {
            try!(writeln!(writer, "pub use self::{}::ClassMethod as {}Method;", class.snake_case(), class.pascal_case()));
        }

        try!(writeln!(writer, "\n// Class headers"));
        for class in self.spec.classes() {
            if class.fields().is_empty() {
                continue;
            }
            try!(writeln!(writer, "pub use self::{}::Header as {}Header;", class.snake_case(), class.pascal_case()));
        }

        let method_enum = MethodEnumWriter::new(&self.spec);
        try!(method_enum.write_rust_to(writer));

        Ok(())
    }
}

impl<'a> RootModuleWriter<'a> {
    pub fn new(spec: &'a Spec) -> Self {
        RootModuleWriter {
            spec: spec,
        }
    }

    pub fn mod_name(&self) -> &str {
        self.spec.mod_name()
    }

    pub fn write_class_constants<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        try!(writeln!(writer, "// Class Constants"));
        try!(writeln!(writer, "// Generated by primalgen::codegen::spec_module::SpecModuleWriter"));
        for (_, class_names) in self.spec.class_indexes() {
            for class_name in class_names {
                let class = self.spec.class(class_name).unwrap();
                let constant_class = class.constant_case();
                try!(writeln!(writer, "pub const CLASS_{}: u16 = {};", constant_class, class.index()));
            }
        }

        Ok(())
    }

    /// Write the correct frame strucutre
    /// 0.8.x had a "cycle" in each frame, 0.9.x doesn't
    pub fn write_frame_struct<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        let version = self.spec.version();

        let (cycle_prop, cycle_getter, cycle_arg, cycle_create) = if version.minor() == 8 {
            (
                "cycle: u8,\n",
                "\n\
                    pub fn cycle(&self) -> u8 {{\n\
                        self.cycle\n\
                    }} // fn cycle()\n\
                 \n",
                "cycle: u8,\n",
                "cycle: cycle,\n",
            )
        } else {
            ("", "", "", "")
        };

        // struct Frame
        try!(writeln!(writer, "\n\
                // Generated by primalgen::codegen::spec_module::SpecModuleWriter
                #[derive(Debug)]
                pub struct Frame<'a> {{\n\
                    channel: u16,\n\
                    {}\
                    payload: FramePayload<'a>,\n\
                }} // struct Frame\n\
                \n\
                impl<'a> Frame<'a> {{\n\
                    pub fn new<P>(channel: u16, {}payload: P) -> Self\n\
                        where P: Into<FramePayload<'a>>\n\
                    {{\n\
                        Frame {{\n\
                            channel: channel,\n\
                            {}\
                            payload: payload.into(),\n\
                        }} // Frame\n\
                    }} // fn new()\n\
                    \n\
                    pub fn channel(&self) -> u16 {{\n\
                        self.channel\n\
                    }} // fn channel()\n\
                    \n\
                    pub fn payload(&self) -> &FramePayload<'a> {{\n\
                        &self.payload\n\
                    }} // fn payload()\n\
                    {}\n\
                 }} // impl Frame<'a>
                    ",
                      cycle_prop, // struct Frame<'a>...
                      cycle_arg, // fn new()...
                      cycle_create, // Frame { cycle: ... }
                      cycle_getter, // fn cycle() -> u8
        ));

        Ok(())
    }

    pub fn write_method_constants<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        try!(writeln!(writer, "\n// Class Methods"));
        try!(write!(writer, "// Generated by codegen::spec_module::SpecModuleWriter"));
        for class in self.spec.classes() {
            try!(writeln!(writer, ""));
            for (_, method_names) in class.method_indexes() {
                for method_name in method_names {
                    let method = class.method(method_name).unwrap();
                    try!(writeln!(
                        writer,
                        "pub const METHOD_{}_{}: u16 = {};",
                        class.constant_case(),
                        method.constant_case(),
                        method.index()
                    ));
                }
            }
        }

        Ok(())
    }
}