
use std::borrow::Cow;
use std::mem;
use std::str;

use nom::IResult;
use nom::{be_f32, be_f64};
use nom::{be_u8, be_u16, be_u32, be_u64};
use nom::{be_i8, be_i16, be_i32, be_i64};

use amqp0::nom::{shortstr, longstr};
use amqp0::nom::{NomBytes, ParserPool};
use super::{List, Table, Val};

impl<'a> NomBytes<'a> for Val<'a> {

    #[allow(unused_variables)]
    fn nom_bytes<'b, P>(input: &'a [u8], pool: &'b mut P) -> IResult<&'a [u8], Self>
        where P: ParserPool
    {
        let result = switch!(input, take!(1),
            b"s" => map!(shortstr, |s: &'a str| Val::ShortString(Cow::Borrowed(s))) |
            b"S" => map!(longstr, |b: &'a [u8]| Val::LongString(Cow::Borrowed(b))) |
            b"t" => map!(be_u8, |b| Val::Bool(b != 0)) |
            b"b" => map!(be_i8, Val::I8) |
            b"B" => map!(be_u8, Val::U8) |
            b"u" => map!(be_i16, Val::I16) |
            b"U" => map!(be_u16, Val::U16) |
            b"i" => map!(be_i32, Val::I32) |
            b"I" => map!(be_u32, Val::U32) |
            b"l" => map!(be_i64, Val::I64) |
            b"L" => map!(be_u64, Val::U64) |
            b"f" => map!(be_f32, Val::F32) |
            b"d" => map!(be_f64, Val::F64) |
            b"D" => map!(tuple!(be_u8, be_u32), |(scale, val): (u8, u32)| Val::Decimal(scale, val)) |
            b"T" => map!(be_u64, Val::Timestamp) |
            b"V" => value!(Val::Void) |
            b"F" => map!(call!(Table::nom_bytes, pool), Val::Table) |
            b"A" => map!(call!(List::nom_bytes, pool), Val::List)
        );
        result
    }
}

type TableEntry<'a> = (&'a str, Val<'a>);

impl<'a> NomBytes<'a> for Table<'a> {
    fn nom_bytes<'b, P>(input: &'a [u8], pool: &'b mut P) -> IResult<&'a [u8], Self>
        where P: ParserPool
    {
        let (_, bytes) = try_parse!(input, peek!(length_bytes!(be_u32)));

        #[ignore(unused_variables)]
        let (rem, mut entries) = try_parse!(input,
            length_value!(be_u32,
                fold_many0!(
                    tuple!(shortstr, apply!(Val::nom_bytes, pool)),
                    pool.new_table_entries_vec(bytes),
                    |mut entries: Vec<TableEntry<'a>>, entry: TableEntry<'a>| {
                        entries.push(entry);
                        entries
                    }
                )
            )
        );

        let mut table = pool.new_table(entries.len());
        for (k, v) in entries.drain(..) {
            table.insert(k, v);
        }

        // Vec is now empty, discard the lifetimes and return it to the pool
        let entries = unsafe { mem::transmute(entries) };
        pool.return_table_entries_vec(entries);

        IResult::Done(rem, table)
    }
}

