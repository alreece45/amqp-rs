// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;

use WriteRust;
use common::Spec;

pub struct MethodEnumWriter<'a> {
    spec: &'a Spec,
    has_lifetimes: bool
}

impl<'a> WriteRust for MethodEnumWriter<'a> {
    fn write_rust_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        let lifetimes = if self.has_lifetimes { "<'a>" } else { "" };
        try!(writeln!(writer, "#[derive(Debug)]"));
        try!(writeln!(writer, "pub enum SpecMethod{} {{", lifetimes));
        for class in self.spec.classes() {
            if class.methods().len() == 0 {
                try!(writeln!(writer, "{},", class.pascal_case()));
            }
            else {
                let has_lifetimes = class.methods().any(|m| m.has_lifetimes());
                let lifetimes = if has_lifetimes { "<'a>" } else { "" };
                try!(writeln!(writer, "{0}({0}Method{1}),", class.pascal_case(), lifetimes));
            }
        }
        try!(writeln!(writer, "}} // enum SpecMethod"));

        try!(self.write_encodable_impl(writer));
        try!(self.write_payload_method_payload_impl(writer));

        Ok(())
    }
}

impl<'a> MethodEnumWriter<'a> {
    pub fn new(spec: &'a Spec) -> Self {
        let has_lifetimes = spec.classes()
            .any(|class| class.has_method_lifetimes());

        MethodEnumWriter {
            spec: spec,
            has_lifetimes: has_lifetimes,
        }
    }

    fn write_encodable_impl<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        let lifetimes = if self.has_lifetimes { "<'a>" } else { "" };
        try!(writeln!(writer, "\nimpl{0} ::Encodable for SpecMethod{0} {{", lifetimes));
        try!(writeln!(writer, "\nfn encoded_size(&self) -> usize {{"));
        try!(writeln!(writer, "match *self {{"));
        for class in self.spec.classes() {
            try!(writeln!(writer, "SpecMethod::{}(ref method) => ::Encodable::encoded_size(method),", class.pascal_case()));
        }
        try!(writeln!(writer, "\n}} // match *self"));
        try!(writeln!(writer, "\n}} // fn encoded_size"));

        try!(writeln!(writer, "fn write_encoded_to<W>(&self, _: &mut W) -> ::std::io::Result<()>"));
        try!(writeln!(writer, "where W: ::std::io::Write"));
        try!(writeln!(writer, "{{"));
        try!(writeln!(writer, "unimplemented!()"));
        try!(writeln!(writer, "}} // fn write_encoded_to()"));

        try!(writeln!(writer, "\n}} // impl ::Encodable for SpecMethod{0}", lifetimes));
        Ok(())
    }

    fn write_payload_method_payload_impl<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        let methods = [
            ("class", "::Class"),
            ("class_id", "u16"),
            ("class_name", "&'static str"),
            ("method_id", "u16"),
            ("method_name", "&'static str"),
        ];

        try!(writeln!(writer, "\nimpl<'a> ::ProtocolMethodPayload for SpecMethod<'a> {{"));

        for &(method_name, method_return) in &methods {
            try!(writeln!(writer, "\nfn {}(&self) -> {} {{", method_name, method_return));
            try!(writeln!(writer, "match *self {{"));
            for class in self.spec.classes() {
                try!(writeln!(
                    writer,
                    "SpecMethod::{}(ref method) => ::ProtocolMethodPayload::{}(method),",
                    class.pascal_case(),
                    method_name
                ));
            }
            try!(writeln!(writer, "\n}} // match *self"));
            try!(writeln!(writer, "\n}} // fn {}", method_name));
        }

        try!(writeln!(writer, "\n}} // impl ProtocolMethodPayload for SpecMethod"));

        Ok(())
    }

    fn write_protocol_method_impl<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
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