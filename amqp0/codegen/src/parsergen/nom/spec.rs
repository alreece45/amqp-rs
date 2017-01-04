// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;

use inflections::Inflect;

use CodeGenerator;
use common::frame_type_name;
use common::{Spec, Class};
use super::MethodModuleWriter;

pub struct SpecModuleWriter<'a> {
    spec: &'a Spec,
}

impl<'a> CodeGenerator for SpecModuleWriter<'a> {
    fn write_rust_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        if self.spec.classes().is_empty() {
            return Ok(());
        }

        // some nom parsers have an unused variable e
        try!(writeln!(writer, "#![allow(unused_variables)]"));
        try!(writeln!(writer, "\nuse nom::{{IResult, be_u8, be_u16, be_u32, be_u64}};\n"));

        for class in self.spec.classes() {
            try!(write!(writer, "// Class {}", class.name()));
            for method in class.methods() {
                let method_writer = MethodModuleWriter::new(&self.spec, &class, method);
                try!(method_writer.write_rust_to(writer));
            }

            //try!(self.write_class_header_parser(class, writer));
            try!(self.write_class_method_parser(class, writer));
        }

        //try!(self.write_spec_header_parser(writer));
        try!(self.write_spec_method_parser(writer));
        try!(self.write_frame_parser(writer));

        Ok(())
    }
}

impl<'a> SpecModuleWriter<'a> {
    pub fn new(spec: &'a Spec) -> Self {
        SpecModuleWriter {
            spec: spec,
        }
    }

    fn write_frame_parser<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        try!(writeln!(writer, "\n\
            impl<'a> ::NomBytes<'a> for ::primitives::{}::Frame<'a> {{\n\
                type Output = Self;\n\
                fn nom_bytes<'pool, P>(input: &'a [u8], pool: &'pool mut P)  -> IResult<&'a [u8], Self>\n\
                    where P: ::pool::ParserPool\n\
                {{\n\
                    switch!(input, be_u8,\n",
              self.spec.mod_name()
        ));

        let (cycle_parse, cycle_arg) = if self.spec.version().minor() == 8 {
            ("\ncycle: be_u8 >>", "cycle, ")
        } else {
            ("", "")
        };

        let mut has_parent = false;
        for ty in self.spec.frame_types().values() {
            let name = frame_type_name(ty.name());

            // FIXME: implement header parsing
            if name == "Header" || name == "OobHeader" {
                continue;
            }
            if has_parent {
                try!(writeln!(writer, " | // do_parse"))
            }
            else {
                has_parent = true;
            }

            try!(write!(writer, "{} => ", ty.value()));
            try!(writeln!(writer, "do_parse!({}", cycle_parse));

            match name.as_str() {
                "Heartbeat" => {
                    try!(write!(writer,
                                "payload: value!(\n\
                                    ::primitives::{0}::FramePayload::Heartbeat,\n\
                                    tag!(b\"\\x00\\x00\\xCE\")\n\
                                ) >>\
                                channel: value!(0) >>",
                        self.spec.mod_name()
                    ));
                },
                enum_name => {
                    try!(write!(writer, "\
                            channel: be_u16 >>\n\
                            payload: "
                    ));

                    let struct_name = match enum_name {
                        "Method" | "OobMethod" => Some("SpecMethod"),
                        "Header" | "OobHeader" => Some("SpecHeader"),
                        _ => None,
                    };

                    if let Some(struct_name) = struct_name {
                        try!(writeln!(writer,
                                      "map!(\n\
                                          length_value!(\n\
                                              be_u32,\n\
                                              call!(<::primitives::{0}::{1} as ::NomBytes>::nom_bytes, pool)\n\
                                          ),\n\
                                          ::primitives::{0}::FramePayload::{2}\n\
                                      ) >> // map",
                              self.spec.mod_name(),
                              struct_name,
                              enum_name
                        ));
                    }
                        else {
                            try!(writeln!(
                                writer,
                                "map!(length_bytes!(be_u32), ::primitives::{}::FramePayload::{}) >>",
                                self.spec.mod_name(),
                                enum_name
                            ));
                        }
                }
            }

            try!(writeln!(
                writer,
                "(::primitives::{}::Frame::new(channel, {}payload))\n",
                self.spec.mod_name(),
                cycle_arg,
            ));
            try!(write!(writer, ")"));
        }

        try!(writeln!(writer, " // do_parse"));
        try!(writeln!(writer, ") // switch!"));
        try!(writeln!(writer, "}} // fn nom_bytes"));
        try!(writeln!(writer, "}} // impl NomBytes for ::primitives::{}::Frame<'a>", self.spec.mod_name()));

