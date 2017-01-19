// Copyright 2017 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;
use std::io;
use inflections::Inflect;

use common::SpecMethod;
use WriteRust;

pub struct BuilderSetterImplWriter<'a> {
    method: &'a SpecMethod,
}

impl<'a> WriteRust for BuilderSetterImplWriter<'a> {
    fn write_rust_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        if !self.method.has_usable_fields() {
            return Ok(())
        }

        let pascal_method = self.method.method_name().to_pascal_case();
        let section = format!(
            "impl<{lifetimes}T> {method}Builder<T>",
            lifetimes = if self.method.has_lifetimes() { "'a, " } else { "" },
            method = pascal_method,
        );

        try!(writeln!(
            writer,
            "{section}\n\
                where T: {traits} + Set{method}MethodFields{lifetimes}\n\
            {{\n",
            section = section,
            method = pascal_method,
            traits = self.method.method_traits(),
            lifetimes = if self.method.has_lifetimes() { "<'a>" } else { "" },
        ));

        for field in self.method.fields() {
            let (var_name, ty) = (field.var_name(), field.ty());
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
                "pub fn {var_name}{generics}(mut self, {var_name}: {ty}) -> Self {bounds} {{\n\
                    Set{method}MethodFields::set_{var_name}(&mut self.payload, {var_name}{into});\n\
                    self\n\
                }} // set_{var_name}()",
                method = pascal_method,
                var_name = var_name,
                generics = generics,
                ty = ty,
                bounds = bounds,
                into = into,
            ));
        }

        if self.method.has_content() {
            try!(writeln!(
                writer,
                "pub fn set_headers<V>(self, _: V) -> Self\n\
                    where V: Into<<T as ::Content<'a>>::Headers>\n\
                {{\n\
                    self\n\
                }}\n\
                \n\
                pub fn set_body<V>(self, _: V) -> Self\n\
                    where V: Into<::std::borrow::Cow<'a, [u8]>>\n\
                {{\n\
                    self\n\
                }}",
            ))
        }

        try!(writeln!(writer, "}} // {}", section));
        Ok(())
    }
}

impl<'a> BuilderSetterImplWriter<'a> {
    pub fn new(method: &'a SpecMethod) -> Self {
        BuilderSetterImplWriter {
            method: method,
        }
    }
}
