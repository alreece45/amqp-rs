
use std::borrow::Cow;
use std::mem;
use std::str;

use nom::{ErrorKind, IResult};
use nom::{be_f32, be_f64};
use nom::{be_u8, be_u16, be_u32, be_u64};
use nom::{be_i8, be_i16, be_i32, be_i64};

use amqp0::nom::{shortstr, longstr};
use amqp0::nom::{NomBytes, ParserPool};
use super::{List, Table, Value};

impl<'a> NomBytes<'a> for Value<'a> {

    #[allow(unused_variables)]
    fn nom_bytes<'b, P>(input: &'a [u8], pool: &'b mut P) -> IResult<&'a [u8], Self>
        where P: ParserPool
    {
        let char = try_parse!(input,
            peek!(take_str!(1))
        );
        println!("Value Type: {:?}", char.1);
        println!("Value Bytes: {:?}", char.0);

        let result = switch!(input, take!(1),
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
            b"F" => map!(call!(Table::nom_bytes, pool), Value::Table) |
            b"A" => map!(call!(List::nom_bytes, pool), Value::List)
        );
        println!("Value: {:?}", result);
        result
    }
}

type TableEntry<'a> = (&'a str, Value<'a>);

impl<'a> NomBytes<'a> for List<'a> {
    /// TODO: Return error if there is junk at the end
    fn nom_bytes<'b, P>(input: &'a [u8], pool: &'b mut P) -> IResult<&'a [u8], Self>
        where P: ParserPool
    {
        let (input, len) = try_parse!(input, be_u32);
        if len == 0 {
            return IResult::Done(input, List::from_vec(vec![]));
        }

        println!("Length: {:?}", len);
        println!("Input: {:?}", input);
        let (_, bytes) = try_parse!(input, peek!(length_bytes!(value!(len))));
        println!("Payload: {:?}", bytes);

        #[ignore(unused_variables)]
        let (rem, entries): (&'a [u8], Vec<Value<'a>>) = try_parse!(input,
            length_value!(value!(len),
                terminated!(
                    many0!(apply!(Value::nom_bytes, pool)),
                    eof!()
                )
            )
        );

        IResult::Done(rem, List::from_vec(entries))
    }
}

