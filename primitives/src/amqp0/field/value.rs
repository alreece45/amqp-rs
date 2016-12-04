// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//!
//! Basic "field" that essentially represents dynamic-types in the AMQP protocol.
//!
//! There are two main types to represent "field values": Val and Value. Val is unsized (without
//! ownership or heap allocations) and one is Sized/Owned -- much like the difference between
//! str/String, Path/PathBuf, Cstr/CStr, and OsStr/OsString.
//!

use std::borrow::Cow;
use std::time::{SystemTime, UNIX_EPOCH};
use super::MAX_SHORTSTR_LEN;
use super::{TableBuf, ListBuf, Val};

/// Value is the "owned" version of Val
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
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
    ShortString(String),
    LongString(Vec<u8>),
    Timestamp(u64),

    List(ListBuf),
    Table(TableBuf),
}

impl<'a> From<Val<'a>> for Value {
    fn from(val: Val<'a>) -> Value {
        val.into_owned()
    }
}

impl From<TableBuf> for Value {
    fn from(table: TableBuf) -> Self {
        Value::Table(table)
    }
}

impl From<SystemTime> for Value {
    fn from(time: SystemTime) -> Value {
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
        impl From<$from> for Value {
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

impl From<()> for Value {
    fn from(_: ()) -> Self {
        Value::Void
    }
}


impl From<ListBuf> for Value {
    fn from(value: ListBuf) -> Self {
        Value::List(value)
    }
}

impl<'a> From<String> for Value {
    fn from(value: String) -> Self {
        if value.len() <= MAX_SHORTSTR_LEN {
            Value::ShortString(value)
        }
        else {
            Value::LongString(value.into_bytes())
        }
    }
}

impl<'a> From<&'a str> for Value {
    fn from(value: &'a str) -> Self {
        if value.len() <= MAX_SHORTSTR_LEN {
            Value::ShortString(value.to_owned())
        }
        else {
            Value::LongString(value.as_bytes().to_vec())
        }
    }
}

impl<'a> From<Cow<'a, str>> for Value {
    fn from(value: Cow<'a, str>) -> Self {
        if value.len() <= MAX_SHORTSTR_LEN {
            Value::ShortString(value.into_owned())
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
    use super::{Value, ListBuf, TableBuf};

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
        let str = "test5555".to_owned();
        let value_str: Value = str.as_str().into();
        assert_eq!(value_str, Value::ShortString(str.clone()));

        let value_string: Value = str.clone().into();
        assert_eq!(value_string, Value::ShortString(str.clone()));

        let cow_str: Cow<str> = str.clone().into();
        let value_cow: Value = cow_str.into();
        assert_eq!(value_cow, Value::ShortString(str.clone()));
    }

    #[test]
    fn test_from_long_string() {
        let long = (0..1024).map(|_| "X").collect::<String>();
        let expected = long.clone().into_bytes();
        let value_str: Value = long.as_str().into();
        assert_eq!(value_str, Value::LongString(expected.clone()));

        let value_string: Value = long.clone().into();
        assert_eq!(value_string, Value::LongString(expected.clone()));

        let cow_str: Cow<str> = long.as_str().into();
        let value_cow: Value = cow_str.into();
        assert_eq!(value_cow, Value::LongString(expected.clone()));
    }

    #[test]
    fn test_from_list() {
        let value: Value = ListBuf::new().into();
        match value {
            Value::List(_) => (),
            v => panic!("expected list value, got {:?}", v),
        }
    }

    #[test]
    fn test_from_table() {
        let value: Value = TableBuf::new().into();
        match value {
            Value::Table(_) => (),
            v => panic!("expected table value, got {:?}", v),
        }
    }
}
