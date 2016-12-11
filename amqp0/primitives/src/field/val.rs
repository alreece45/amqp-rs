// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//!
//! Non-owned dynamic AMQP variable.
//!
//! This is mostly used in lower-level protocol handlers. The buffer
//!
//! *May* own the Vec<> of Val references, but they may contain non-owned data.
//!
//! This value is useful for parsing a buffer, reusing the slices in the buffer after parsing.
//! If ownership is desired, use the to_owned() to get a Value -- which owns all references
//! (with a possible cost on the heap)
//!
//! From<...> is implemented for most primitives.
//!
//! Vec<_> and &[_] are not implemented, as Vec<u8> and %[u8] could be interpreted as
//! either as List of Value::U8 or as a LongString.
//!
//! From<...> is also implemented for String/&str, but with some caveats:
//!
//! The maximum ShortString length is 255 octets/bytes. You can create your own ShortString that exceeds
//! this length and will only get an error when trying to send it. into() on &str and String will map
//! to a ShortString when under 255 octets allows it, and a LongString otherwise. The number of octets
//! may be more than the number of characters (.len() vs .as_bytes().len())
//!
//! The maximum LongString length is 4294967296 octets/bytes. Again, you can create your own LongString
//! that exceeds this limit, receiving an error when you try to send it. into() on &str and String will
//! map to a LongString when its length exceeds 255 octets.
//!

use std::borrow::Cow;

use super::MAX_SHORTSTR_LEN;
use super::{Table, List, Value};

#[derive(Debug, Clone, PartialEq)]
pub enum Val<'a> {
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
    Table(Table<'a>),
}

impl<'a> Val<'a> {
}

macro_rules! impl_from_primitive {
    ($from:ty, $to:expr) => {
        impl From<$from> for Val<'static> {
            fn from(val: $from) -> Self {
                $to(val)
            }
        }
    }
}

impl_from_primitive!(bool, Val::Bool);
impl_from_primitive!(u8, Val::U8);
impl_from_primitive!(i8, Val::I8);
impl_from_primitive!(u16, Val::U16);
impl_from_primitive!(i16, Val::I16);

impl_from_primitive!(u32, Val::U32);
impl_from_primitive!(i32, Val::I32);
impl_from_primitive!(f32, Val::F32);

impl_from_primitive!(u64, Val::U64);
impl_from_primitive!(i64, Val::I64);
impl_from_primitive!(f64, Val::F64);

impl From<()> for Val<'static> {
    fn from(_: ()) -> Val<'static> {
        Val::Void
    }
}

impl<'a> From<Table<'a>> for Val<'a> {
    fn from(val: Table<'a>) -> Self {
        Val::Table(val)
    }
}

impl<'a> From<List<'a>> for Val<'a> {
    fn from(val: List<'a>) -> Self {
        Val::List(val)
    }
}

impl<'a> From<String> for Val<'a> {
    fn from(val: String) -> Self {
        if val.len() <= MAX_SHORTSTR_LEN {
            Val::ShortString(val.into())
        }
        else {
            Val::LongString(val.into_bytes().into())
        }
    }
}

impl<'a> From<&'a str> for Val<'a> {
    fn from(val: &'a str) -> Self {
        if val.len() <= MAX_SHORTSTR_LEN {
            Val::ShortString(val.into())
        }
        else {
            Val::LongString(val.as_bytes().into())
        }
    }
}

impl<'a> From<Cow<'a, str>> for Val<'a> {
    fn from(val: Cow<'a, str>) -> Self {
        if val.len() <= MAX_SHORTSTR_LEN {
            Val::ShortString(val)
        }
        else {
            match val {
                Cow::Borrowed(s) => s.into(),
                Cow::Owned(s) => s.into(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;
    use super::{Val, List, Table};

    macro_rules! test_from_primitive {
        ($name:ident, $from:ty, $to:expr, $value:expr) => {
            #[test]
            fn $name() {
                let primitive: $from = $value;
                let val: Val = primitive.into();

                assert_eq!(val, $to($value));
            }
        }
    }

    test_from_primitive!(test_from_bool, bool, Val::Bool, true);

    test_from_primitive!(test_from_u8, u8, Val::U8, 1);
    test_from_primitive!(test_from_i8, i8, Val::I8, 2);
    test_from_primitive!(test_from_u16, u16, Val::U16, 3);
    test_from_primitive!(test_from_i16, i16, Val::I16, 4);
    test_from_primitive!(test_from_u32, u32, Val::U32, 5);
    test_from_primitive!(test_from_i32, i32, Val::I32, 6);
    test_from_primitive!(test_from_u64, u64, Val::U64, 7);
    test_from_primitive!(test_from_i64, i64, Val::I64, 8);

    test_from_primitive!(test_from_f32, f32, Val::F32, 99.99);
    test_from_primitive!(test_from_f64, f64, Val::F64, 999.99);

    #[test]
    fn test_from_empty_tuple() {
        assert_eq!(Val::Void, ().into())
    }

    #[test]
    fn test_from_short_string() {
        let val_str: Val = "short".into();
        assert_eq!(val_str, Val::ShortString(Cow::Borrowed("short")));

        let val_string: Val = "owned".to_owned().into();
        match val_string {
            Val::ShortString(Cow::Owned(ref s)) if s == "owned" => (),
            v => panic!("expected lshortstring val, got {:?}", v),
        }

        let cow_str: Cow<str> = "cow".into();
        let val_cow: Val = cow_str.into();
        assert_eq!(val_cow, Val::ShortString(Cow::Borrowed("cow")));
    }

    #[test]
    fn test_from_long_string() {
        let long = (0..1024).map(|_| "X").collect::<String>();
        let val_str: Val = long.as_str().into();
        assert_eq!(val_str, Val::LongString(Cow::Borrowed(long.as_str().as_bytes())));

        let val_string: Val = long.clone().into();
        match val_string {
            Val::LongString(Cow::Owned(ref s)) if s[..] == long.as_str().as_bytes()[..] => (),
            v => panic!("expected longstring val, got {:?}", v),
        }

        let cow_str: Cow<str> = long.as_str().into();
        let val_cow: Val = cow_str.into();
        assert_eq!(val_cow, Val::LongString(Cow::Borrowed(long.as_str().as_bytes())));
    }

    #[test]
    fn test_from_list() {
        let val: Val = List::new().into();
        match val {
            Val::List(_) => (),
            v => panic!("expected list val, got {:?}", v),
        }
    }

    #[test]
    fn test_from_table() {
        let val: Val = Table::new().into();
        match val {
            Val::Table(_) => (),
            v => panic!("expected table val, got {:?}", v),
        }
    }
}
