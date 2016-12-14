// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;

use inflections::Inflect;
use specs::Spec;

use CodeGenerator;
use common::spec_mod_name;
use common::domain::DomainMapper;
use super::MethodModuleWriter;

pub struct SpecModuleWriter<'a> {
    mod_name: String,
    domain_mapper: DomainMapper<'a>,
    spec: &'a Spec,
}

impl<'a> CodeGenerator for SpecModuleWriter<'a> {
    fn write_rust_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        if self.spec.classes().is_empty() {
            return Ok(());
        }

        try!(writeln!(writer, "\nuse nom::{{IResult, be_u8, be_u16, be_u32, be_u64}};\n"));

        for class in self.spec.classes().values() {
            try!(write!(writer, "// {} Class", class.name()));
            for method in class.methods() {
                let mod_name = format!("{}::{}", self.mod_name, class.name().to_snake_case());
                let method_writer = MethodModuleWriter::new(&mod_name, method, &self.domain_mapper);
                try!(method_writer.write_rust_to(writer));
            }
        }

        Ok(())
    }
}

impl<'a> SpecModuleWriter<'a> {
    pub fn new(spec: &'a Spec) -> Self {
        SpecModuleWriter {
            mod_name: spec_mod_name(spec),
            domain_mapper: DomainMapper::new(spec.domains()),
            spec: spec,
        }
    }

    pub fn mod_name(&self) -> &str {
        &self.mod_name
    }
}