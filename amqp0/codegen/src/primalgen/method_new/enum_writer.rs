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
use super::MethodModuleWriter;

pub struct MethodEnumWriter<'a> {
    spec: &'a Spec,
}

impl<'a> CodeGenerator for MethodEnumWriter<'a> {
    fn write_rust_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        if self.spec.classes().is_empty() {
            return Ok(());
        }

        for class in self.spec.classes().values() {
  //          try!(writeln!(writer, "enum {}Method {{", pascal_case));
            for method in class.methods() {
                let pascal_case = method.name().to_pascal_case();
//                try!(writeln!(writer, "{1}({0}::{1}),", snake_case, pascal_case));
            }
            //try!(writeln!(writer, "}} // enum {}Method", pascal_case));
        }

        Ok(())
    }
}

impl<'a> MethodEnumWriter<'a> {
    pub fn new(spec: &'a Spec) -> Self {
        MethodEnumWriter {
            spec: spec
        }
    }
}