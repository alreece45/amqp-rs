// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap};

impl Spec {
    pub fn new<N>(name: N, protocol: parsed::Protocol<'b>) -> Self
        where N: Into<Cow<'a, str>>
    {
        let name = name.into();
        let (minor, revision) = {
            let version = protocol.version();
            (version.minor(), version.revision())
        };

        Protocol {
            name: name,
            pascal_name: pascal_name,
            snake_name: snake_name,
            protocol: protocol,
            classes: BTreeMap::new(),
        }
    }

    pub fn classes(&self) -> &BTreeMap<String, parsed::Class<'b>> {
        self.parsed_protocol().classes()
    }

    pub fn parsed_protocol<'_self>(&'_self self) -> &'_self parsed::Protocol<'b> {
        &self.protocol
    }

    pub fn name(&self) -> &str {
        &*self.name
    }

    pub fn version(&self) -> &Version {
        self.parsed_protocol().version()
    }

    pub fn map_constant<'c>(&self, constant: &'c parsed::Constant<'c>) -> Constant<'c> {
        let ty = match constant.name().to_kebab_case().as_str() {
            "frame-end" => "u8",
            "frame-min-size" => "usize",
            _ => "u16",
        };
        Constant::new(constant.name(), ty, constant.value())
    }

    pub fn map_domain<'c, D>(&self, domain: D) -> Cow<'c, str>
        where D: Into<Cow<'c, str>>
    {
        let mapped_domain = {
            let mut domain = domain.into();
            loop {
                let orig = domain.clone();
                let mapping = self.protocol.domain(&domain);
                if let Some(mapping) = mapping {
                    domain = Cow::Borrowed(mapping.mapping());
                }
                if orig == domain {
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

    pub fn map_error_class_enum_name<'c>(&self, error_class: &'c str) -> Cow<'c, str> {
        let error_class = error_class.to_pascal_case();
        if error_class.ends_with("Error") {
            Cow::Owned((&error_class[..(error_class.len() - 5)]).to_owned())
        }
        else {
            Cow::Owned(error_class)
        }
    }

    pub fn map_group_enum_name<'c>(&self, group: &'c ConstantGroup, constant: &'c parsed::Constant) -> Cow<'c, str> {
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

    pub fn map_group_constant_name<'c>(&self, group: &'c ConstantGroup, constant: &'c parsed::Constant) -> Cow<'c, str> {
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

    pub fn write_rust<W>(&self, writer: &mut W) -> Result<(), Error>
        where W: io::Write
    {
        let parsed = self.parsed_protocol();

        // organize the constants
        let (ungrouped, frame_types, response_codes) = {
            let (mut ungrouped, mut frame_types, mut response_codes) = (vec![], vec![], vec![]);

            for (name, c) in parsed.constants() {
                match name.to_kebab_case().as_str() {
                    "frame-min-size"                    => &mut ungrouped,
                    "frame-end"                         => &mut ungrouped,
                    name if name.starts_with("frame-")
                        && name != "frame-error"        => &mut frame_types,
                    _ if c.value().len() == 3           => &mut response_codes,
                    _                                   => &mut ungrouped,
                }.push(c);
            }

            (ungrouped, frame_types, response_codes)
        };


        let name = self.name().to_snake_case();
        let version = self.version();
        let module = format!("{}0_{}_{}", name, version.minor(), version.revision());

        // protocol module
        try!(writeln!(writer, "pub mod {} {{", module));
        {
            try!(writeln!(writer, "pub mod frame {{"));

            let frame_type_group = ConstantGroup::new("FrameType", "Type", "u16", frame_types.iter().cloned());
            try!(frame_type_group.write_rust(self, writer));

            try!(frame_types.iter()
                .map(|frame_type| {
                    writeln!(writer, "pub struct {};", &frame_type.name().to_pascal_case()[5..])
                })
                .collect::<Result<Vec<_>, _>>()
                .map(|_| ())
            );
            try!(writeln!(writer, "}}"));
            // mod frame

            try!(writeln!(writer, "pub struct Parser;"))

            // enum Frame
            try!(writeln!(writer, "pub enum Frame {{"));
            try!(frame_types.iter()
                .map(|frame_type| {
                    let name = &frame_type.name().to_pascal_case()[5..];
                    writeln!(writer, "{0}(frame::{0}),", name)
                })
                .collect::<Result<Vec<_>, _>>()
                .map(|_| ())
            );
            try!(writeln!(writer, "}}"));
            // enum frame

            try!(writeln!(writer, "pub struct Table;"));

            // mod class
            try!(writeln!(writer, "pub mod class {{"));
            try!(parsed.classes().iter()
                .map(|(_, c)| {
                    let ty = self.map_domain("class-id");
                    let value = try!(self.map_value(&ty, c.index()));
                    let name = c.name().to_constant_case();
                    try!(writeln!(writer, "pub const CLASS_{}: {} = {};", name, ty, value));

                    Ok(())
                })
                .collect::<Result<Vec<_>, Error>>()
                .map(|_| ())
            );
            try!(writeln!(writer, ""));

            try!(parsed.classes().iter()
                .map(|(_, c)| {
                    try!(Class::from_parsed(&c).write_rust(self, writer));
                    Ok(())
                })
                .collect::<Result<Vec<_>, Error>>()
                .map(|_| ())
            );
            try!(writeln!(writer, "}}"));
            // mod class
        }
        try!(writeln!(writer, "}}"));
        // protocol module

        Ok(())
    }
}