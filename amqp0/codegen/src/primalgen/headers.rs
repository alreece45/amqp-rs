// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;
use specs::{Class, ClassField};

use common;
use common::domain::{Domain, DomainMapper};

type Field<'a> = common::Field<'a, ClassField>;

pub struct HeadersStructWriter<'a> {
    fields: Vec<Field<'a>>,
    has_lifetimes: bool,
}

impl<'a> HeadersStructWriter<'a> {
    pub fn new(class: &'a Class,domain_mapper: &DomainMapper) -> Self {
        let fields = class.fields().iter()
            .map(|field| {
                let ty = Domain::new(domain_mapper.map(field.domain()));
                Field::from_amqp0_field(field, ty)
            })
            .collect::<Vec<_>>();

        let has_lifetimes = fields.iter()
            .map(|f| !f.ty().is_copy())
            .any(|is_copy| is_copy);

        HeadersStructWriter {
            fields: fields,
            has_lifetimes: has_lifetimes,
        }
    }

    pub fn write_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        try!(self.write_struct(writer));
        try!(self.write_inherent_impl(writer));
        // try!(self.write_amqp0_payload_impl(writer));
        // try!(self.write_nom_bytes_impl(writer));

        Ok(())
    }

    pub fn write_struct<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        if self.fields.is_empty() {
            try!(writeln!(writer, "pub struct Headers;"));
            return Ok(());
        }

        let lifetimes = if self.has_lifetimes { "<'a>" } else { "" };

        try!(writeln!(writer, "pub struct Headers{} {{", lifetimes));
        for field in &self.fields {
            try!(writeln!(writer, "{}: Option<{}>,", field.var_name(), field.ty().cow_definition("a")));
        }
        try!(writeln!(writer, "}}\n"));
        Ok(())
    }

    pub fn write_inherent_impl<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        if self.fields.is_empty() {
            return Ok(());
        }

        let lifetimes = if self.has_lifetimes { "<'a>" } else { "" };

        try!(writeln!(writer, "impl{} Headers{} {{", lifetimes, lifetimes));
        try!(self.write_getters(writer));
        try!(writeln!(writer, "}}"));

        Ok(())
    }

    pub fn write_getters<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        if self.fields.is_empty() {
            return Ok(());
        }

        try!(writeln!(writer, "impl_properties! {{"));
        for field in &self.fields {
            try!(write!(writer, "({0}, {0}_mut, set_{0}, take_{0}) -> ", field.var_name()));

            let ty = field.ty().borrowed_type();
            try!(match (field.ty().is_copy(), field.ty().is_owned()) {
                (true, _) => writeln!(writer, "Option<{}>,", ty),
                (_, true) => writeln!(writer, "Option<&{}>,", ty),
                _ => writeln!(writer, "Option< Cow<{}> >,", ty)
            });
        }
        try!(writeln!(writer, "}} // impl_properties"));

        Ok(())
    }
}
