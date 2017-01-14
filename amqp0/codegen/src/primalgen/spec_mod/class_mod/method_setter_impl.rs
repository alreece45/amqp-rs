// Copyright 2017 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;
use std::io;

use WriteRust;
use common::{Class, ClassMethod};

pub struct MethodSetterImplWriter<'a> {
    class: &'a Class,
    method: &'a ClassMethod,
}

impl<'a> WriteRust for MethodSetterImplWriter<'a> {
    fn write_rust_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        let lifetimes = if self.method.has_lifetimes() { "<'a>" } else { "" };

        if self.method.has_usable_fields() {
            let section = format!(
                "impl{lifetimes} ::method::{class}::Set{method}MethodFields{lifetimes} \
                    for {method}{lifetimes}",
                lifetimes = lifetimes,
                class = self.class.snake_case(),
                method = self.method.pascal_case()
            );

            try!(writeln!(writer, "{} {{", section));
            for field in self.method.fields() {
                if field.is_reserved() {
                    continue;
                }

                let ty = field.ty();
                let (ty, generics, bounds, into) = if ty.is_copy() {
                    (ty.owned_type(), "", "". into(), "")
                } else {
                    (
                        "V",
                        "<V>",
                        Cow::Owned(format!("\nwhere V: Into<{}>", ty.cow_definition("a"))),
                        ".into()",
                    )
                };

                try!(writeln!(
                    writer,
                    "fn set_{var_name}{generics}(&mut self, {var_name}: {ty}) {bounds} {{\n\
                        self.set_{var_name}({var_name}{into})\n\
                    }} // set_{var_name}()",
                    var_name = field.var_name(),
                    generics = generics,
                    ty = ty,
                    bounds = bounds,
                    into = into,
                ));
            }
            try!(writeln!(writer, "}} // {}", section));
        }
        Ok(())
    }
}

impl<'a> MethodSetterImplWriter<'a> {
    pub fn new(class: &'a Class, method: &'a ClassMethod) -> Self {
        MethodSetterImplWriter {
            class: class,
            method: method,
        }
    }
}
