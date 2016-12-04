// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::BTreeMap;
use std::io;

use amqp0::{Assertion, Class, Constant, Domain, Field, Method, Spec, Version};
use codegen::{self, FormatRustCode};

#[allow(match_same_arms)]
pub fn write_generated<W>(writer: &mut W, name: &'static str, spec: &Spec) -> io::Result<()>
    where W: io::Write
{
    let (ungrouped, frame_types, response_codes) = {
        let (mut ungrouped, mut frame_types, mut response_codes) = (vec![], vec![], vec![]);
        for constant in spec.constants().values() {
            match constant.name().replace(" ", "-").as_str() {
                "frame-min-size" | "frame-end"      => &mut ungrouped,
                name if name.starts_with("frame-")
                    && name != "frame-error"        => &mut frame_types,
                _ if constant.value().len() == 3    => &mut response_codes,
                _                                   => &mut ungrouped,
            }.push((constant.name().replace(" ", "-"), constant));
        };
        (ungrouped, frame_types, response_codes)
    };
    try!(writeln!(writer, "Spec {{"));
    try!(writeln!(writer, "name: {},", name.format_rust()));
    try!(writeln!(writer, "classes: {},", spec.classes().format_rust()));
    try!(writeln!(writer, "constants: {}.into_iter().collect(),", codegen::format_to_vec(ungrouped.iter())));
    try!(writeln!(writer, "domains: {},", spec.domains().format_rust()));
    try!(writeln!(writer, "frame_types: {}.into_iter().collect(),", codegen::format_to_vec(frame_types.iter())));
    try!(writeln!(writer, "response_codes: {}.into_iter().collect(),", codegen::format_to_vec(response_codes.iter())));
    try!(writeln!(writer, "version: {},", spec.version().format_rust()));
    try!(writeln!(writer, "}}"));

    Ok(())
}

impl<'a> FormatRustCode for BTreeMap<String, Class<'a>> {
    fn format_rust(&self) -> String {
        format!("{}.into_iter().collect()", codegen::format_to_vec(self.iter()))
    }
}

impl<'a> FormatRustCode for BTreeMap<String, Domain<'a>> {
    fn format_rust(&self) -> String {
        let iter = self.iter().map(|(k, v)| (k.replace(" ", "-"), v));
        format!("{}.into_iter().collect()", codegen::format_to_vec(iter))
    }
}

impl<'a> FormatRustCode for Version {
    fn format_rust(&self) -> String {
        format!(
            "Version {{ minor: {}, revision: {} }}",
            self.minor(),
            self.revision(),
        )
    }
}

impl<'a> FormatRustCode for Class<'a> {
    fn format_rust(&self) -> String {
        format!(
            "Class {{\nname: {},\nindex: {},\nmethods: {}\n}}",
            self.name().format_rust(),
            self.index(),
            self.methods().format_rust(),
        )
    }
}

impl<'a> FormatRustCode for Domain<'a> {
    fn format_rust(&self) -> String {
        self.mapping().format_rust()
    }
}

impl<'a> FormatRustCode for Constant<'a> {
    fn format_rust(&self) -> String {
        format!(
            "Constant {{\nname: {},\nvalue: {},\nclass: {}\n}}",
            self.name().replace(" ", "-").format_rust(),
            self.value(),
            self.class().map(|c| c.replace(" ", "-")).format_rust()
        )
    }
}

impl<'a> FormatRustCode for Method<'a> {
    fn format_rust(&self) -> String {
        let chassis = self.chassis();
        let chassis_client = chassis.get("client");
        let chassis_server = chassis.get("server");
        format!(
            "ClassMethod {{\nname: {},\nindex: {},\nchassis_client: {},\nchassis_server: {},\nresponse: {},\nis_synchronous: {},\nfields: {},\n}}",
            self.name().format_rust(),
            self.index(),
            chassis_client.format_rust(),
            chassis_server.format_rust(),
            self.response().format_rust(),
            self.is_synchronous().format_rust(),
            self.fields().format_rust(),
        )
    }
}

impl<'a> FormatRustCode for Field<'a> {
    fn format_rust(&self) -> String {
        format!(
            "ClassMethodField {{\nname: {},\ndomain: {},\nassertions: {},\nis_reserved: {}\n}}",
            self.name().replace(" ", "-").format_rust(),
            self.domain().replace(" ", "-").format_rust(),
            self.assertions().format_rust(),
            self.is_reserved().format_rust()
        )
    }
}

impl FormatRustCode for Assertion {
    fn format_rust(&self) -> String {
        match *self {
            Assertion::Null => "ClassMethodFieldAssertion::Null".to_string(),
            Assertion::NotNull => "ClassMethodFieldAssertion::NotNull".to_string(),
            Assertion::ChannelMax => "ClassMethodFieldAssertion::ChannelMax".to_string(),
            Assertion::NotZero => "ClassMethodFieldAssertion::NotZero".to_string(),
            Assertion::Enum(ref values) => {
                format!("ClassMethodFieldAssertion::Enum({})", codegen::format_to_vec(values.iter()))
            },
            Assertion::Length(ref length) => format!("ClassMethodFieldAssertion::Length({})", length),
            Assertion::Regexp(ref pattern) => {
                format!("ClassMethodFieldAssertion::Regexp({})", pattern.format_rust())
            },
            Assertion::Syntax(ref syntax) => {
                format!("ClassMethodFieldAssertion::Syntax({})", syntax.format_rust())
            },
        }
    }
}