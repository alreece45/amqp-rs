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

pub struct MethodFromImplsWriter<'a> {
    class: &'a Class,
    method: &'a ClassMethod,
}

impl<'a> MethodFromImplsWriter<'a> {
    pub fn new(class: &'a Class, method: &'a ClassMethod) -> Self {
        MethodFromImplsWriter {
            class: class,
            method: method,
        }
    }
}

impl<'a> WriteRust for MethodFromImplsWriter<'a> {
    fn write_rust_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        let lifetimes = match (self.class.has_method_lifetimes(), self.method.has_lifetimes()) {
            (true, true) => ("<'a>", "<'a>", "<'a>", "<'a>"),
            (true, false) => ("<'a>", "<'static>", "<'a>", ""),
            (false, false) => ("", "<'static>", "", ""),
            (false, true) => unreachable!()
        };

        // ClassMethod
        try!(writeln!(
            writer,
            "impl{lifetimes} \
                From<{method}{method_lifetimes}> \
                for ClassMethod{class_lifetimes}\
            {{\n\
                fn from(from: {method}{method_lifetimes}) -> Self {{\n\
                   ClassMethod::{method}(from)\n\
                }} // fn from()\n\
            }} // impl From<{method}{method_lifetimes}> for ClassMethod\n\
            \n\
            impl{method_lifetimes} \
                From<{method}{method_lifetimes}> \
                for super::SpecMethod{spec_lifetimes}\
            {{\n\
                fn from(from: {method}{method_lifetimes}) -> Self {{\n\
                   super::SpecMethod::{class}(from.into())\n\
                }} // fn default()\n\
            }} // impl From<{method}{method_lifetimes}> for ::super::SpecMethod",

            class = self.class.pascal_case(),
            method = self.method.pascal_case(),
            lifetimes = lifetimes.0,
            spec_lifetimes = lifetimes.1,
            class_lifetimes = lifetimes.2,
            method_lifetimes = lifetimes.3,
        ));

        Ok(())
    }
}