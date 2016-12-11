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

pub struct PropertiesStructWriter<'a> {
    fields: Vec<Field<'a>>,
    has_lifetimes: bool,
}

impl<'a> PropertiesStructWriter<'a> {
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

        PropertiesStructWriter {
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
            try!(writeln!(writer, "pub struct Properties;"));
            return Ok(());
        }

        let lifetimes = if self.has_lifetimes { "<'a>" } else { "" };

        try!(writeln!(writer, "pub struct Properties{} {{", lifetimes));
        for field in &self.fields {
            try!(writeln!(writer, "{}: Option<{}>,", field.var_name(), field.ty().cow_definition("a")));
        }
        try!(writeln!(writer, "}}\n"));
        Ok(())
    }

    pub fn write_inherent_impl<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        let lifetimes = if self.has_lifetimes { "<'a>" } else { "" };

        try!(writeln!(writer, "impl{} Properties{} {{", lifetimes, lifetimes));
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

        for field in &self.fields {
            let is_copy = field.ty().is_copy();
            let ty = field.ty().borrowed_type();
            let borrow = if is_copy { "" } else { "&" };
            try!(writeln!(writer, "pub fn {}(&self) -> Option<{}{}> {{", field.var_name(), borrow, ty));

            if is_copy {
                try!(writeln!(writer, "self.{}", field.var_name()));
            }
            else {
                try!(writeln!(writer, "self.{}.as_ref().map(|v| &**v)", field.var_name()));
            }

            try!(writeln!(writer, "}}"));
        }

        Ok(())
    }
}
