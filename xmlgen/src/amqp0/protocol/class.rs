// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;
use std::io;

use inflections::Inflect;
use parser::amqp0 as parsed;

use {WriteRust, Error};
use amqp0::Protocol;

pub struct Class<'a, 'b: 'a>(&'a parsed::Class<'b>);

impl<'a, 'b> Class<'a, 'b> {
    pub fn from_parsed(class: &'a parsed::Class<'b>) -> Self {
        Class(class)
    }
}

impl<'a, 'b: 'a> WriteRust<Protocol<'b>> for Class<'a, 'b> {
    fn write_rust<W>(&self, protocol: &Protocol<'b>, writer: &mut W) -> Result<(), Error>
        where W: io::Write
    {
        let module_name = match self.0.name().to_snake_case() {
            ref name if name == "tx" => Cow::Borrowed("transaction"),
            name => Cow::Owned(name),
        };
        try!(writeln!(writer, "pub mod {} {{", module_name));
        {
            try!(self.0.methods().iter()
                .map(|m| {
                    let ty = protocol.map_domain("method-id");
                    let value = try!(protocol.map_value(&ty, m.index()));
                    let name = m.name().to_constant_case();

                    try!(writeln!(writer, "pub const METHOD_{}: {} = {};", name, ty, value));

                    Ok(())
                })
                .collect::<Result<Vec<_>, Error>>()
                .map(|_| ())
            );

            try!(self.0.methods().iter()
                .map(|m| {
                    let name = m.name().to_pascal_case();
                    if m.fields().len() > 0 {

                        let generic_params = m.fields().iter()
                            .filter(|f| !(f.is_optional() || f.is_reserved()))
                            .fold(vec![], |mut params, f| {
                                let ty = protocol.map_domain(f.domain());

                                if ty == "String" || ty == "Vec<u8>" {
                                    let char_u8 = ('A' as u8) + (params.len() as u8);
                                    let char_str = (char_u8 as char).to_string();
                                    params.push((ty, char_str));
                                }
                                params
                            });

                        if generic_params.len() > 0 {
                            try!(writeln!(writer, "pub struct {}Method<'a> {{", name));
                        }
                        else {
                            try!(writeln!(writer, "pub struct {}Method {{", name));
                        }

                        try!(m.fields().iter()
                            .filter(|f| !f.is_reserved())
                            .map(|f| {
                                let member_name = match f.name().to_snake_case() {
                                    ref name if name == "type" => Cow::Borrowed("ty"),
                                    name => Cow::Owned(name),
                                };

                                let ty = match protocol.map_domain(f.domain()) {
                                    ref ty if ty == "String" => Cow::Borrowed("::std::borrow::Cow<'a, str>"),
                                    ref ty if ty == "Vec<u8>" => Cow::Borrowed("::std::borrow::Cow<'a, [u8]>"),
                                    ref ty if ty == "Table" => Cow::Borrowed("super::super::Table"),
                                    ty => ty,
                                };
                                if f.is_optional() {
                                    try!(writeln!(writer, "{}: Option<{}>,", member_name, ty));
                                }
                                else {
                                    try!(writeln!(writer, "{}: {},", member_name, ty));
                                }

                                Ok(())
                            })
                            .collect::<Result<Vec<_>, Error>>()
                            .map(|_| ())
                        );
                        try!(writeln!(writer, "}}"));


                        if generic_params.len() > 0 {
                            try!(writeln!(writer, "impl<'a> {}Method<'a> {{", name));
                        }
                        else {
                            try!(writeln!(writer, "impl {}Method {{", name));
                        }

                        {
                            // pub fn Method::new()
                            try!(write!(writer, "pub fn new"));

                            let generic_names = generic_params.iter()
                                .map(|&(_, ref _char)| _char.as_str())
                                .collect::<Vec<_>>();

                            try!(write!(writer, "<{}>", generic_names.join(", ")));

                            let mut generic_param_iter = generic_params.iter();
                            try!(writeln!(writer, "("));
                            try!(m.fields().iter()
                                    .filter(|f| !f.is_reserved())
                                    .map(|f| {
                                        let member_name = match f.name().to_snake_case() {
                                            ref name if name == "type" => Cow::Borrowed("ty"),
                                            name => Cow::Owned(name),
                                        };

                                        let ty = match protocol.map_domain(f.domain()) {
                                            ref ty if ty == "String" => Cow::Borrowed(generic_param_iter.next().unwrap().1.as_str()),
                                            ref ty if ty == "Vec<u8>" => Cow::Borrowed(generic_param_iter.next().unwrap().1.as_str()),
                                            ref ty if ty == "Table" => Cow::Borrowed("super::super::Table"),
                                            ty => ty,
                                        };

                                        try!(writeln!(writer, "    {}: {}, ", member_name, ty));

                                        Ok(())
                                    })
                                    .collect::<Result<Vec<_>, Error>>()
                                    .map(|_| ())
                            );
                            if generic_params.len() == 0 {
                                try!(writeln!(writer, ") -> Self {{"));
                            }
                            else {
                                let generic_clause = generic_params.iter()
                                    .map(|&(ref ty, ref _char)| {
                                        let ty = match &**ty {
                                            "String" => "str",
                                            "Vec<u8>" => "[u8]",
                                            _ => panic!("Unexpected generic type"),
                                        };
                                        format!("{}: Into<::std::borrow::Cow<'a, {}>>", _char, ty)
                                    })
                                    .collect::<Vec<_>>()
                                    .join(",\n  ");
                                try!(writeln!(writer, ") -> Self"));
                                try!(write!(writer, " where "));
                                try!(write!(writer, "{} ", generic_clause));
                                try!(writeln!(writer, "{{"))
                            }

                            {
                                try!(writeln!(writer, "{}Method {{", name));
                                try!(m.fields().iter()
                                    .filter(|f| !(f.is_reserved()))
                                    .map(|f| {
                                        let member_name = match f.name().to_snake_case() {
                                            ref name if name == "type" => Cow::Borrowed("ty"),
                                            name => Cow::Owned(name),
                                        };

                                        let value = match &*protocol.map_domain(f.domain()) {
                                            "String" | "Vec<u8>" if f.is_optional() => {
                                                format!("Some({0}.into())", member_name)
                                            },
                                            "String" | "Vec<u8>" => {
                                                format!("{0}.into()", member_name)
                                            },
                                            _ if f.is_optional() => {
                                                format!("Some({})", member_name)
                                            }
                                            _ => format!("{}", member_name),
                                        };

                                        try!(writeln!(writer, "{}: {},", member_name, value));
                                        Ok(())
                                    })
                                    .collect::<Result<Vec<_>, Error>>()
                                    .map(|_| ())
                                );
                                try!(writeln!(writer, "}}"))
                            }
                            // fn new()
                            try!(writeln!(writer, "}}"));
                        }
                        {
                            try!(m.fields().iter()
                                    .filter(|f| !f.is_reserved())
                                    .map(|f| {
                                        let member_name = match f.name().to_snake_case() {
                                            ref name if name == "type" => Cow::Borrowed("ty"),
                                            name => Cow::Owned(name),
                                        };

                                        let ty = match protocol.map_domain(f.domain()) {
                                            ref ty if ty == "String" => Cow::Borrowed("&str"),
                                            ref ty if ty == "Vec<u8>" => Cow::Borrowed("&[u8]"),
                                            ref ty if ty == "Table" => Cow::Borrowed("&super::super::Table"),
                                            ty => ty,
                                        };

                                        let _ref = if ty.starts_with("&") { "&" } else { "" };

                                        try!(writeln!(writer, "pub fn {}(&self) -> {} {{", member_name, ty));
                                        try!(writeln!(writer, "{}self.{}", _ref, member_name));
                                        try!(writeln!(writer, "}}"));

                                        Ok(())
                                    })
                                    .collect::<Result<Vec<_>, Error>>()
                                    .map(|_| ())
                            );
                        }
                        // impl Method<'a>
                        try!(writeln!(writer, "}}"));
                    }
                    else {
                        try!(writeln!(writer, "pub struct {};", name));
                    }

                    Ok(())
                })
                .collect::<Result<Vec<_>, Error>>()
                .map(|_| ())
            );
        }
        try!(writeln!(writer, "}}"));

        Ok(())
    }
}
