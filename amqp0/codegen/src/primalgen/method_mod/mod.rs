// Copyright 2017 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod builder_struct;
mod builder_impls;
mod builder_default_impl;
mod builder_setter_impl;
mod setter_trait_def;

use std::io;
use inflections::Inflect;

use common::Specs;
use WriteRust;

use self::builder_struct::BuilderStructWriter;
use self::builder_default_impl::DefaultImplWriter;
use self::builder_impls::BuilderImplsWriter;
use self::builder_setter_impl::BuilderSetterImplWriter;
use self::setter_trait_def::SetterTraitDefinitionWriter;

pub struct MethodModuleWriter<'a> {
    specs: &'a Specs<'a>
}

impl<'a> MethodModuleWriter<'a> {
    pub fn new(specs: &'a Specs<'a>) -> Self {
        MethodModuleWriter {
            specs: specs
        }
    }
}

impl<'a> WriteRust for MethodModuleWriter<'a> {
    fn write_rust_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        for class_name in self.specs.class_names() {
            let class_snake = class_name.to_snake_case();

            try!(writeln!(writer, "\npub mod {} {{", class_snake));

            for method in self.specs.class_methods(class_name) {
                let lifetimes = if method.has_lifetimes() { "<'a>" } else { "" };
                let pascal_method = method.method_name().to_pascal_case();

                let section = format!("pub trait {}Method{}", pascal_method, lifetimes);
                try!(write!(writer, "{} {{\ntype Payload: Default", section));

                if method.has_usable_fields() {
                    try!(write!(writer, " + Set{}MethodFields{}", pascal_method, lifetimes))
                }
                try!(writeln!(writer, ";\n}} // {}\n", section));

                let setter = SetterTraitDefinitionWriter::new(method);
                try!(setter.write_rust_to(writer));

                let builder_struct = BuilderStructWriter::new(method);
                try!(builder_struct.write_rust_to(writer));

                let builder_impls = BuilderImplsWriter::new(method);
                try!(builder_impls.write_rust_to(writer));

                let builder_impl = DefaultImplWriter::new(method);
                try!(builder_impl.write_rust_to(writer));

                let setter_impl = BuilderSetterImplWriter::new(method);
                try!(setter_impl.write_rust_to(writer));
            }

            try!(writeln!(writer, "}} // mod {}", class_snake));
        }

        Ok(())
    }
}