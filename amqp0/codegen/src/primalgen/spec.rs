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
use super::{MethodModuleWriter, HeaderStructWriter};

pub struct SpecModuleWriter<'a> {
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

            let property_writer = HeaderStructWriter::new(class, &self.domain_mapper);
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

            try!(writeln!(writer, "\npub enum ClassMethod{} {{", lifetimes));
            for &(ref name, has_lifetimes) in &methods {
                let lifetimes = if has_lifetimes { "<'a>" } else {""};
                try!(writeln!(writer, "{0}({0}{1}),", name, lifetimes));
            }
            try!(writeln!(writer, "}} // enum ClassMethod\n"));

            try!(writeln!(writer, "\nimpl{0} ::ProtocolMethodPayload for ClassMethod{0} {{", lifetimes));

            // ProtocolMethod::class_id
            try!(writeln!(writer, "\nfn class_id(&self) -> u16 {{"));
            try!(writeln!(writer, "match *self {{"));
            for &(ref name, _) in &methods {
                try!(writeln!(writer, "ClassMethod::{}(ref method) => ::ProtocolMethodPayload::class_id(method),", name));
            }
            try!(writeln!(writer, "\n}} // match *self"));
            try!(writeln!(writer, "\n}} // fn class_id"));

            // ProtocolMethod::method_id
            try!(writeln!(writer, "\nfn method_id(&self) -> u16 {{"));
            try!(writeln!(writer, "match *self {{"));
            for &(ref name, _) in &methods {
                try!(writeln!(writer, "ClassMethod::{}(ref method) => ::ProtocolMethodPayload::method_id(method),", name));
            }
            try!(writeln!(writer, "\n}} // match *self"));
            try!(writeln!(writer, "\n}} // fn method_id"));

            // ProtocolMethod::method_id
            try!(writeln!(writer, "\nfn payload_size(&self) -> usize {{"));
            try!(writeln!(writer, "match *self {{"));
            for &(ref name, _) in &methods {
                try!(writeln!(writer, "ClassMethod::{}(ref method) => ::ProtocolMethodPayload::payload_size(method),", name));
            }
            try!(writeln!(writer, "\n}} // match *self"));
            try!(writeln!(writer, "\n}} // fn method_id"));

            try!(writeln!(writer, "}} // impl ::ProtocolMethodPayload for ClassMethod\n"));

            try!(writeln!(writer, "}} // mod {}\n", module_name));

            let has_methods = !class.methods().is_empty();
            let pascal_case = class.name().to_pascal_case();
            class_methods.push((pascal_case, module_name, has_lifetimes, has_methods));
        }

        let has_lifetimes = class_methods.iter().map(|c| c.3).any(|c| c);
        let lifetimes = if has_lifetimes { "<'a>" } else { "" };

        try!(writeln!(writer, "\n// Class methods"));
        for &(ref pascal_case, ref module, _, has_methods) in &class_methods {
            if has_methods {
                try!(writeln!(writer, "pub use self::{}::ClassMethod as {}Method;", module, pascal_case));
            }
        }

        try!(writeln!(writer, "\n// Class headers"));
        for &(ref pascal_case, ref module, _, has_methods) in &class_methods {
            if has_methods {
                try!(writeln!(writer, "pub use self::{}::Header as {}Header;", module, pascal_case));
            }
        }

        try!(writeln!(writer, "\npub enum SpecMethod{} {{", lifetimes));
        for &(ref pascal_case, _, has_lifetimes, has_methods) in &class_methods {
            if has_methods {
                let lifetimes = if has_lifetimes { "<'a>" } else { "" };
                try!(writeln!(writer, "{0}({0}Method{1}),", pascal_case, lifetimes));
            }
            else {
                try!(writeln!(writer, "{},", pascal_case));
            }
        }
        try!(writeln!(writer, "}} // enum SpecMethod"));

        // ProtocolMethod
        try!(writeln!(writer, "\nimpl<'a> ::ProtocolMethodPayload for SpecMethod<'a> {{"));

        // ProtocolMethod::class_id
        try!(writeln!(writer, "\nfn class_id(&self) -> u16 {{"));
        try!(writeln!(writer, "match *self {{"));
        for &(ref pascal_case, _, _, _) in &class_methods {
            try!(writeln!(writer, "SpecMethod::{}(ref method) => ::ProtocolMethodPayload::class_id(method),", pascal_case));
        }
        try!(writeln!(writer, "\n}} // match *self"));
        try!(writeln!(writer, "\n}} // fn class_id"));

        // ProtocolMethod::method_id
        try!(writeln!(writer, "\nfn method_id(&self) -> u16 {{"));
        try!(writeln!(writer, "match *self {{"));
        for &(ref pascal_case, _, _, _) in &class_methods {
            try!(writeln!(writer, "SpecMethod::{}(ref method) => ::ProtocolMethodPayload::method_id(method),", pascal_case));
        }
        try!(writeln!(writer, "\n}} // match *self"));
        try!(writeln!(writer, "\n}} // fn method_id"));

        // ProtocolMethod::method_id
        try!(writeln!(writer, "\nfn payload_size(&self) -> usize {{"));
        try!(writeln!(writer, "match *self {{"));
        for &(ref pascal_case, _, _, _) in &class_methods {
            try!(writeln!(writer, "SpecMethod::{}(ref method) => ::ProtocolMethodPayload::payload_size(method),", pascal_case));
        }
        try!(writeln!(writer, "\n}} // match *self"));
        try!(writeln!(writer, "\n}} // fn method_id"));

        try!(writeln!(writer, "\n}} // impl ProtocolMethodPayload for SpecMethod"));


        // ProtocolMethod
        try!(writeln!(writer, "\nimpl<'a> ::ProtocolMethod<'a> for SpecMethod<'a> {{\n\
            type Start = connection::Start<'a>;\n\

            fn as_start(&self) -> Option<&Self::Start> {{\n\
                if let SpecMethod::Connection(ConnectionMethod::Start(ref start)) = *self {{\n\
                    Some(start)\n\
                }}\n\
                else {{\n\
                    None\n\
                }} // if let Some(Method::Start(start)) == *self\n\
            }} // fn as_start()\n\
        }} // impl ::ProtocolMethod"));

        Ok(())
    }
}

