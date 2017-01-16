// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::BTreeMap;

use super::FormatRustCode;
use super::{format_to_map, format_to_slice};
use {Assertion, Constant, Class, ClassField, ClassMethod, ClassMethodField, Domain, Version};

impl<'a> FormatRustCode for BTreeMap<String, Class<'a>> {
    fn format_rust(&self) -> String {
        format_to_map(self.iter())
    }
}

impl<'a> FormatRustCode for BTreeMap<String, Domain<'a>> {
    fn format_rust(&self) -> String {
        let iter = self.iter().map(|(k, v)| (k.replace(" ", "-"), v));
        format_to_map(iter)
    }
}

impl<'a> FormatRustCode for Version {
    fn format_rust(&self) -> String {
        format!(
            "::Version {{ minor: {}, revision: {} }}",
            self.minor(),
            self.revision(),
        )
    }
}

impl<'a> FormatRustCode for Class<'a> {
    fn format_rust(&self) -> String {
        format!(
            "::Class {{\nname: {},\nfields: {},\nindex: {},\nmethods: {}\n}}",
            self.name().format_rust(),
            self.fields().format_rust(),
            self.index(),
            self.methods().format_rust(),
        )
    }
}

impl<'a> FormatRustCode for ClassField<'a> {
    fn format_rust(&self) -> String {
        format!(
            "::ClassField {{\nname: {},\ndomain: {}\n}}",
            self.name().replace(" ", "-").format_rust(),
            self.domain().replace(" ", "-").format_rust(),
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
            "::Constant {{\nname: {},\nvalue: {},\nclass: {}\n}}",
            self.name().replace(" ", "-").format_rust(),
            self.value(),
            self.class().map(|c| c.replace(" ", "-")).format_rust()
        )
    }
}

impl<'a> FormatRustCode for ClassMethod<'a> {
    fn format_rust(&self) -> String {
        let chassis = self.chassis();
        let chassis_client = chassis.get("client");
        let chassis_server = chassis.get("server");
        format!(
            "::ClassMethod {{\nname: {},\nindex: {},\nchassis_client: {},\nchassis_server: {},\nresponse: {},\nis_synchronous: {},\nfields: {},\n}}",
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

impl<'a> FormatRustCode for ClassMethodField<'a> {
    fn format_rust(&self) -> String {
        format!(
            "::ClassMethodField {{\nname: {},\ndomain: {},\nassertions: {},\nis_reserved: {}\n}}",
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
            Assertion::Null => "::ClassMethodFieldAssertion::Null".to_string(),
            Assertion::NotNull => "::ClassMethodFieldAssertion::NotNull".to_string(),
            Assertion::ChannelMax => "::ClassMethodFieldAssertion::ChannelMax".to_string(),
            Assertion::NotZero => "::ClassMethodFieldAssertion::NotZero".to_string(),
            Assertion::Enum(ref values) => {
                format!("::ClassMethodFieldAssertion::Enum({})", format_to_slice(values.iter()))
            },
            Assertion::Length(ref length) => format!("::ClassMethodFieldAssertion::Length({})", length),
            Assertion::Regexp(ref pattern) => {
                format!("::ClassMethodFieldAssertion::Regexp({})", pattern.format_rust())
            },
            Assertion::Syntax(ref syntax) => {
                format!("ClassMethodFieldAssertion::Syntax({})", syntax.format_rust())
            },
        }
    }
}