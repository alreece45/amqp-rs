// Copyright 2017 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;

use WriteRust;
use common::{Class, ClassMethod};

pub struct MethodContentImplWriter<'a> {
    class: &'a Class,
    method: &'a ClassMethod,
}

impl<'a> WriteRust for MethodContentImplWriter<'a> {
    fn write_rust_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        if !self.method.has_content() {
            return Ok(());
        }

        let lifetimes = match (self.method.has_lifetimes(), self.class.has_field_lifetimes()) {
            (false, false) => ("", "", ""),
            (true, false) => ("<'a>", "<'a>", ""),
            (false, true) => ("<'a>", "", "<'a>"),
            (true, true) => ("<'a>", "<'a>", "<'a>"),
        };

        write!(
            writer,
            "impl<'a> ::Content<'a> for {method}{method_lifetimes} {{\n\
                type Headers = {headers}{header_lifetimes};\n\
            }}",
            method = self.method.pascal_case(),
            method_lifetimes = lifetimes.1,
            headers = if self.class.fields().is_empty() { "()" } else { "Properties" },
            header_lifetimes = lifetimes.2,
        )
    }
}

impl<'a> MethodContentImplWriter<'a> {
    pub fn new(class: &'a Class, method: &'a ClassMethod) -> Self {
        MethodContentImplWriter {
            class: class,
            method: method,
        }
    }
}
