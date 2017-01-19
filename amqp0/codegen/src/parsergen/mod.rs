// Copyright 2016-17 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub mod nom;

use std::borrow::Cow;
use std::mem;

use common::{self, Domain};

type Field = common::Field;

///
/// The values for multiple fields can be packed into one byte.
///
/// This structure is used when When a byte/chunk represents multiple fields. Currently
/// its only used for bools/bits, but may be expanded if needed
///
#[derive(Debug)]
enum FieldChunk<'a> {
    Field(&'a str, Option<&'a str>), // parser, var_name
    Flags(u8, Vec<bool>, Option<Cow<'a, str>>),   // flag number, num_bits, is_used
}

impl<'a> FieldChunk<'a> {
    pub fn from_field(field: &'a Field, flag_num: u8) -> Self {
        if field.is_reserved() {
            match *field.ty() {
                Domain::Bit => FieldChunk::Flags(flag_num, vec![!field.is_reserved()], None),
                _ => FieldChunk::Field(field.ty().nom_parser(), None),
            }
        }
        else {
            let name = field.var_name();
            match *field.ty() {
                Domain::Bit => FieldChunk::Flags(flag_num, vec![!field.is_reserved()], Some(name.as_str().into())),
                _ => FieldChunk::Field(field.ty().nom_parser(), Some(name.as_str())),
            }
        }
    }

    pub fn add_field(&mut self, field: &'a Field) -> bool {
        match *self {
            FieldChunk::Flags(flag_num, ref mut bits, ref mut name) if bits.len() <= 8 => {
                bits.push(!field.is_reserved());

                if bits.len() > 1 {
                    if let Some(Cow::Borrowed(_)) = *name {
                        mem::replace(name, Some(format!("flag{}", flag_num).into()));
                    }
                }
                true
            }
            _ => false,
        }
    }

    pub fn capture_name(&self) -> Option<&str> {
        match *self {
            FieldChunk::Flags(_, _, ref name) => name.as_ref().map(|n| n.as_ref()),
            FieldChunk::Field(_, name) => name,
        }
    }

    pub fn arg_names(&self) -> Vec<Cow<str>> {
        match *self {
            FieldChunk::Field(_, Some(name)) => vec![(*name).into()],
            FieldChunk::Flags(_, ref bits, Some(ref name)) => {
                if bits.len() > 1 {
                    bits.iter()
                        .filter(|f| **f)
                        .enumerate()
                        .map(|(bit, _): (usize, &bool)| -> Cow<str> { format!("{}.{}", name, bit).into() })
                        .collect()
                }
                    else {
                        vec![Cow::Borrowed(&*name)]
                    }
            },
            _ => vec![],
        }
    }

    pub fn nom_parser(&self) -> Cow<'a, str> {
        const BOOL_MAPPER: &'static str = "call!(::common::bool_bit)";
        match *self {
            FieldChunk::Field(parser, _) => parser.into(),
            FieldChunk::Flags(_, ref bits, _) => {
                if bits.len()> 1 {
                    let collectors = bits.iter()
                        .map(|_| BOOL_MAPPER)
                        .collect::<Vec<_>>()
                        .join(",\n");
                    format!("bits!(tuple!(\n{}\n))", collectors).into()
                }
                    else {
                        format!("bits!({})", BOOL_MAPPER).into()
                    }
            },
        }
    }
}