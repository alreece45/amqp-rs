// Copyright 2016-17 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;

use common::{Class, ClassField, Domain};
use WriteRust;

impl<'a> WriteRust for EncodableHeaderImplWriter<'a> {
    fn write_rust_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        if self.class.fields().is_empty() {
            return Ok(());
        }

        let lifetimes = if self.class.has_field_lifetimes() { ("<'a>") } else { ("") };

        try!(writeln!(writer, "\nimpl{0} ::Encodable for Header{0} {{", lifetimes));
        try!(self.write_encoded_size(writer));
        try!(self.write_encoded_writer(writer));
        try!(writeln!(writer, "}} // impl Encodable"));

        Ok(())
    }
}

pub struct EncodableHeaderImplWriter<'a> {
    class: &'a Class,
}

impl<'a> EncodableHeaderImplWriter<'a> {
    pub fn new(class: &'a Class) -> Self {
        EncodableHeaderImplWriter {
            class: class
        }
    }

    fn write_bit_fields<W>(&self, writer: &mut W, fields: &mut Vec<&ClassField>) -> io::Result<()>
        where W: io::Write
    {
        let num_padding_bytes = if fields.len() % 8 != 0 { 1 } else { 0 };
        let cap = (fields.len() / 8 + num_padding_bytes) * 8;
        try!(writeln!(writer, "try!(::Encodable::write_encoded_to(&{{"));
        try!(writeln!(writer, "let mut bits = ::bit_vec::BitVec::from_elem({}, false);", cap));

        let mut bit_num = 0;
        for field in fields.drain(..) {
            try!(writeln!(writer, "bits.set({}, self.{});", bit_num, field.var_name()));
            bit_num += 1;
        }
        try!(writeln!(writer, "bits\n}}, writer));"));

        Ok(())
    }

    pub fn write_encoded_size<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        let padding_bytes = if self.class.fields().len() % 8 != 0 { 1 } else { 0 };
        let num_flag_bytes = self.class.fields().len() / 8 + padding_bytes;
        let num_field_bits = self.class.fields().iter()
            .map(|field| field.ty().num_bits_fixed())
            .fold(0, |sum, num_bits| sum + num_bits);

        let static_size_bits = num_flag_bytes * 8 + num_field_bits;
        let static_size = static_size_bits / 8 + if static_size_bits % 8 > 0 { 1 } else { 0 };
        try!(write!(writer, "fn encoded_size(&self) -> usize {{\n{}", static_size));

        for field in self.class.fields() {
            if field.ty().dynamic_bit_method().is_none() {
                continue;
            }

            try!(write!(
                writer,
                " + ::Encodable::encoded_size(&self.{0})",
                field.var_name()
            ));
        }
        try!(writeln!(writer, "\n}} // encoded_size"));

        Ok(())
    }

    fn write_encoded_writer<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        try!(writeln!(writer, "fn write_encoded_to<W>(&self, writer: &mut W) -> ::io::Result<()>"));
        try!(writeln!(writer, "where W: ::io::Write"));
        try!(writeln!(writer, "{{"));

        let mut bit_fields = Vec::with_capacity(8);

        try!(writeln!(writer, "try!(::Encodable::write_encoded_to(&self.flag_bits(), writer));\n"));

        // values
        for field in self.class.fields() {
            if Domain::Bit == *field.ty() {
                bit_fields.push(field);
                continue;
            } else if !bit_fields.is_empty() {
                try!(self.write_bit_fields(writer, &mut bit_fields))
            }

            try!(writeln!(
                writer,
                "try!(::Encodable::write_encoded_to(&self.{}, writer));",
                field.var_name()
            ));
        }

        if !bit_fields.is_empty() {
            try!(self.write_bit_fields(writer, &mut bit_fields));
        }

        try!(writeln!(writer, ""));

        try!(writeln!(writer, "::std::result::Result::Ok(())"));
        try!(writeln!(writer, "}} // fn write_encoded_to()"));

        Ok(())
    }
}
