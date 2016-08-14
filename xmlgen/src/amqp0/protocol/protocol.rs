
use std::borrow::Cow;
use inflections::Inflect;
use parser::amqp0 as parsed;

use Error;
use common::ConstantGroup;
use super::Constant;

pub struct Protocol<'a>(parsed::Protocol<'a>);
// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

impl<'a> Protocol<'a> {
    pub fn new(protocol: parsed::Protocol<'a>) -> Self {
        Protocol(protocol)
    }

    pub fn parsed_protocol<'_self>(&'_self self) -> &'_self parsed::Protocol<'a> {
        &self.0
    }

    pub fn map_constant<'c>(&self, constant: &'c parsed::Constant<'c>) -> Constant<'c> {
        let ty = match constant.name().to_kebab_case().as_str() {
            "frame-end" => "u8",
            "frame-min-size" => "usize",
            _ => "u16",
        };
        Constant::new(constant.name(), ty, constant.value())
    }

    pub fn map_domain<'b, D>(&self, domain: D) -> Cow<'a, str>
        where D: Into<Cow<'b, str>>
    {
        let mapped_domain = {
            let mut domain = domain.into();
            loop {
                let orig = domain.clone();
                let mapping = self.0.domain(&domain);
                if let Some(mapping) = mapping {
                    println!("Set Domain: {}", mapping.mapping());
                    domain = Cow::Borrowed(mapping.mapping());
                }
                if orig == domain {
                    println!("Break: {}", domain);
                    break
                }
            }

            match &*domain {
                "bit"       => "bool".into(),
                "octet"     => "u8".into(),
                "short"     => "u16".into(),
                "long"      => "u32".into(),
                "longlong"  => "u64".into(),
                "shortstr"  => "String".into(),
                "longstr"   => "Vec<u8>".into(),
                "timestamp" => "i64".into(),
                "table"     => "Table".into(),
                domain      =>  Cow::Owned(domain.to_string()),
            }
        };

        // in the earlier specs, some of these domains aren't defined
        // these are fallbacks in case they aren't defined earlier
        match mapped_domain {
            ref domain if domain == "queue name" => "u16".into(),
            ref domain if domain == "class-id" => "u16".into(),
            ref domain if domain == "method-id" => "u16".into(),
            ref domain if domain == "content" => "Vec<u8>".into(),
            domain => domain,
        }
    }

    pub fn map_error_class_enum_name<'b>(&self, error_class: &'b str) -> Cow<'b, str> {
        let error_class = error_class.to_pascal_case();
        if error_class.ends_with("Error") {
            Cow::Owned((&error_class[..(error_class.len() - 5)]).to_owned())
        }
        else {
            Cow::Owned(error_class)
        }
    }

    pub fn map_group_enum_name<'b>(&self, group: &'b ConstantGroup, constant: &'b parsed::Constant) -> Cow<'b, str> {
        let name = constant.name();
        let name = match name.to_pascal_case() {
            ref pascal_case if name == pascal_case => Cow::Borrowed(name),
            pascal_case => Cow::Owned(pascal_case),
        };
        let group_group = group.group().to_pascal_case();
        match group_group.as_str() {
            "FrameType" if name.starts_with("FrameOob") => Cow::Owned("OutOfBand".to_string() + &name[8..]),
            "FrameType" if name.starts_with("Frame") => Cow::Owned((&*name)[5..].to_string()),
            // TODO 2016-08: should be able to borrow here
            "ResponseCode" if name.len() != 5 && name.ends_with("Error") => Cow::Owned((&*name)[..(name.len() - 5)].to_string()),
            _ => name
        }
    }

    pub fn map_group_constant_name<'b>(&self, group: &'b ConstantGroup, constant: &'b parsed::Constant) -> Cow<'b, str> {
        let name = constant.name();
        let name = match constant.name().to_constant_case() {
            ref constant_case if name == constant_case => Cow::Borrowed(name),
            constant_case => Cow::Owned(constant_case)
        };
        let group_group = group.group().to_pascal_case();
        match group_group.as_str() {
            "FrameType" if name.starts_with("FRAME") => ("TYPE".to_string() + &name[5..]).into(),
            //"ResponseCode" if name == "TYPE_ERROR" => Cow::Borrowed("TYPE_TYPE"),
            "ResponseCode" if name == "INVALID_PATH" => Cow::Borrowed("TYPE_PATH_INVALID"),
            "ResponseCode" if name.ends_with("_ERROR") => Cow::Owned("TYPE_".to_owned() + &name[..(name.len() - 6)]),
            "ResponseCode" => Cow::Owned("TYPE_".to_owned() + &name),
            _ => name,
        }
    }

    pub fn map_value<'value>(&self, ty: &str, value: &'value str) -> Result<Cow<'value, str>, Error> {
        match ty {
            "bool"  => Ok(Some(Cow::Borrowed(if value != "0" { "true" } else { "false" }))),
            "u8"    => value.parse::<u8>().map(|v|    Some(Cow::Owned(format!("{}", v)))),
            "u16"   => value.parse::<u16>().map(|v|   Some(Cow::Owned(format!("{}", v)))),
            "u32"   => value.parse::<u32>().map(|v|   Some(Cow::Owned(format!("{}", v)))),
            "u64"   => value.parse::<u16>().map(|v|   Some(Cow::Owned(format!("{}", v)))),
            "usize" => value.parse::<usize>().map(|v| Some(Cow::Owned(format!("{}", v)))),
            _ => Ok(None)
        }.map(|v| v.unwrap_or(value.to_string().into()))
         .map_err(|_| Error::InvalidValue(ty.to_string().into(), value.to_string().into()))
    }
}