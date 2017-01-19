// Copyright 2016-17 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;

use WriteRust;
use common::{Specs, Spec, Class, ClassMethod};

pub struct CommonImplWriter<'a> {
    specs: &'a Specs<'a>,
    spec: &'a Spec,
    class: &'a Class,
    method: &'a ClassMethod,
}

impl<'a> WriteRust for CommonImplWriter<'a> {
    fn write_rust_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        let spec_method = self.specs.method(self.class.name(), self.method.name()).unwrap();
        let lifetimes = match (spec_method.has_lifetimes(), self.method.has_lifetimes()) {
            (true, true) => ("<'a>", "<'a>", "<'a>"),
            (true, false) => ("<'a>", "<'a>", ""),
            (false, true) => ("<'a>", "", "<'a>"),
            (false, false) => ("", "", ""),
        };
        let section = format!(
            "impl{lifetimes_impl} ::method::{class}::{method}Method{lifetimes_trait} for ::{spec}",
            lifetimes_impl = lifetimes.0,
            lifetimes_trait = lifetimes.1,
            class = self.class.snake_case(),
            method = self.method.pascal_case(),
            spec = self.spec.pascal_case(),
        );

        try!(writeln!(writer, "{} {{", section));
        try!(writeln!(writer, "type Payload = {}{};", self.method.pascal_case(), lifetimes.2));
        try!(writeln!(writer, "}} // {}", section));

        Ok(())
    }
}

impl<'a> CommonImplWriter<'a> {
    pub fn new(specs: &'a Specs<'a>, spec: &'a Spec, class: &'a Class, method: &'a ClassMethod) -> Self {
        CommonImplWriter {
            specs: specs,
            spec: spec,
            class: class,
            method: method,
        }
    }
}
