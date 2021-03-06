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

impl<'a> WriteRust for MethodStructWriter<'a> {
    fn write_rust_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        try!(writeln!(writer, "\n// generated by primalgen::codegen::spec-module::class_mod::method_struct"));
        try!(writeln!(writer, "#[derive(Debug)]"));
        if !self.method.has_usable_fields() {
            try!(writeln!(writer, "pub struct {};", self.method.pascal_case()));
            return Ok(());
        }

        let lifetimes = if self.method.has_lifetimes() { "<'a>" } else { "" };

        try!(writeln!(writer, "pub struct {}{} {{", self.method.pascal_case(), lifetimes));
        for field in self.method.fields() {
            if field.is_reserved() {
                continue;
            }
            try!(write!(writer, "{}: ", field.var_name()));
            if field.ty().is_copy() || field.ty().is_owned() {
                try!(writeln!(writer, "{},", field.ty().owned_type()));
            }
            else {
                try!(writeln!(writer, "{},", field.ty().cow_definition("a")));
            }
        }
        try!(writeln!(writer, "}} // struct {}{}", self.method.pascal_case(), lifetimes));

        Ok(())
    }
}

pub struct MethodStructWriter<'a> {
    method: &'a ClassMethod,
}

impl<'a> MethodStructWriter<'a> {
    pub fn new(method: &'a ClassMethod) -> Self {
        MethodStructWriter {
            method: method,
        }
    }
}
