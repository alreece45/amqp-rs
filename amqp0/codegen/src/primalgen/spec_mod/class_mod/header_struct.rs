// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;
use common::Class;

pub struct HeaderStructWriter<'a> {
    class: &'a Class,
}

impl<'a> HeaderStructWriter<'a> {
    pub fn new(class: &'a Class) -> Self {
        HeaderStructWriter {
            class: class
        }
    }

    pub fn write_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        if self.class.fields().is_empty() {
            return Ok(());
        }

        try!(writeln!(writer, "\n// Generated by primalgen::spec::frame_payload_enum::ClassEnumWriter"));
        try!(self.write_struct(writer));
        try!(self.write_inherent_impl(writer));
        try!(self.write_encodable_impl(writer));

        Ok(())
    }

    fn write_struct<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        try!(writeln!(writer, "#[derive(Debug)]"));

        let lifetimes = if self.class.has_field_lifetimes() { "<'a>" } else { "" };

        try!(writeln!(writer, "pub struct Header{} {{", lifetimes));
        for field in self.class.fields() {
            try!(writeln!(writer, "{}: Option<{}>,", field.var_name(), field.ty().cow_definition("a")));
        }
        try!(writeln!(writer, "}} // struct Header"));
        Ok(())
    }

    fn write_inherent_impl<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        let lifetimes = if self.class.has_field_lifetimes() { "<'a>" } else { "" };

        try!(writeln!(writer, "\nimpl{0} Header{0} {{", lifetimes));
        try!(self.write_getters(writer));
        try!(writeln!(writer, "}} // impl Headers"));

        Ok(())
    }

    fn write_encodable_impl<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        let lifetimes = if self.class.has_field_lifetimes() { "<'a>" } else { "" };

        try!(writeln!(writer, "\nimpl{0} ::Encodable for Header{0} {{", lifetimes));
        try!(writeln!(writer, "fn encoded_size(&self) -> usize {{"));
        try!(writeln!(writer, "unimplemented!()"));
        try!(writeln!(writer, "}} // fn encoded_size"));
        try!(writeln!(writer, "}} // impl ::Encodable for Header{0}", lifetimes));

        Ok(())
    }

    fn write_getters<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        if self.class.fields().is_empty() {
            return Ok(());
        }

        try!(writeln!(writer, "impl_properties! {{"));
        for field in self.class.fields() {
            try!(write!(writer, "({0}, {0}_mut, set_{0}, take_{0}) -> ", field.var_name()));

            let ty = field.ty().borrowed_type();
            try!(match (field.ty().is_copy(), field.ty().is_owned()) {
                (true, _) => writeln!(writer, "Option< {} >,", ty),
                (_, true) => writeln!(writer, "Option< &{} >,", ty),
                _ => writeln!(writer, "Option< Cow<{}> >,", ty)
            });
        }
        try!(writeln!(writer, "}} // impl_properties"));

        Ok(())
    }
}