// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;
use std::time::{SystemTime, UNIX_EPOCH};

use Encodable;
use super::MAX_SHORTSTR_LEN;
use super::{TableEntries, List};

///
/// Basic "field" that essentially represents dynamic-types in the AMQP protocol.
///
#[derive(Debug, PartialEq, Clone)]
pub enum Value<'a> {
    Void,
    Bool(bool),

    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),

    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),

    F32(f32),
    F64(f64),

    Decimal(u8, u32),
    ShortString(Cow<'a, str>),
    LongString(Cow<'a, [u8]>),
    Timestamp(u64),

    List(List<'a>),
    Table(TableEntries<'a>),
}

impl<'a> Value<'a> {

    pub fn id(&self) -> char {
        match *self {
            Value::Table(_)       => 'F',
            Value::Bool(_)        => 't',
            Value::I8(_)          => 'b',
            Value::U8(_)          => 'B',
            Value::I16(_)         => 'U',
            Value::U16(_)         => 'u',
            Value::I32(_)         => 'I',
            Value::U32(_)         => 'i',
            Value::I64(_)         => 'L',
            Value::U64(_)         => 'l',
            Value::F32(_)         => 'd',
            Value::F64(_)         => 'f',
            Value::Timestamp(_)   => 'T',
            Value::Decimal(_, _)  => 'D',
            Value::ShortString(_) => 's',
            Value::LongString(_)  => 'S',
            Value::List(_)        => 'A',
            Value::Void           => 'V',
        }
    }

    pub fn into_static(self) -> Value<'static> {
        match self {
            Value::Bool(val)           => Value::Bool(val),
            Value::I8(val)             => Value::I8(val),
            Value::U8(val)             => Value::U8(val),
            Value::I16(val)            => Value::I16(val),
            Value::U16(val)            => Value::U16(val),
            Value::I32(val)            => Value::I32(val),
            Value::U32(val)            => Value::U32(val),
            Value::I64(val)            => Value::I64(val),
            Value::U64(val)            => Value::U64(val),
            Value::F32(val)            => Value::F32(val),
            Value::F64(val)            => Value::F64(val),
            Value::Timestamp(val)      => Value::Timestamp(val),
            Value::Decimal(s, b)       => Value::Decimal(s, b),
            Value::ShortString(string) => Value::ShortString(Cow::Owned(string.into_owned())),
            Value::LongString(bytes)   => Value::LongString(Cow::Owned(bytes.into_owned())),
            Value::List(list)          => Value::List(list.into_static()),
            Value::Table(entries)      => Value::Table(entries.into_static()),
            Value::Void                => Value::Void,
        }
    }
}

impl<'a> Encodable for Value<'a> {
    fn encoded_size(&self) -> usize {
        match *self {
            Value::Void => 0,
            Value::Bool(_) | Value::I8(_)  | Value::U8(_)  => 1,
            Value::I16(_) | Value::U16(_) => 2,
            Value::I32(_) | Value::U32(_) | Value::F32(_) | Value::Timestamp(_) => 4,
            Value::I64(_) | Value::U64(_) | Value::F64(_) => 8,
            Value::Decimal(_, _) => 3,
            Value::ShortString(ref value) => value.len(),
            Value::LongString(ref value) => value.len(),
            Value::List(ref entries) => entries.iter().map(|e| e.encoded_size()).sum(),
            Value::Table(ref table) => table.encoded_size(),
        }
    }
}

impl<'a> From<TableEntries<'a>> for Value<'a> {
    fn from(entries: TableEntries<'a>) -> Self {
        Value::Table(entries)
    }
}

impl From<SystemTime> for Value<'static> {
    fn from(time: SystemTime) -> Value<'static> {
        let result = time.duration_since(UNIX_EPOCH);
        let duration = match result {
            Ok(duration) => duration,
            Err(e) => e.duration(),
        };
        Value::Timestamp(duration.as_secs())
    }
}

macro_rules! impl_from_primitive {
    ($from:ty, $to:expr) => {
        impl From<$from> for Value<'static> {
            fn from(value: $from) -> Self {
                $to(value)
            }
        }
    }
}

impl_from_primitive!(bool, Value::Bool);
impl_from_primitive!(u8, Value::U8);
impl_from_primitive!(i8, Value::I8);
impl_from_primitive!(u16, Value::U16);
impl_from_primitive!(i16, Value::I16);