impl<'a> SpecModuleWriter<'a> {
    pub fn new(spec: &'a Spec) -> Self {
        let mod_name = spec_mod_name(spec);

        SpecModuleWriter {
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

    pub fn write_method_enum<W>(&self, _: &mut W) -> io::Result<()>
        where W: io::Write
    {
        Ok(())
    }

    pub fn write_header_enum<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        try!(writeln!(writer, "\npub enum SpecHeader<'a> {{"));
        for class in self.spec.classes().values() {
            let pascal_case = class.name().to_pascal_case();
            if class.fields().is_empty() {
                try!(writeln!(writer, "{},", pascal_case));
            }
            else {
                let snake_case = class.name().to_snake_case();
                try!(writeln!(writer, "{}({}::Header<'a>),", pascal_case, snake_case));
            }
        }
        try!(writeln!(writer, "}} // enum SpecHeader"));

        Ok(())
    }

    pub fn write_frame_enum<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        let frame_types = self.spec.frame_types().keys()
            .map(|name| {
                let prefix_end = if name.starts_with("frame-") { 6 } else { 0 };
                (&name[prefix_end..]).to_pascal_case()
            })
            .collect::<Vec<_>>();

        if frame_types.is_empty() {
            return Ok(())
        }

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

        // struct FramePayload
        try!(writeln!(writer, "\npub enum FramePayload<'a> {{"));
        for frame_type in &frame_types {
            try!(match frame_type.as_str() {
                "Method" | "OobMethod" => writeln!(writer, "{}(SpecMethod<'a>),", frame_type),
                "Header" | "OobHeader" => writeln!(writer, "{}(SpecHeader<'a>),", frame_type),
                "Body" | "OobBody" | "Trace" => writeln!(writer, "{}(&'a [u8]),", frame_type),
                _ => writeln!(writer, "{},", frame_type),
            });

        }
        try!(writeln!(writer, "}} // enum FramePayload"));

        // impl ProtocolFramePayload
        try!(writeln!(writer, "impl<'a> ::ProtocolFramePayload<'a> for FramePayload<'a> {{"));
        try!(writeln!(writer, "type Method = SpecMethod<'a>;"));

        // fn as_method
        try!(writeln!(writer, "fn as_method(&self) -> Option<&SpecMethod<'a>> {{"));
        try!(writeln!(writer, "if let FramePayload::Method(ref method) = *self {{"));
        try!(writeln!(writer, "Some(method)"));
        try!(writeln!(writer, "}} else {{"));
        try!(writeln!(writer, "None"));
        try!(writeln!(writer, "}} // if "));
        try!(writeln!(writer, "}} // fn as_method()"));

        try!(writeln!(writer, "}} // impl ::ProtocolFramePayload for FramePayload"));

        for frame_type in &frame_types {
            match frame_type.as_str() {
                "Heartbeat" | "Body" | "OobBody" | "OobHeader" | "OobMethod" => break,
                _ => (),
            }

            let struct_name = if frame_type == "Method" { "SpecMethod" } else { frame_type };
            // impl From<..> for FramePayload
            try!(writeln!(writer, "impl<'a> From<{0}<'a>> for FramePayload<'a> {{\n\
                fn from(payload: {0}<'a>) -> {0}<'a> {{\n\
                        FramePayload::{1}(payload)\n\
                    }}\n\
                }} // impl From<{0}> for FramePayload<'a>\n",
                struct_name,
                frame_type
            ));
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