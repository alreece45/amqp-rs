// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;
use std::collections::BTreeMap;

use inflections::Inflect;

mod specs;
mod spec;
mod nom;

pub use self::spec::SpecWriter;
pub use self::nom::ParserWriter;

pub struct DomainMapper<'a> {
    domains: &'a BTreeMap<&'a str, &'a str>,
}

impl<'a> DomainMapper<'a> {
    pub fn new(domains: &'a BTreeMap<&str, &str>) -> DomainMapper<'a> {
        DomainMapper {
            domains: domains
        }
    }
    pub fn map(&self, domain: &'a str) -> &'a str {
        let mut ty = domain;
        while let Some(mapping) = self.domains.get(ty) {
            // detect identity mappings
            if ty == *mapping {
                break;
            }
            ty = mapping;
        }
        ty
    }
}

pub use self::specs::Specs;

pub fn write_common<W>(writer: &mut W, specs: &Specs) -> io::Result<()>
    where W: io::Write
{
    // ensure that class ids remain consistent accross the specs
    specs.assert_name_indexes_consistent();

    try!(write_common_classes(writer, specs));
    try!(write_common_methods(writer, specs));

    Ok(())
}

pub fn write_common_classes<W>(writer: &mut W, specs: &Specs) -> io::Result<()>
    where W: io::Write
{
    try!(writeln!(writer, "//"));
    try!(writeln!(writer, "// Index values for classes shared among multiple specs"));
    try!(writeln!(writer, "//"));
    try!(writeln!(writer, "// Sometimes, the index value is repeated in different classes, but these are not reused"));
    try!(writeln!(writer, "// within a single protocol"));
    try!(writeln!(writer, "//"));
    try!(writeln!(writer, "// Classes are currently only considered common if they are used in more than one"));
    try!(writeln!(writer, "// spec. This behavior *may* change in the future as more specs are added."));
    try!(writeln!(writer, "//"));

    let common_classes = {
        let mut clasess = specs.common_classes().into_iter().collect::<Vec<_>>();
        clasess.sort_by(|&(_, a), &(_, b)| a.cmp(&b));
        clasess
    };

    for (class_name, index) in common_classes {
        let constant_class = class_name.to_constant_case();
        try!(writeln!(writer, "pub const CLASS_{}: u16 = {};", constant_class, index));
    }
    try!(writeln!(writer, ""));

    Ok(())
}

pub fn write_common_methods<W>(writer: &mut W, specs: &Specs) -> io::Result<()>
    where W: io::Write
{
    try!(writeln!(writer, "//"));
    try!(writeln!(writer, "// Index values for methods common among the different specs"));
    try!(writeln!(writer, "//"));
    try!(writeln!(writer, "// Methods are only considered common when:"));
    try!(writeln!(writer, "//"));
    try!(writeln!(writer, "//   * The index value is consistent across all of the specs"));
    try!(writeln!(writer, "//   * The method is used in more than one spec"));
    try!(writeln!(writer, "//"));
    try!(writeln!(writer, "// This may change in the future-- in that case, methods *may* be removed, or"));
    try!(writeln!(writer, "// one of the requirements may be relaxed."));
    try!(writeln!(writer, "//"));

    let common_methods = {
        let mut methods = specs.common_methods().into_iter().collect::<Vec<_>>();
        methods.sort_by(|&(a, _), &(b, _)| a.cmp(b));
        methods
    };

    for (class_name, methods) in common_methods {
        let methods = {
            let mut methods = methods.into_iter().collect::<Vec<_>>();
            methods.sort_by(|&(_, a), &(_, b)| a.cmp(&b));
            methods
        };

        let constant_class = class_name.to_constant_case();

        for (method_name, index) in methods {
            let constant_method = method_name.to_constant_case();

            if constant_method != "_" {
                try!(writeln!(writer, "pub const METHOD_{}_{}: u16 = {};", constant_class, constant_method, index));
            }
        }
        try!(writeln!(writer, ""));
    }

    Ok(())
}

enum AmqpType {
    Bit,
    Octet,
    Short,
    Long,
    LongLong,
    Timestamp,
    ShortString,
    LongString,
    Table,
    Content,
}

impl AmqpType {
    pub fn new(name: &str) -> Self {
        match name {
            "bit" => AmqpType::Bit,
            "octet" => AmqpType::Octet,
            "short" => AmqpType::Short,
            "long" => AmqpType::Long,
            "longlong" => AmqpType::LongLong,
            "timestamp" => AmqpType::Timestamp,
            "shortstr" => AmqpType::ShortString,
            "longstr" => AmqpType::LongString,
            "table" => AmqpType::Table,
            "content" => AmqpType::Content,
            _ => panic!("Unimplemented type: {}", name)
        }
    }

    fn is_copy(&self) -> bool {
        match *self {
            AmqpType::Bit | AmqpType::Octet
             | AmqpType::Short | AmqpType::Long
             | AmqpType::LongLong | AmqpType::Timestamp => true,
            _ => false,
        }
    }
    fn borrowed_type(&self) -> &'static str {
        match *self {
            AmqpType::ShortString => "str",
            AmqpType::LongString => "[u8]",
            AmqpType::Content => "[u8]",
            _ => self.owned_type(),
        }
    }
    fn owned_type(&self) -> &'static str {
        match *self {
            AmqpType::Bit => "bool",
            AmqpType::Octet => "u8",
            AmqpType::Short => "u16",
            AmqpType::Long => "u32",
            AmqpType::LongLong => "u64",
            AmqpType::Timestamp => "u64",
            AmqpType::ShortString => "String",
            AmqpType::LongString => "Vec<u8>",
            AmqpType::Table => "::amqp0::field::Table<'a>",
            AmqpType::Content => "::amqp0::field::List<'a>",
        }
    }

    fn num_bits_fixed(&self) -> usize {
        match *self {
            AmqpType::Bit => 1,
            AmqpType::Octet | AmqpType::ShortString | AmqpType::Content => 8,
            AmqpType::Short | AmqpType::LongString => 16,
            AmqpType::Long => 32,
            AmqpType::LongLong => 64,
            AmqpType::Timestamp => 64,
            AmqpType::Table => 0,
        }
    }

    fn dynamic_bit_method(&self) -> Option<&'static str> {
        match *self {
            AmqpType::ShortString | AmqpType::LongString | AmqpType::Content => Some("len"),
            AmqpType::Table => Some("amqp_size"),
            _ => None,
        }
    }

    pub fn nom_parser(&self) -> &'static str {
        match *self {
            AmqpType::Bit => "be_u8",
            AmqpType::Octet => "be_u8",
            AmqpType::Short => "be_u16",
            AmqpType::Long => "be_u32",
            AmqpType::LongLong => "be_u64",
            AmqpType::Timestamp => "be_u64",
            AmqpType::ShortString => "call!(::amqp0::nom::shortstr)",
            AmqpType::LongString => "call!(::amqp0::nom::longstr)",
            AmqpType::Table => "call!(::amqp0::field::Table::nom_bytes)",
            //AmqpType::Content => "::amqp0::value::Content::from_bytes",
            AmqpType::Content => "length_bytes!(be_u32)",
        }
    }
}