impl_from_primitive!(u32, Value::U32);
impl_from_primitive!(i32, Value::I32);
impl_from_primitive!(f32, Value::F32);

impl_from_primitive!(u64, Value::U64);
impl_from_primitive!(i64, Value::I64);
impl_from_primitive!(f64, Value::F64);

impl From<()> for Value<'static> {
    fn from(_: ()) -> Self {
        Value::Void
    }
}


impl<'a> From<List<'a>> for Value<'a> {
    fn from(value: List<'a>) -> Self {
        Value::List(value)
    }
}

impl From<String> for Value<'static> {
    fn from(value: String) -> Self {
        if value.len() <= MAX_SHORTSTR_LEN {
            Value::ShortString(Cow::Owned(value))
        }
        else {
            Value::LongString(Cow::Owned(value.into_bytes()))
        }
    }
}

impl<'a> From<&'a str> for Value<'a> {
    fn from(value: &'a str) -> Self {
        if value.len() <= MAX_SHORTSTR_LEN {
            Value::ShortString(Cow::Borrowed(value))
        }
        else {
            Value::LongString(Cow::Borrowed(value.as_bytes()))
        }
    }
}

impl<'a> From<Cow<'a, str>> for Value<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        if value.len() <= MAX_SHORTSTR_LEN {
            Value::ShortString(value)
        }
        else {
            match value {
                Cow::Borrowed(s) => s.into(),
                Cow::Owned(s) => s.into(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;
    use field::{Value, List, TableEntries};

    macro_rules! test_from_primitive {
        ($name:ident, $from:ty, $to:expr, $value:expr) => {
            #[test]
            fn $name() {
                let primitive: $from = $value;
                let value: Value = primitive.into();

                assert_eq!(value, $to($value));
            }
        }
    }

    test_from_primitive!(test_from_bool, bool, Value::Bool, true);

    test_from_primitive!(test_from_u8, u8, Value::U8, 1);
    test_from_primitive!(test_from_i8, i8, Value::I8, 2);
    test_from_primitive!(test_from_u16, u16, Value::U16, 3);
    test_from_primitive!(test_from_i16, i16, Value::I16, 4);
    test_from_primitive!(test_from_u32, u32, Value::U32, 5);
    test_from_primitive!(test_from_i32, i32, Value::I32, 6);
    test_from_primitive!(test_from_u64, u64, Value::U64, 7);
    test_from_primitive!(test_from_i64, i64, Value::I64, 8);

    test_from_primitive!(test_from_f32, f32, Value::F32, 99.99);
    test_from_primitive!(test_from_f64, f64, Value::F64, 999.99);

    #[test]
    fn test_from_empty_tuple() {
        assert_eq!(Value::Void, ().into())
    }


    #[test]
    fn test_from_short_string() {
        let val_str: Value = "short".into();
        assert_eq!(val_str, Value::ShortString(Cow::Borrowed("short")));

        let val_string: Value = "owned".to_owned().into();
        match val_string {
            Value::ShortString(Cow::Owned(ref s)) if s == "owned" => (),
            v => panic!("expected lshortstring val, got {:?}", v),
        }

        let cow_str: Cow<str> = "cow".into();
        let val_cow: Value = cow_str.into();
        assert_eq!(val_cow, Value::ShortString(Cow::Borrowed("cow")));
    }

    #[test]
    fn test_from_long_string() {
        let long = (0..1024).map(|_| "X").collect::<String>();
        let val_str: Value = long.as_str().into();
        assert_eq!(val_str, Value::LongString(Cow::Borrowed(long.as_str().as_bytes())));

        let val_string: Value = long.clone().into();
        match val_string {
            Value::LongString(Cow::Owned(ref s)) if s[..] == long.as_str().as_bytes()[..] => (),
            v => panic!("expected longstring val, got {:?}", v),
        }

        let cow_str: Cow<str> = long.as_str().into();
        let val_cow: Value = cow_str.into();
        assert_eq!(val_cow, Value::LongString(Cow::Borrowed(long.as_str().as_bytes())));
    }

    #[test]
    fn test_from_list() {
        let value: Value = List::new().into();
        match value {
            Value::List(_) => (),
            v => panic!("expected list value, got {:?}", v),
        }
    }

    #[test]
    fn test_from_table_entries() {
        let value: Value = TableEntries::new().into();
        match value {
            Value::Table(_) => (),
            v => panic!("expected table value, got {:?}", v),
        }
    }
}
