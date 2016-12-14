// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;

use nom::{be_f32, be_f64};
use nom::{be_u8, be_u16, be_u32, be_u64};
use nom::{be_i8, be_i16, be_i32, be_i64};
use nom::IResult;
use primitives::field::{List, TableEntries, Value};

use common::{longstr, shortstr};
use pool::ParserPool;
use NomBytes;

impl<'a> NomBytes<'a> for Value<'a> {

    #[allow(unused_variables, cyclomatic_complexity)]
    fn nom_bytes<'b, P>(input: &'a [u8], pool: &'b mut P) -> IResult<&'a [u8], Self>
        where P: ParserPool
    {
        switch!(input, take!(1),
            b"s" => map!(shortstr, |s: &'a str| Value::ShortString(Cow::Borrowed(s))) |
            b"S" => map!(longstr, |b: &'a [u8]| Value::LongString(Cow::Borrowed(b))) |
            b"t" => map!(be_u8, |b| Value::Bool(b != 0)) |
            b"b" => map!(be_i8, Value::I8) |
            b"B" => map!(be_u8, Value::U8) |
            b"u" => map!(be_i16, Value::I16) |
            b"U" => map!(be_u16, Value::U16) |
            b"i" => map!(be_i32, Value::I32) |
            b"I" => map!(be_u32, Value::U32) |
            b"l" => map!(be_i64, Value::I64) |
            b"L" => map!(be_u64, Value::U64) |
            b"f" => map!(be_f32, Value::F32) |
            b"d" => map!(be_f64, Value::F64) |
            b"D" => map!(tuple!(be_u8, be_u32), |(scale, value): (u8, u32)| Value::Decimal(scale, value)) |
            b"T" => map!(be_u64, Value::Timestamp) |
            b"V" => value!(Value::Void) |
            b"F" => map!(call!(TableEntries::nom_bytes, pool), Value::Table) |
            b"A" => map!(call!(List::nom_bytes, pool), Value::List)
        )
    }
}