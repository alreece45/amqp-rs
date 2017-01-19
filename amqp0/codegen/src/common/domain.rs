// Copyright 2016-17 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct DomainMapper {
    spec: &'static ::specs::Spec,
}

impl DomainMapper {
    pub fn from_spec(spec: &'static ::specs::Spec) -> Self {
        DomainMapper {
            spec: spec
        }
    }

    pub fn map(&self, domain: &str) -> Domain {
        let mut ty = domain;
        while let Some(mapping) = self.spec.domain(ty) {
            // detect identity mappings
            if ty == mapping {
                break;
            }
            ty = mapping;
        }
        Domain::new(ty)
    }
}

#[derive(Debug, Clone, Hash, PartialEq)]
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

    pub fn empty_encoded_value(&self) -> &'static str {
        match *self {
            Domain::Bit | Domain::ShortString | Domain::Octet => "0u8",
            Domain::Short => "0u16",
            Domain::Long | Domain::LongString | Domain::Table | Domain::Content  => "0u32",
            Domain::LongLong | Domain::Timestamp => "0u64",
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

    pub fn is_owned(&self) -> bool {
        match (self.is_copy(), self) {
            (false, &Domain::Table) => true,
            _ => false,
        }
    }

    pub fn borrowed_type(&self) -> &'static str {
        match *self {
            Domain::ShortString => "str",
            Domain::LongString | Domain::Content => "[u8]",
            Domain::Table => "::field::TableEntries<'a>",
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
            Domain::Table => "::field::TableEntries<'a>",
            Domain::Content => "Vec< ::field::Value >",
        }
    }

    pub fn num_bits_static(&self) -> usize {
        match *self {
            Domain::Bit => 1,
            Domain::Octet | Domain::Content | Domain::ShortString => 8,
            Domain::Short => 16,
            Domain::Long => 32,
            Domain::LongLong | Domain::Timestamp => 64,
            Domain::LongString | Domain::Table => 32,
        }
    }

    pub fn dynamic_bit_method(&self) -> Option<&'static str> {
        match *self {
            Domain::ShortString | Domain::LongString | Domain::Content => Some("len"),
            Domain::Table => Some("Encodable::encoded_size"),
            _ => None,
        }
    }

    pub fn option_type(&self) -> &'static str {
        match *self {
            Domain::Bit => "Option<bool>",
            Domain::Octet => "Option<u8>",
            Domain::Short => "Option<u16>",
            Domain::Long => "Option<u32>",
            Domain::LongLong | Domain::Timestamp => "Option<u64>",
            Domain::ShortString => "Option<&str>",
            Domain::LongString => "Option<Vec<u8>>",

            // table entries and content are... present
            Domain::Content => "Vec<u8>",
            Domain::Table => "::field::TableEntries<'a>",
        }
    }

    pub fn is_option_type(&self) -> bool {
        match *self {
            Domain::Content | Domain::Table => false,
            _ => true,
        }
    }

    pub fn nom_parser(&self) -> &'static str {
        match *self {
            Domain::Bit | Domain::Octet => "be_u8",
            Domain::Short => "be_u16",
            Domain::Long => "be_u32",
            Domain::LongLong | Domain::Timestamp => "be_u64",
            Domain::ShortString => "call!(::common::shortstr)",
            Domain::LongString => "call!(::common::longstr)",
            Domain::Table => "apply!(<::primitives::field::TableEntries as ::NomBytes>::nom_bytes, pool)",
            //AmqpType::Content => "::amqp0::value::Content::from_bytes",
            Domain::Content => "length_bytes!(be_u32)",
        }
    }

    pub fn cow_definition<S>(&self, lifetime: S) -> Cow<'static, str>
        where S: AsRef<str>
    {
        if self.is_copy() || self.is_owned() {
            Cow::Borrowed(self.borrowed_type())
        }
        else {
            format!("::std::borrow::Cow<'{}, {}>", lifetime.as_ref(), self.borrowed_type()).into()
        }
    }
}