impl<'a> NomBytes<'a> for List<'a> {
    /// TODO: Return error if there is junk at the end
    fn nom_bytes<'b, P>(input: &'a [u8], pool: &'b mut P) -> IResult<&'a [u8], Self>
        where P: ParserPool
    {
        let (_, bytes) = try_parse!(input, peek!(length_bytes!(be_u32)));

        #[ignore(unused_variables)]
        let (rem, entries): (&'a [u8], Vec<Val<'a>>) = try_parse!(input,
            length_value!(be_u32,
                fold_many0!(
                    apply!(Val::nom_bytes, pool),
                    pool.new_vals_vec(bytes),
                    |mut entries: Vec<Val<'a>>, entry: Val<'a>| {
                        entries.push(entry);
                        entries
                    }
                )
            )
        );

        IResult::Done(rem, List::from_vec(entries))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use amqp0::nom::NoParserPool;

    #[test]
    fn val_short_string() {
        let bytes = b"s\x0FThis is a test.";
        let (_, val) = Val::nom_bytes(bytes, &mut NoParserPool).unwrap();
        assert_eq!(val, Val::ShortString(Cow::Borrowed("This is a test.")));
    }

    #[test]
    fn val_short_string_invalid() {
        let bytes = b"s\x12Invalid UTF-8: \xe2\x28\xa1";
        assert!(Val::nom_bytes(bytes, &mut NoParserPool).is_err());
    }

    #[test]
    fn val_short_string_incomplete() {
        let bytes = b"s\xffThis is expected to be much longer...";
        assert!(Val::nom_bytes(bytes, &mut NoParserPool).is_incomplete());
    }

    #[test]
    fn val_long_string() {
        let bytes = b"S\x00\x00\x00\x0FThis is a test.";
        let (_, val) = Val::nom_bytes(bytes, &mut NoParserPool).unwrap();
        assert_eq!(val, Val::LongString(Cow::Borrowed(&bytes[5..])));
    }

    #[test]
    fn val_long_string_incomplete() {
        let bytes = b"S\x00\x00\x00\xffThis is expected to be much longer...";
        assert!(Val::nom_bytes(bytes, &mut NoParserPool).is_incomplete());
    }

    macro_rules! parse_val {
        ($bytes:expr) => { Val::nom_bytes(&$bytes[..], &mut NoParserPool) }
    }
    macro_rules! parse_done_val {
        ($bytes:expr) => {{
            let bytes = &$bytes;

            if bytes.len() > 1 {
                assert_incomplete!(bytes[0..bytes.len() - 1]);
            }

            let result = parse_val!(bytes);
            if !result.is_done() {
                panic!("Expected IResult::Done when parsing val, got {:?}", result);
            }

            let result = result.unwrap();
            if result.0.len() > 0 {
                panic!("Expected no remaining bytes, got {:?}", result.0)
            }
            result.1
        }}
    }

    /// Like above, but changes the value of the specified offset and ensure it errors.
    /// Usually used to make one of the lengths invalid.
    macro_rules! parse_done_val_with_length_offset {
        ($bytes:expr, $offset:expr) => {{
            let bytes = &$bytes;
            let offset = $offset;
            if offset > 0 && bytes.len() > offset + 1 {
                let mut bytes = bytes[..].to_vec();
                bytes[$offset] -= 1;
                assert_incomplete!(bytes[..bytes.len() - 1]);

                assert_error!(bytes);
            }
            parse_done_val!(bytes)
        }}
    }
    macro_rules! assert_val_parses {
       ($bytes:expr, $expected:expr) => {
            let _ = parse_done_val!($bytes);
       }
    }

    macro_rules! assert_incomplete {
        ($bytes:expr) => {
            let bytes = &$bytes;
            let result = Val::nom_bytes(&bytes[..], &mut NoParserPool);
            if !result.is_incomplete() {
                panic!("Expected result to be IResult::Incomplete, got: {:?}", result)
            }
        }
    }

    /// Asserts the bytes given do not parse and are not considered "incomplete"
    /// This is more relevant for dynamically-sized types where there are two sizes that
    /// may mismatch. Those are considered errors.
    macro_rules! assert_error {
        ($bytes:expr) => {
            let bytes = &$bytes;
            let result = Val::nom_bytes(&bytes[..], &mut NoParserPool);
            if !result.is_err() {
                panic!("Expected result to be IResult::Err, got: {:?}", result)
            }
        }
    }

    #[test]
    fn val_void() {
        assert_val_parses!(b"V", Val::Void);
    }

    #[test]
    fn val_bool() {
        assert_val_parses!(b"t\x00", Val::Bool(false));
        assert_val_parses!(b"t\x01", Val::Bool(true));
        assert_val_parses!(b"t\x11", Val::Bool(true));
        assert_val_parses!(b"t\xff", Val::Bool(true));
    }
    
    #[test]
    fn val_i8() {
        assert_val_parses!(b"b\x01", Val::I8(0x01));
        assert_val_parses!(b"b\x11", Val::I8(0x11));
        assert_val_parses!(b"b\xff", Val::I8(-1));
    }

    #[test]
    fn val_u8() {
        assert_val_parses!(b"B\x00", Val::U8(0x00));
        assert_val_parses!(b"B\x01", Val::U8(0x01));
        assert_val_parses!(b"B\x11", Val::U8(0x11));
        assert_val_parses!(b"B\xff", Val::U8(0xff));
    }

    #[test]
    fn val_i16() {
        assert_val_parses!(b"u\x00\x00", Val::I16(0x0000));
        assert_val_parses!(b"u\x01\x01", Val::I16(0x0101));
        assert_val_parses!(b"u\x11\x11", Val::I16(0x1111));
        assert_val_parses!(b"u\xff\xff", Val::I16(-1));
    }

    #[test]
    fn val_u16() {
        assert_val_parses!(b"U\x00\x00", Val::U16(0x0000));
        assert_val_parses!(b"U\x01\x01", Val::U16(0x0101));
        assert_val_parses!(b"U\x11\x11", Val::U16(0x1111));
        assert_val_parses!(b"U\xff\xff", Val::U16(0xffff));
    }
    #[test]
    fn val_i32() {
        assert_val_parses!(b"i\x00\x00\x00\x00", Val::I32(0x00000000));
        assert_val_parses!(b"i\x01\x01\x01\x01", Val::I32(0x01010101));
        assert_val_parses!(b"i\x11\x11\x11\x11", Val::I32(0x11111111));
        assert_val_parses!(b"i\xff\xff\xff\xff", Val::I32(-1));
    }

    #[test]
    fn val_u32() {
        assert_val_parses!(b"I\x00\x00\x00\x00", Val::U32(0x00000000));
        assert_val_parses!(b"I\x01\x01\x01\x01", Val::U32(0x01010101));
        assert_val_parses!(b"I\x11\x11\x11\x11", Val::U32(0x11111111));
        assert_val_parses!(b"I\xff\xff\xff\xff", Val::U32(0xffffffff));
    }

    #[test]
    fn val_i64() {
        assert_val_parses!(b"l\x00\x00\x00\x00\x00\x00\x00\x00", Val::I64(0x0000000000000000));
        assert_val_parses!(b"l\x01\x01\x01\x01\x01\x01\x01\x01", Val::I64(0x0101010101010101));
        assert_val_parses!(b"l\x11\x11\x11\x11\x11\x11\x11\x11", Val::I64(0x1111111111111111));
        assert_val_parses!(b"l\xff\xff\xff\xff\xff\xff\xff\xff", Val::I64(-1));
    }

    #[test]
    fn val_u64() {
        assert_val_parses!(b"L\x00\x00\x00\x00\x00\x00\x00\x00", Val::U64(0));
        assert_val_parses!(b"L\x01\x01\x01\x01\x01\x01\x01\x01", Val::U64(0x0101010101010101));
        assert_val_parses!(b"L\x11\x11\x11\x11\x11\x11\x11\x11", Val::U64(0x1111111111111111));
        assert_val_parses!(b"L\xff\xff\xff\xff\xff\xff\xff\xff", Val::U64(0xffffffffffffffff));
    }

    #[test]
    fn val_decimal() {
        assert_val_parses!(b"D\x01\x00\x00\x00\x00", Val::Decimal(1, 0));
        assert_val_parses!(b"D\x01\x01\x01\x01\x01", Val::Decimal(1, 0x01010101));
        assert_val_parses!(b"D\x11\x11\x11\x11\x11", Val::Decimal(0x11, 0x11111111));
        assert_val_parses!(b"D\xff\xff\xff\xff\xff", Val::Decimal(0xff, 0xffffffff));
    }

    #[test]
    fn val_timestamp() {
        assert_incomplete!(b"T\x00\x00\x00\x00\x00\x00\x00");
        assert_val_parses!(b"T\x00\x00\x00\x00\x00\x00\x00\x00", Val::Timestamp(0));
        assert_val_parses!(b"T\x01\x01\x01\x01\x01\x01\x01\x01", Val::Timestamp(0x0101010101010101));
        assert_val_parses!(b"T\x11\x11\x11\x11\x11\x11\x11\x11", Val::Timestamp(0x1111111111111111));
        assert_val_parses!(b"T\xff\xff\xff\xff\xff\xff\xff\xff", Val::Timestamp(0xffffffffffffffff));
    }

    #[test]
    fn val_f32() {
        assert_val_parses!(b"f\x00\x00\x00\x00", Val::F32(0.0));
        assert_val_parses!(b"f\xc0\x78\x00\x00", Val::F32(-3.875));
        assert_val_parses!(b"f\x47\xf8\x00\x00", Val::F32(126976.0));

        let bytes = b"f\xff\xff\xff\xff";
        let (_, val) = Val::nom_bytes(bytes, &mut NoParserPool).unwrap();
        match val {
            Val::F32(f) if f.is_nan() => (),
            _ => panic!("Expected f32 to be NaN"),
        }

        let bytes = b"f\x7f\x80\x00\x00";
        let (_, val) = Val::nom_bytes(bytes, &mut NoParserPool).unwrap();
        match val {
            Val::F32(f) if f.is_infinite() => (),
            _ => panic!("Expected f32 to be infinite"),
        }
    }

    #[test]
    fn val_f64() {
        assert_val_parses!(b"d\x00\x00\x00\x00\x00\x00\x00\x00", Val::F64(0.0));
        assert_val_parses!(b"d\xc0\x0f\x00\x00\x00\x00\x00\x00", Val::F64(-3.875));
        assert_val_parses!(b"d\x40\xfe\xf9\xc0\x00\x00\x00\x00", Val::F64(126876.0));

        let bytes = b"d\x7f\xff\xff\xff\xff\xff\xff\xff";
        let (_, val) = Val::nom_bytes(bytes, &mut NoParserPool).unwrap();
        match val {
            Val::F64(f) if f.is_nan() => (),
            _ => panic!("Expected f32 to be NaN"),
        }

        let bytes = b"d\x7f\xf0\x00\x00\x00\x00\x00\x00";
        let (_, val) = Val::nom_bytes(bytes, &mut NoParserPool).unwrap();
        match val {
            Val::F64(f) if f.is_infinite() => (),
            _ => panic!("Expected f32 to be infinite"),
        }
    }
    #[test]
    fn val_list_empty() {
        let result = parse_done_val!(b"A\x00\x00\x00\x00");

        match result {
            Val::List(ref list) if list.len() == 0 => (),
            val => panic!("Expected empty list, got {:?}", val)
        }
    }

    #[test]
    fn val_list_with_entry() {
        let bytes = b"A\x00\x00\x00\x07s\x05value";
        let val = parse_done_val_with_length_offset!(bytes, 6);

        let list = match val {
            Val::List(ref list) if list.len() == 1 => list,
            val => panic!("Expected list with [value], got {:?}", val)
        };

        assert_eq!(list.last(), Some(&Val::ShortString(Cow::Borrowed("value"))));
    }

    #[test]
    fn val_list_with_entries() {
        let bytes = b"A\x00\x00\x00\x12VVVVVs\x0Bhello world";
        assert_error!(b"A\x00\x00\x00\x08VVVVVs\x02a");

        let bytes = b"A\x00\x00\x00\x12VVVVVs\x0Bhello world";
        let (_, val) = Val::nom_bytes(bytes, &mut NoParserPool).unwrap();

        let list = match val {
            Val::List(ref list) if list.len() == 6 => list,
            val => panic!("Expected list with 6 elements, got {:?}", val)
        };

        assert_eq!(list.get(0), Some(&Val::Void));
        assert_eq!(list.get(1), Some(&Val::Void));
        assert_eq!(list.get(2), Some(&Val::Void));
        assert_eq!(list.get(3), Some(&Val::Void));
        assert_eq!(list.get(4), Some(&Val::Void));
        assert_eq!(list.get(5), Some(&Val::ShortString(Cow::Borrowed("hello world"))));
    }

    #[test]
    fn val_table_empty() {
        assert_incomplete!(b"F\x00\x00\x00");

        let bytes = b"F\x00\x00\x00\x00";
        let (_, val) = Val::nom_bytes(bytes, &mut NoParserPool).unwrap();

        match val {
            Val::Table(ref table) if table.len() == 0 => {},
            val => panic!("Expected empty table, got {:?}", val)
        }
    }

    #[test]
    fn val_table_with_entry() {
        assert_incomplete!(b"F\x00\x00\x00\x01");
        assert_error!     (b"F\x00\x00\x00\x01\x01");
        assert_incomplete!(b"F\x00\x00\x00\x02\x01");
        assert_error!     (b"F\x00\x00\x00\x02\x01a");
        assert_incomplete!(b"F\x00\x00\x00\x03\x01b");
        assert_error!     (b"F\x00\x00\x00\x03\x01cs");
        assert_incomplete!(b"F\x00\x00\x00\x04\x01ds");
        assert_error!     (b"F\x00\x00\x00\x04\x01es\x01");
        assert_incomplete!(b"F\x00\x00\x00\x05\x01fs\x01");
        assert_error!     (b"F\x00\x00\x00\x05\x01gs\x02a");
        assert_incomplete!(b"F\x00\x00\x00\x06\x01hs\x02b");

        let bytes = b"F\x00\x00\x00\x0B\
            \x03key\
            s\x05value";
        let (_, val) = Val::nom_bytes(bytes, &mut NoParserPool).unwrap();

        let table = match val {
            Val::Table(ref table) if table.len() == 1 => table,
            val => panic!("Expected table with keys set to value, got {:?}", val)
        };

        assert_eq!(table.get("key"), Some(&Val::ShortString(Cow::Borrowed("value"))));
    }

    #[test]
    fn val_table_with_entries() {

        let bytes = b"A\x00\x00\x00\x12VVVVVs\x0Bhello world";
        let (_, val) = Val::nom_bytes(bytes, &mut NoParserPool).unwrap();

        let list = match val {
            Val::List(ref list) if list.len() == 6 => list,
            val => panic!("Expected list with 6 elements, got {:?}", val)
        };

        assert_eq!(list.get(0), Some(&Val::Void));
        assert_eq!(list.get(1), Some(&Val::Void));
        assert_eq!(list.get(2), Some(&Val::Void));
        assert_eq!(list.get(3), Some(&Val::Void));
        assert_eq!(list.get(4), Some(&Val::Void));
        assert_eq!(list.get(5), Some(&Val::ShortString(Cow::Borrowed("hello world"))));
    }
}
