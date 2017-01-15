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

pub struct HeaderEnumWriter<'a> {
    spec: &'a Spec,
    has_lifetimes: bool,
}

impl<'a> WriteRust for HeaderEnumWriter<'a> {
    fn write_rust_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        let lifetimes = if self.has_lifetimes {"<'a>"} else {""};
        try!(writeln!(writer, "#[derive(Debug)]"));
        try!(writeln!(writer, "pub enum SpecProperties{} {{", lifetimes));
        for class in self.spec.classes() {
            let pascal_case = class.pascal_case();
            if class.fields().is_empty() {
                try!(writeln!(writer, "{},", pascal_case));
            }
            else {
                let snake_case = class.snake_case();
                try!(writeln!(writer, "{}({}::Properties<'a>),", pascal_case, snake_case));
            }
        }
        try!(writeln!(writer, "}} // enum SpecProperties"));

        try!(self.write_encodable_impl(writer));

        Ok(())
    }
}

impl<'a> HeaderEnumWriter<'a> {
    pub fn new(spec: &'a Spec) -> Self {
        let has_lifetimes = spec.classes()
            .any(|class| class.has_field_lifetimes());

        HeaderEnumWriter {
            spec: spec,
            has_lifetimes: has_lifetimes
        }
    }

    fn write_encodable_impl<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        try!(writeln!(writer, "\nimpl<'a> ::Encodable for SpecProperties<'a> {{"));
        try!(writeln!(writer, "\nfn encoded_size(&self) -> usize {{"));
        try!(writeln!(writer, "match *self {{"));
        for class in self.spec.classes() {
            let matcher = if class.fields().is_empty() {
                ("", "0")
            }
            else {
                ("(ref properties)", "::Encodable::encoded_size(properties)")
            };
            try!(writeln!(writer, "SpecProperties::{}{} => {},", class.pascal_case(), matcher.0, matcher.1));
        }
        try!(writeln!(writer, "\n}} // match *self"));
        try!(writeln!(writer, "\n}} // fn encoded_size"));

        try!(writeln!(writer, "fn write_encoded_to<W>(&self, _: &mut W) -> ::std::io::Result<()>"));
        try!(writeln!(writer, "where W: ::std::io::Write"));
        try!(writeln!(writer, "{{"));
        try!(writeln!(writer, "unimplemented!()"));
        try!(writeln!(writer, "}} // fn write_encoded_to()"));

        try!(writeln!(writer, "\n}} // F::Encodable for SpecMethod"));

        Ok(())
    }
}