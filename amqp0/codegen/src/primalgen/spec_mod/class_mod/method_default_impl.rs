// Copyright 2017 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;
use WriteRust;
use common::{ClassMethod, Domain};

pub struct DefaultImplWriter<'a> {
    method: &'a ClassMethod,
}

impl<'a> DefaultImplWriter<'a> {
    pub fn new(method: &'a ClassMethod) -> Self {
        DefaultImplWriter {
            method: method,
        }
    }
}

impl<'a> WriteRust for DefaultImplWriter<'a> {
    fn write_rust_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        let lifetimes = if self.method.has_lifetimes() { "<'a>" } else { "" };
        let pascal_case = self.method.pascal_case();

        let arguments = self.method.fields().iter()
            .filter(|field| !field.is_reserved())
            .map(|field| match *field.ty() {
                Domain::Bit => "false",
                Domain::Octet | Domain::Short | Domain::Long | Domain::LongLong => "0",
                Domain::ShortString => "\"\"",
                Domain::LongString => "&[][..]",
                Domain::Table => "::field::TableEntries::new()",
                Domain::Timestamp => "0",
                Domain::Content => "&[][..]",
            })
            .collect::<Vec<_>>()
            .join(", ");

        try!(writeln!(
            writer,
            "impl{lifetimes} Default for {method}{lifetimes} {{\n\
                fn default() -> Self {{\n\
                    {method}::new({arguments})\n\
                }} // fn default()\n\
            }} // impl Default for {method}",
            lifetimes = lifetimes,
            method = pascal_case,
            arguments = arguments
        ));

        Ok(())
    }
}