        Ok(())
    }

    fn write_spec_header_parser<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        try!(writeln!(writer, "\n\
            impl<'a> ::NomBytes<'a> for ::primitives::{}::SpecHeader<'a> {{\n\
                type Output = Self;\n\
                fn nom_bytes<'pool, P>(input: &'a [u8], pool: &'pool mut P)  -> IResult<&'a [u8], Self>\n\
                    where P: ::pool::ParserPool\n\
                {{\n\
                    switch!(input, be_u16,\n",
              self.spec.mod_name()
        ));

        let mut has_parent = false;
        for class in self.spec.classes() {
            if has_parent {
                try!(writeln!(writer, " | // map"))
            }
                else {
                    has_parent = true;
                }

            try!(write!(
                writer,
                "{0} => map!(\n\
                    call!(<::primitives::{1}::{2}Header as ::NomBytes>::nom_bytes, pool),\n\
                    ::primitives::{1}::SpecHeader::{2}\n\
                )",
                class.index(),
                self.spec.mod_name(),
                class.pascal_case(),
            ));
        }
        try!(writeln!(writer, " // map!"));
        try!(writeln!(writer, ") // switch!"));
        try!(writeln!(writer, "}} // fn nom_bytes"));
        try!(writeln!(writer, "}} // impl ::NomBytes for ::primitives::{}::SpecHeader<'a>", self.spec.mod_name()));

        Ok(())
    }

    fn write_spec_method_parser<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        try!(writeln!(writer, "\n\
            impl<'a> ::NomBytes<'a> for ::primitives::{}::SpecMethod<'a> {{\n\
                type Output = Self;\n\
                fn nom_bytes<'pool, P>(input: &'a [u8], pool: &'pool mut P)  -> IResult<&'a [u8], Self>\n\
                    where P: ::pool::ParserPool\n\
                {{\n\
                    switch!(input, be_u16,\n",
            self.spec.mod_name()
        ));

        let mut has_parent = false;
        for class in self.spec.classes() {
            if has_parent {
                try!(writeln!(writer, " | // map"))
            }
            else {
                has_parent = true;
            }

            try!(write!(
                writer,
                "{0} => map!(\n\
                    call!(<::primitives::{1}::{2}Method as ::NomBytes>::nom_bytes, pool),\n\
                    ::primitives::{1}::SpecMethod::{2}\n\
                )",
                class.index(),
                self.spec.mod_name(),
                class.name().to_pascal_case(),
            ));
        }
        try!(writeln!(writer, " // map!"));
        try!(writeln!(writer, ") // switch!"));
        try!(writeln!(writer, "}} // fn nom_bytes"));
        try!(writeln!(writer, "}} // impl ::NomBytes for ::primitives::{}::SpecMethod<'a>", self.spec.mod_name()));

        Ok(())
    }

    fn write_class_header_parser<W>(&self, class: &Class, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        let domain_mapper = self.spec.domain_mapper();
        let has_lifetimes = class.fields().iter()
            .map(|field| domain_mapper.map(field.domain()))
            .any(|domain| !domain.is_copy());

        let lifetimes = if has_lifetimes { "<'a>" } else { "" };

        let class_mod_name = class.name().to_snake_case();
        try!(writeln!(writer, "\n\
            impl<'a> ::NomBytes<'a> for ::primitives::{}::{}::Header{} {{\n\
                type Output = Self;\n\
                fn nom_bytes<'pool, P>(input: &'a [u8], pool: &'pool mut P)  -> IResult<&'a [u8], Self>\n\
                    where P: ::pool::ParserPool\n\
                {{\n\
                    switch!(input, be_u16,\n",
                      self.spec.mod_name(),
                      class_mod_name,
                      lifetimes
        ));

        let mut has_parent = false;
        for method in class.methods() {
            if has_parent {
                try!(writeln!(writer, " | // map"))
            }
                else {
                    has_parent = true;
                }

            try!(write!(
                writer,
                "{0} => map!(\n\
                    call!(<::primitives::{1}::{2}::{3} as ::NomBytes>::nom_bytes, pool),\n\
                    ::primitives::{1}::{2}::Header::{3}\n\
                )",
                method.index(),
                self.spec.mod_name(),
                class_mod_name,
                method.name().to_pascal_case(),
            ));
        }
        try!(writeln!(writer, " // map!"));
        try!(writeln!(writer, ") // switch!"));
        try!(writeln!(writer, "}} // fn nom_bytes"));
        try!(writeln!(
            writer,
            "}} // impl ::NomBytes for ::primitives::{}::{}::Header<'a>",
            self.spec.mod_name(),
            class_mod_name
        ));

        Ok(())
    }

    fn write_class_method_parser<W>(&self, class: &Class, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        let domain_mapper = self.spec.domain_mapper();
        let has_lifetimes = class.methods().iter()
            .flat_map(|method| method.fields())
            .filter(|field| !field.is_reserved())
            .map(|field| domain_mapper.map(field.domain()))
            .any(|domain| !domain.is_copy());

        let lifetimes = if has_lifetimes { "<'a>" } else { "" };

        let class_mod_name = class.name().to_snake_case();
        try!(writeln!(writer, "\n\
            impl<'a> ::NomBytes<'a> for ::primitives::{}::{}::ClassMethod{} {{\n\
                type Output = Self;\n\
                fn nom_bytes<'pool, P>(input: &'a [u8], pool: &'pool mut P)  -> IResult<&'a [u8], Self>\n\
                    where P: ::pool::ParserPool\n\
                {{\n\
                    switch!(input, be_u16,\n",
            self.spec.mod_name(),
            class_mod_name,
            lifetimes
        ));

        let mut has_parent = false;
        for method in class.methods() {
            if has_parent {
                try!(writeln!(writer, " | // map"))
            }
            else {
                has_parent = true;
            }

            try!(write!(
                writer,
                "{0} => map!(\n\
                    call!(<::primitives::{1}::{2}::{3} as ::NomBytes>::nom_bytes, pool),\n\
                    ::primitives::{1}::{2}::ClassMethod::{3}\n\
                )",
                method.index(),
                self.spec.mod_name(),
                class_mod_name,
                method.name().to_pascal_case(),
            ));
        }
        try!(writeln!(writer, " // map!"));
        try!(writeln!(writer, ") // switch!"));
        try!(writeln!(writer, "}} // fn nom_bytes"));
        try!(writeln!(
            writer,
            "}} // impl ::NomBytes for ::primitives::{}::{}::SpecMethod<'a>",
            self.spec.mod_name(),
            class_mod_name
        ));

        Ok(())
    }
}