// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;
use std::io;
use specs::Spec;

use CodeGenerator;
use common::spec_mod_name;

pub struct SpecsModuleWriter<'a> {
    specs: Cow<'a, [Spec]>,
}

impl<'a> SpecsModuleWriter<'a> {
    pub fn new<S>(specs: S) -> Self
        where S: Into<Cow<'a, [Spec]>>
    {
        SpecsModuleWriter {
            specs: specs.into()
        }
    }
}

impl<'a> CodeGenerator for SpecsModuleWriter<'a> {
    fn write_rust_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        try!(writeln!(writer, ""));
        for spec in self.specs.iter() {
            let mod_name = spec_mod_name(spec);
            try!(writeln!(writer, "pub mod {};", mod_name));
        }
        try!(writeln!(writer, ""));

        Ok(())
    }
}