impl<'a> NomBytes<'a> for Table<'a> {
    fn nom_bytes<'b, P>(input: &'a [u8], pool: &'b mut P) -> IResult<&'a [u8], Self>
        where P: ParserPool
    {
        let (input, num_bytes) = try_parse!(input, be_u32);
        match num_bytes {
            i if i == 0 => return IResult::Done(input, Table::new()),
            i if i < 3  => return IResult::Error(ErrorKind::Custom(1)),
            _ => (),
        }

        println!("Table Expected Length: {:?}", num_bytes);
        println!("Table Input: {:?}", input);
        println!("Table Input Length: {:?}", input.len());
        let (_, bytes) = try_parse!(input, peek!(length_bytes!(value!(num_bytes))));

        #[ignore(unused_variables)]
        let (rem, mut entries) = try_parse!(input,
            length_value!(value!(num_bytes),
                terminated!(
                    fold_many0!(
                        tuple!(shortstr, apply!(Value::nom_bytes, pool)),
                        pool.new_table_entries_vec(bytes),
                        |mut entries: Vec<TableEntry<'a>>, entry: TableEntry<'a>| {
                            entries.push(entry);
                            entries
                        }
                    ),
                    eof!()
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

#[cfg(test)]
mod test {
    use super::*;
    use amqp0::nom::NoParserPool;

    macro_rules! parse_value {
        ($bytes:expr) => { Value::nom_bytes(&$bytes[..], &mut NoParserPool) }
    }
    macro_rules! parse_done_value {
        ($bytes:expr) => {{
            let bytes = &$bytes;

            if bytes.len() > 1 {
                assert_incomplete!(bytes[0..bytes.len() - 1]);
            }

            let result = parse_value!(bytes);
            if !result.is_done() {
                panic!("Expected IResult::Done when parsing value, got {:?}", result);
            }

            match result {
                IResult::Done(bytes, _) if bytes.len() > 0 => panic!("Expected no remaining bytes, got {:?}", bytes),
                IResult::Done(_, value) => value,
                result => panic!("Expected IResult::Done, got {:?}", result),
            }
        }}
    }

    /// Like above, but changes the value of the specified offset and ensure it errors.
    /// Usually used to make one of the lengths invalid.
    macro_rules! parse_done_value_with_length_offset {
        ($bytes:expr, $offset:expr) => {{
            let bytes = &$bytes;
            let offset = $offset;
            if offset > 0 && bytes.len() > offset + 1 {
                let mut bytes = bytes[..].to_vec();
                bytes[$offset] -= 1;
                {
                    assert_incomplete!(bytes[..bytes.len() - 1]);
                    assert_error!(bytes);
                }
                bytes[$offset] += 1;
            }
            parse_done_value!(bytes)
        }}
    }
    macro_rules! assert_value_parses {
       ($bytes:expr, $expected:expr) => {
            let _ = parse_done_value!($bytes);
       }
    }

    macro_rules! assert_incomplete {
        ($bytes:expr) => {
            let bytes = &$bytes;
            let result = Value::nom_bytes(&bytes[..], &mut NoParserPool);
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
            let result = Value::nom_bytes(&bytes[..], &mut NoParserPool);
            if !result.is_err() {
                panic!("Expected result to be IResult::Err, got: {:?}", result)
            }
        }
    }

    #[test]
    fn value_short_string() {
        let bytes = b"s\x0FThis is a test.";
        let value = parse_done_value!(bytes);
        assert_eq!(value, Value::ShortString(Cow::Borrowed("This is a test.")));
    }

    #[test]
    fn value_short_string_invalueid() {
        let bytes = b"s\x12Invalueid UTF-8: \xe2\x28\xa1";
        assert_error!(bytes);
    }

    #[test]
    fn value_short_string_incomplete() {
        let bytes = b"s\xffThis is expected to be much longer...";
        assert!(Value::nom_bytes(bytes, &mut NoParserPool).is_incomplete());
    }

    #[test]
    fn value_long_string() {
        let bytes = b"S\x00\x00\x00\x0FThis is a test.";
        let (_, value) = Value::nom_bytes(bytes, &mut NoParserPool).unwrap();
        assert_eq!(value, Value::LongString(Cow::Borrowed(&bytes[5..])));
    }

    #[test]
    fn value_long_string_incomplete() {
        let bytes = b"S\x00\x00\x00\xffThis is expected to be much longer...";
        assert!(Value::nom_bytes(bytes, &mut NoParserPool).is_incomplete());
    }

    #[test]
    fn value_void() {
        assert_value_parses!(b"V", Value::Void);
    }

    #[test]
    fn value_bool() {
        assert_value_parses!(b"t\x00", Value::Bool(false));
        assert_value_parses!(b"t\x01", Value::Bool(true));
        assert_value_parses!(b"t\x11", Value::Bool(true));
        assert_value_parses!(b"t\xff", Value::Bool(true));
    }
    
    #[test]
    fn value_i8() {
        assert_value_parses!(b"b\x01", Value::I8(0x01));
        assert_value_parses!(b"b\x11", Value::I8(0x11));
        assert_value_parses!(b"b\xff", Value::I8(-1));
    }

    #[test]
    fn value_u8() {
        assert_value_parses!(b"B\x00", Value::U8(0x00));
        assert_value_parses!(b"B\x01", Value::U8(0x01));
        assert_value_parses!(b"B\x11", Value::U8(0x11));
        assert_value_parses!(b"B\xff", Value::U8(0xff));
    }

    #[test]
    fn value_i16() {
        assert_value_parses!(b"u\x00\x00", Value::I16(0x0000));
        assert_value_parses!(b"u\x01\x01", Value::I16(0x0101));
        assert_value_parses!(b"u\x11\x11", Value::I16(0x1111));
        assert_value_parses!(b"u\xff\xff", Value::I16(-1));
    }

    #[test]
    fn value_u16() {
        assert_value_parses!(b"U\x00\x00", Value::U16(0x0000));
        assert_value_parses!(b"U\x01\x01", Value::U16(0x0101));
        assert_value_parses!(b"U\x11\x11", Value::U16(0x1111));
        assert_value_parses!(b"U\xff\xff", Value::U16(0xffff));
    }
    #[test]
    fn value_i32() {
        assert_value_parses!(b"i\x00\x00\x00\x00", Value::I32(0x00000000));
        assert_value_parses!(b"i\x01\x01\x01\x01", Value::I32(0x01010101));
        assert_value_parses!(b"i\x11\x11\x11\x11", Value::I32(0x11111111));
        assert_value_parses!(b"i\xff\xff\xff\xff", Value::I32(-1));
    }

    #[test]
    fn value_u32() {
        assert_value_parses!(b"I\x00\x00\x00\x00", Value::U32(0x00000000));
        assert_value_parses!(b"I\x01\x01\x01\x01", Value::U32(0x01010101));
        assert_value_parses!(b"I\x11\x11\x11\x11", Value::U32(0x11111111));
        assert_value_parses!(b"I\xff\xff\xff\xff", Value::U32(0xffffffff));
    }

    #[test]
    fn value_i64() {
        assert_value_parses!(b"l\x00\x00\x00\x00\x00\x00\x00\x00", Value::I64(0x0000000000000000));
        assert_value_parses!(b"l\x01\x01\x01\x01\x01\x01\x01\x01", Value::I64(0x0101010101010101));
        assert_value_parses!(b"l\x11\x11\x11\x11\x11\x11\x11\x11", Value::I64(0x1111111111111111));
        assert_value_parses!(b"l\xff\xff\xff\xff\xff\xff\xff\xff", Value::I64(-1));
    }

    #[test]
    fn value_u64() {
        assert_value_parses!(b"L\x00\x00\x00\x00\x00\x00\x00\x00", Value::U64(0));
        assert_value_parses!(b"L\x01\x01\x01\x01\x01\x01\x01\x01", Value::U64(0x0101010101010101));
        assert_value_parses!(b"L\x11\x11\x11\x11\x11\x11\x11\x11", Value::U64(0x1111111111111111));
        assert_value_parses!(b"L\xff\xff\xff\xff\xff\xff\xff\xff", Value::U64(0xffffffffffffffff));
    }

    #[test]
    fn value_decimal() {
        assert_value_parses!(b"D\x01\x00\x00\x00\x00", Value::Decimal(1, 0));
        assert_value_parses!(b"D\x01\x01\x01\x01\x01", Value::Decimal(1, 0x01010101));
        assert_value_parses!(b"D\x11\x11\x11\x11\x11", Value::Decimal(0x11, 0x11111111));
        assert_value_parses!(b"D\xff\xff\xff\xff\xff", Value::Decimal(0xff, 0xffffffff));
    }

    #[test]
    fn value_timestamp() {
        assert_incomplete!(b"T\x00\x00\x00\x00\x00\x00\x00");
        assert_value_parses!(b"T\x00\x00\x00\x00\x00\x00\x00\x00", Value::Timestamp(0));
        assert_value_parses!(b"T\x01\x01\x01\x01\x01\x01\x01\x01", Value::Timestamp(0x0101010101010101));
        assert_value_parses!(b"T\x11\x11\x11\x11\x11\x11\x11\x11", Value::Timestamp(0x1111111111111111));
        assert_value_parses!(b"T\xff\xff\xff\xff\xff\xff\xff\xff", Value::Timestamp(0xffffffffffffffff));
    }

    #[test]
    fn value_f32() {
        assert_value_parses!(b"f\x00\x00\x00\x00", Value::F32(0.0));
        assert_value_parses!(b"f\xc0\x78\x00\x00", Value::F32(-3.875));
        assert_value_parses!(b"f\x47\xf8\x00\x00", Value::F32(126976.0));

        let bytes = b"f\xff\xff\xff\xff";
        let (_, value) = Value::nom_bytes(bytes, &mut NoParserPool).unwrap();
        match value {
            Value::F32(f) if f.is_nan() => (),
            _ => panic!("Expected f32 to be NaN"),
        }

        let bytes = b"f\x7f\x80\x00\x00";
        let (_, value) = Value::nom_bytes(bytes, &mut NoParserPool).unwrap();
        match value {
            Value::F32(f) if f.is_infinite() => (),
            _ => panic!("Expected f32 to be infinite"),
        }
    }

    #[test]
    fn value_f64() {
        assert_value_parses!(b"d\x00\x00\x00\x00\x00\x00\x00\x00", Value::F64(0.0));
        assert_value_parses!(b"d\xc0\x0f\x00\x00\x00\x00\x00\x00", Value::F64(-3.875));
        assert_value_parses!(b"d\x40\xfe\xf9\xc0\x00\x00\x00\x00", Value::F64(126876.0));

        let bytes = b"d\x7f\xff\xff\xff\xff\xff\xff\xff";
        let (_, value) = Value::nom_bytes(bytes, &mut NoParserPool).unwrap();
        match value {
            Value::F64(f) if f.is_nan() => (),
            _ => panic!("Expected f32 to be NaN"),
        }

        let bytes = b"d\x7f\xf0\x00\x00\x00\x00\x00\x00";
        let (_, value) = Value::nom_bytes(bytes, &mut NoParserPool).unwrap();
        match value {
            Value::F64(f) if f.is_infinite() => (),
            _ => panic!("Expected f32 to be infinite"),
        }
    }
    #[test]
    fn value_list_empty() {
        let result = parse_done_value!(b"A\x00\x00\x00\x00");

        match result {
            Value::List(ref list) if list.len() == 0 => (),
            value => panic!("Expected empty list, got {:?}", value)
        }
    }

    #[test]
    fn value_list_with_entry() {
        let bytes = b"A\x00\x00\x00\x07s\x05value";
        let value = parse_done_value_with_length_offset!(bytes, 6);

        let list = match value {
            Value::List(ref list) if list.len() == 1 => list,
            value => panic!("Expected list with [value], got {:?}", value)
        };

        assert_eq!(list.last(), Some(&Value::ShortString(Cow::Borrowed("value"))));
    }

    #[test]
    fn value_list_with_entries() {
        let bytes = b"A\x00\x00\x00\x12VVVVVs\x0Bhello world";
        assert_error!(b"A\x00\x00\x00\x08VVVVVs\x02a");

        let bytes = b"A\x00\x00\x00\x12VVVVVs\x0Bhello world";
        let value = parse_done_value_with_length_offset!(bytes, 11);

        let list = match value {
            Value::List(ref list) if list.len() == 6 => list,
            value => panic!("Expected list with 6 elements, got {:?}", value)
        };

        assert_eq!(list.get(0), Some(&Value::Void));
        assert_eq!(list.get(1), Some(&Value::Void));
        assert_eq!(list.get(2), Some(&Value::Void));
        assert_eq!(list.get(3), Some(&Value::Void));
        assert_eq!(list.get(4), Some(&Value::Void));
        assert_eq!(list.get(5), Some(&Value::ShortString(Cow::Borrowed("hello world"))));
    }

    #[test]
    fn value_table_empty() {
        let bytes = b"F\x00\x00\x00\x00";
        let value = parse_done_value!(bytes);

        match value {
            Value::Table(ref table) if table.len() == 0 => {},
            value => panic!("Expected empty table, got {:?}", value)
        }
    }

    #[test]
    fn value_table_with_entry() {
        let bytes = b"F\x00\x00\x00\x0B\x03keys\x05value";
        let value = parse_done_value_with_length_offset!(bytes, 9);

        let table = match value {
            Value::Table(ref table) if table.len() == 1 => table,
            value => panic!("Expected table with keys set to value, got {:?}", value)
        };

        assert_eq!(table.get("key"), Some(&Value::ShortString(Cow::Borrowed("value"))));
    }

    #[test]
    fn value_table_with_entries() {

        let bytes = b"F\x00\x00\x00\x1E\x01aV\x01bV\x01cV\x01dV\x01eV\x01fs\x0Bhello world";
        let value = parse_done_value_with_length_offset!(bytes, 22);

        let list = match value {
            Value::Table(ref table) if table.len() == 6 => table,
            value => panic!("Expected Table with 6 elements, got {:?}", value)
        };

        assert_eq!(list.get("a"), Some(&Value::Void));
        assert_eq!(list.get("b"), Some(&Value::Void));
        assert_eq!(list.get("b"), Some(&Value::Void));
        assert_eq!(list.get("d"), Some(&Value::Void));
        assert_eq!(list.get("e"), Some(&Value::Void));
        assert_eq!(list.get("f"), Some(&Value::ShortString(Cow::Borrowed("hello world"))));
    }

    #[test]
    fn test_table_has_no_room_for_value() {
        // the table is a bit more complicated than the list
        // we have the table size, size of each key, and size of the value
        // it is an error if any one of them exceeds what we expect for the table size

        // the table format requires at least a u8, u8+, u8* (3 bytes)
        // the size of 2 is simply too small to be valid, even if we got more bytes
        let bytes = b"F\x00\x00\x00\x02";
        assert_error!(bytes);

        // the expected table length is 3 octets.
        // the key takes up 3 octets (one for size, 2 for value)
        // there are no bytes remaining (just the key reaches the expected table length)
        //
        // ideally, this should fail even without the "aa" at the end
        // but we don't parse the table at all until we have the entire thing
        let bytes = b"F\x00\x00\x00\x03\x02aa";
        assert_error!(bytes);

        let bytes = b"F\x00\x00\x00\x04\x01as\x01";
        assert_error!(bytes);
    }
}
