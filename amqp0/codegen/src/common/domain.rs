// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;
use std::collections::BTreeMap;

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

#[doc(hidden)]
pub enum Domain {
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

impl Domain {
    pub fn new(name: &str) -> Self {
        match name {
            "bit" => Domain::Bit,
            "octet" => Domain::Octet,
            "short" => Domain::Short,
            "long" => Domain::Long,
            "longlong" => Domain::LongLong,
            "timestamp" => Domain::Timestamp,
            "shortstr" => Domain::ShortString,
            "longstr" => Domain::LongString,
            "table" => Domain::Table,
            "content" => Domain::Content,
            _ => panic!("Unimplemented type: {}", name)
        }
    }

    pub fn is_copy(&self) -> bool {
        match *self {
            Domain::Bit | Domain::Octet
            | Domain::Short | Domain::Long
            | Domain::LongLong | Domain::Timestamp => true,
            _ => false,
        }
    }
    pub fn borrowed_type(&self) -> &'static str {
        match *self {
            Domain::ShortString => "str",
            Domain::LongString | Domain::Content => "[u8]",
            _ => self.owned_type(),
        }
    }
    pub fn owned_type(&self) -> &'static str {
        match *self {
            Domain::Bit => "bool",
            Domain::Octet => "u8",
            Domain::Short => "u16",
            Domain::Long => "u32",
            Domain::LongLong | Domain::Timestamp => "u64",
            Domain::ShortString => "String",
            Domain::LongString => "Vec<u8>",
            Domain::Table => "::field::Table<'a>",
            Domain::Content => "::field::List<'a>",
        }
    }

    pub fn num_bits_fixed(&self) -> usize {
        match *self {
            Domain::Bit => 1,
            Domain::Octet | Domain::ShortString | Domain::Content => 8,
            Domain::Short | Domain::LongString => 16,
            Domain::Long => 32,
            Domain::LongLong | Domain::Timestamp => 64,
            Domain::Table => 0,
        }
    }

    pub fn dynamic_bit_method(&self) -> Option<&'static str> {
        match *self {
            Domain::ShortString | Domain::LongString | Domain::Content => Some("len"),
            Domain::Table => Some("amqp_size"),
            _ => None,
        }
    }

    pub fn nom_parser(&self) -> &'static str {
        match *self {
            Domain::Bit | Domain::Octet => "be_u8",
            Domain::Short => "be_u16",
            Domain::Long => "be_u32",
            Domain::LongLong | Domain::Timestamp => "be_u64",
            Domain::ShortString => "call!(::nom::shortstr)",
            Domain::LongString => "call!(::nom::longstr)",
            Domain::Table => "call!(::field::Table::nom_bytes)",
            //AmqpType::Content => "::amqp0::value::Content::from_bytes",
            Domain::Content => "length_bytes!(be_u32)",
        }
    }

    pub fn cow_definition<S>(&self, lifetime: S) -> Cow<'static, str>
        where S: AsRef<str>
    {
        if self.is_copy() {
            Cow::Borrowed(self.borrowed_type())
        }
            else {
                format!("::std::borrow::Cow<'{}, {}>", lifetime.as_ref(), self.borrowed_type()).into()
            }
    }
}