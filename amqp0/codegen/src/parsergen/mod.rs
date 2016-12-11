// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod nom;

pub use self::nom::WriteNomImplementation;

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
                Domain::Bit => Part::Flags(flag_num, vec![!field.is_reserved()], None),
                _ => Part::Field(field.ty().nom_parser(), None),
            }
        }
        else {
            let name = field.var_name();
            match *field.ty() {
                Domain::Bit => Part::Flags(flag_num, vec![!field.is_reserved()], Some(name.into())),
                _ => Part::Field(field.ty().nom_parser(), Some(name)),
            }
        }
    }

    pub fn add_field(&mut self, field: &'a Field) -> bool {
        match *self {
            Part::Flags(flag_num, ref mut bits, ref mut name) if bits.len() <= 8 => {
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
            Part::Flags(_, _, ref name) => name.as_ref().map(|n| n.as_ref()),
            Part::Field(_, name) => name,
        }
    }

    pub fn arg_names(&self) -> Vec<Cow<str>> {
        match *self {
            Part::Field(_, Some(name)) => vec![(*name).into()],
            Part::Flags(_, ref bits, Some(ref name)) => {
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
        const BOOL_MAPPER: &'static str = "call!(::amqp0::nom::bool_bit)";
        match *self {
            Part::Field(parser, _) => parser.into(),
            Part::Flags(_, ref bits, _) => {
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