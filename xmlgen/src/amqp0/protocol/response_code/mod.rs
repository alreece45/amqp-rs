// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::BTreeMap;
use std::io;

use inflections::Inflect;
use parser::amqp0 as parsed;

use {Error, WriteRust};
use amqp0::protocol::Protocol;
use common::ConstantGroup;
use super::Constant;

mod error_class;
mod unified;

use self::error_class::ErrorClass;
use self::unified::Unified;

#[derive(Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub enum ResponseType {
    Success,
    Info,
    Redirect,
    ClientError,
    ServerError,
}

impl ResponseType {
    fn from_parser_constant(constant: &parsed::Constant) -> Option<Self> {
        Some(match constant.value().chars().next() {
            Some('1') => ResponseType::Info,
            Some('2') => ResponseType::Success,
            Some('3') => ResponseType::Redirect,
            Some('4') => ResponseType::ClientError,
            Some('5') => ResponseType::ServerError,
            _   => return None,
        })
    }

    fn name(&self) -> &str {
        match *self {
            ResponseType::Info => "Info",
            ResponseType::Success => "Success",
            ResponseType::Redirect => "Redirect",
            ResponseType::ClientError => "ClientError",
            ResponseType::ServerError => "ServerError",
        }
    }
}

#[derive(Debug)]
pub struct Group<'a> {
    groups: BTreeMap<ResponseType, Vec<&'a parsed::Constant<'a>>>,
}

impl<'a> Group<'a> {
    pub fn new<I>(constants: I) -> Self
        where I: Iterator<Item=&'a parsed::Constant<'a>>
    {
        let groups = BTreeMap::new();

        Group {
            groups: constants.fold(groups, |mut map, code| {
                if let Some(response_type) = ResponseType::from_parser_constant(code) {
                    map.entry(response_type)
                        .or_insert_with(|| vec![])
                        .push(code)
                }
                map
            })
        }
    }

    pub fn groups(&self) -> &BTreeMap<ResponseType, Vec<&'a parsed::Constant<'a>>> {
        &self.groups
    }
}

impl<'a> WriteRust<Protocol<'a>> for Group<'a> {
    fn write_rust<W>(&self, protocol: &Protocol, writer: &mut W) -> Result<(), Error>
        where W: io::Write
    {
        try!(writeln!(writer, "pub mod response {{"));
        {
            let groups = self.groups().iter()
                .map(|(response_type, constants)| {
                    (response_type.name(), constants)
                })
                .collect::<Vec<_>>();

            try!(ErrorClass::new(self).write_rust(protocol, writer));

            // sub-modules
            try!(groups.iter()
                .filter(|&&(_, ref constants)| constants.len() != 0)
                .map(|&(ref name, ref constants)| {
                    if constants.len() == 1 {
                        try!(constants.iter()
                            .map(|c| Constant::new(c.name(), "u16", c.value()))
                            .map(|c| c.write_rust(protocol, writer))
                            .collect::<Result<Vec<_>, _>>()
                            .map(|_| ()))
                    }
                    else {
                        let module_name = name.to_snake_case();

                        try!(writeln!(writer, "pub mod {} {{", module_name));
                        let group = ConstantGroup::new("ResponseCode", "Type", "u16", constants.iter().map(|c| *c));
                        try!(group.write_rust(protocol, writer));

                        let is_all_errors = constants.iter().all(|c| c.class().is_some());
                        if is_all_errors {
                            try!(writeln!(writer, "impl Type {{"));
                            {
                                try!(writeln!(writer, "pub fn error_class(&self) -> super::ErrorClass {{"));
                                {
                                    try!(writeln!(writer, "use super::ErrorClass;"));
                                    try!(writeln!(writer, "match *self {{"));
                                    try!(constants.iter()
                                        .map(|c| {
                                            let enum_name = protocol.map_group_enum_name(&group, c);
                                            let error_class = protocol.map_error_class_enum_name(c.class().unwrap());

                                            writeln!(writer, "Type::{} => ErrorClass::{},", enum_name, error_class)
                                        })
                                        .collect::<Result<Vec<_>, _>>()
                                        .map(|_| ()));
                                    try!(writeln!(writer, "}}"));
                                }
                                try!(writeln!(writer, "}}"));
                            }
                            try!(writeln!(writer, "}}"));
                        }
                        try!(writeln!(writer, "}}"));

                        let enum_name = name.to_pascal_case() + "Type";
                        try!(writeln!(writer, "pub use self::{}::Type as {};", module_name, enum_name));
                        try!(writeln!(writer, ""));
                    }

                    Ok(())
                })
                .collect::<Result<Vec<_>, Error>>()
                .map(|_| ()));

            try!(Unified::new(self).write_rust(protocol, writer));
        }
        try!(writeln!(writer, "}}"));

        Ok(())
    }
}