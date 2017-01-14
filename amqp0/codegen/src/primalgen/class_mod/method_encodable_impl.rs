// Copyright 2016-17 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;

use WriteRust;
use common::ClassMethod;

impl<'a> WriteRust for EncodableMethodImplWriter<'a> {
    fn write_rust_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        let lifetimes = if self.method.has_lifetimes() { ("<'a>") } else { ("") };
        let static_size_bits = self.method.fields().iter()
            .map(|field| field.ty().num_bits_fixed())
            .fold(0, |sum, num_bits| sum + num_bits);

        let static_size = static_size_bits / 8 + if static_size_bits % 8 > 0 { 1 } else { 0 };
        let has_dynamic_field = self.method.fields().iter()
            .any(|field| field.ty().dynamic_bit_method().is_some());

        try!(writeln!(writer, "\nimpl{1} ::Encodable for {0}{1} {{", self.method.pascal_case(), lifetimes));
        try!(writeln!(writer, "fn encoded_size(&self) -> usize {{"));
        if has_dynamic_field {
            try!(writeln!(writer, "["));
            try!(writeln!(writer, "{},", static_size));
            for field in self.method.fields() {
                if field.is_reserved() {
                    continue;
                }

                if field.ty().dynamic_bit_method().is_some() {
                    try!(writeln!(writer, "::Encodable::encoded_size(&self.{}),", field.var_name()));
                }
            }
            try!(writeln!(writer, "].iter().sum()"));
        } else {
            try!(writeln!(writer, "{}", static_size));
        }
        try!(writeln!(writer, "}} // fn encoded_size()"));
        try!(writeln!(writer, "}} // impl Encodable"));

        Ok(())
    }
}

pub struct EncodableMethodImplWriter<'a> {
    method: &'a ClassMethod,
}

impl<'a> EncodableMethodImplWriter<'a> {
    pub fn new(method: &'a ClassMethod) -> Self {
        EncodableMethodImplWriter {
            method: method,
        }
    }
}
