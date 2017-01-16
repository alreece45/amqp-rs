// Generated by build.rs script in amqp0-primitives
// Pre-generated files are used by default. Generation is done with the amqp0-codegen crate
//
// To regenerate, and not use pre-generated files, use: cargo --features="amqp0-build-primitives"
// To format and replace the pre-generated files, use: cargo --features="amqp0-pregen-primitives"
//
// EDITORS BEWARE: Your modifications may be overridden

// generated by primalgen::codegen::spec_module::class_mod::ClassModuleWriter
#![allow(too_many_arguments)]

impl ::method::test::ContentMethod for ::Qpid8_0 {
    type Payload = Content;
} // impl ::method::test::ContentMethod for ::Qpid8_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Content;

impl Content {
    pub fn new() -> Self {
        Content
    } // fn new()
} // impl Content
impl Default for Content {
    fn default() -> Self {
        Content::new()
    } // fn default()
} // impl Default for Content

impl ::Encodable for Content {
    fn encoded_size(&self) -> usize {
        0
    } // encoded_size
    fn write_encoded_to<W>(&self, _: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        ::std::result::Result::Ok(())
    }
} // impl Encodable

#[test]
fn test_content_encodable_bytes_written_matches_len() {
    let payload: Content = Default::default();
    let expected_len = ::Encodable::encoded_size(&payload);
    let mut writer = ::std::io::Cursor::new(Vec::with_capacity(expected_len));
    ::Encodable::write_encoded_to(&payload, &mut writer).unwrap();
    let payload = writer.into_inner();

    if payload.len() != expected_len {
        panic!("Expected payload len {}, got {}, {:?}",
               expected_len,
               payload.len(),
               &payload[..]);
    }
}



impl ::ProtocolMethodPayload for Content {
    fn class(&self) -> ::Class {
        ::Class::Test
    }
    fn class_id(&self) -> u16 {
        120
    }
    fn class_name(&self) -> &'static str {
        "test"
    }
    fn method_id(&self) -> u16 {
        40
    }
    fn method_name(&self) -> &'static str {
        "content"
    }
} // impl ::ProtocolMethodPayload for Content
impl<'a> From<Content> for ClassMethod<'a> {
    fn from(from: Content) -> Self {
        ClassMethod::Content(from)
    } // fn from()
} // impl From<Content> for ClassMethod

impl From<Content> for super::SpecMethod<'static> {
    fn from(from: Content) -> Self {
        super::SpecMethod::Test(from.into())
    } // fn default()
} // impl From<Content> for ::super::SpecMethod
impl ::method::test::ContentOkMethod for ::Qpid8_0 {
    type Payload = ContentOk;
} // impl ::method::test::ContentOkMethod for ::Qpid8_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct ContentOk {
    content_checksum: u32,
} // struct ContentOk

impl ContentOk {
    pub fn new(content_checksum: u32) -> Self {
        ContentOk { content_checksum: content_checksum } // ContentOk
    } // fn new()
    impl_properties! {
(content_checksum, set_content_checksum) -> u32,
} // impl_properties
} // impl ContentOk
impl Default for ContentOk {
    fn default() -> Self {
        ContentOk::new(0)
    } // fn default()
} // impl Default for ContentOk

impl ::Encodable for ContentOk {
    fn encoded_size(&self) -> usize {
        4
    } // encoded_size
    fn write_encoded_to<W>(&self, writer: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        try!(::Encodable::write_encoded_to(&self.content_checksum, writer)); // content_checksum

        ::std::result::Result::Ok(())
    } // fn write_encoded_to()
} // impl Encodable

#[test]
fn test_content_ok_encodable_bytes_written_matches_len() {
    let payload: ContentOk = Default::default();
    let expected_len = ::Encodable::encoded_size(&payload);
    let mut writer = ::std::io::Cursor::new(Vec::with_capacity(expected_len));
    ::Encodable::write_encoded_to(&payload, &mut writer).unwrap();
    let payload = writer.into_inner();

    if payload.len() != expected_len {
        panic!("Expected payload len {}, got {}, {:?}",
               expected_len,
               payload.len(),
               &payload[..]);
    }
}



impl ::ProtocolMethodPayload for ContentOk {
    fn class(&self) -> ::Class {
        ::Class::Test
    }
    fn class_id(&self) -> u16 {
        120
    }
    fn class_name(&self) -> &'static str {
        "test"
    }
    fn method_id(&self) -> u16 {
        41
    }
    fn method_name(&self) -> &'static str {
        "content-ok"
    }
} // impl ::ProtocolMethodPayload for ContentOk
impl ::method::test::SetContentOkMethodFields for ContentOk {
    fn set_content_checksum(&mut self, content_checksum: u32) {
        self.set_content_checksum(content_checksum)
    } // set_content_checksum()
} // impl ::method::test::SetContentOkMethodFields for ContentOk
impl<'a> From<ContentOk> for ClassMethod<'a> {
    fn from(from: ContentOk) -> Self {
        ClassMethod::ContentOk(from)
    } // fn from()
} // impl From<ContentOk> for ClassMethod

impl From<ContentOk> for super::SpecMethod<'static> {
    fn from(from: ContentOk) -> Self {
        super::SpecMethod::Test(from.into())
    } // fn default()
} // impl From<ContentOk> for ::super::SpecMethod
impl ::method::test::IntegerMethod for ::Qpid8_0 {
    type Payload = Integer;
} // impl ::method::test::IntegerMethod for ::Qpid8_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Integer {
    integer_1: u8,
    integer_2: u16,
    integer_3: u32,
    integer_4: u64,
    operation: u8,
} // struct Integer

impl Integer {
    pub fn new(integer_1: u8,
               integer_2: u16,
               integer_3: u32,
               integer_4: u64,
               operation: u8)
               -> Self {
        Integer {
            integer_1: integer_1,
            integer_2: integer_2,
            integer_3: integer_3,
            integer_4: integer_4,
            operation: operation,
        } // Integer
    } // fn new()
    impl_properties! {
(integer_1, set_integer_1) -> u8,
(integer_2, set_integer_2) -> u16,
(integer_3, set_integer_3) -> u32,
(integer_4, set_integer_4) -> u64,
(operation, set_operation) -> u8,
} // impl_properties
} // impl Integer
impl Default for Integer {
    fn default() -> Self {
        Integer::new(0, 0, 0, 0, 0)
    } // fn default()
} // impl Default for Integer

impl ::Encodable for Integer {
    fn encoded_size(&self) -> usize {
        16
    } // encoded_size
    fn write_encoded_to<W>(&self, writer: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        try!(::Encodable::write_encoded_to(&self.integer_1, writer)); // integer_1
        try!(::Encodable::write_encoded_to(&self.integer_2, writer)); // integer_2
        try!(::Encodable::write_encoded_to(&self.integer_3, writer)); // integer_3
        try!(::Encodable::write_encoded_to(&self.integer_4, writer)); // integer_4
        try!(::Encodable::write_encoded_to(&self.operation, writer)); // operation

        ::std::result::Result::Ok(())
    } // fn write_encoded_to()
} // impl Encodable

#[test]
fn test_integer_encodable_bytes_written_matches_len() {
    let payload: Integer = Default::default();
    let expected_len = ::Encodable::encoded_size(&payload);
    let mut writer = ::std::io::Cursor::new(Vec::with_capacity(expected_len));
    ::Encodable::write_encoded_to(&payload, &mut writer).unwrap();
    let payload = writer.into_inner();

    if payload.len() != expected_len {
        panic!("Expected payload len {}, got {}, {:?}",
               expected_len,
               payload.len(),
               &payload[..]);
    }
}



impl ::ProtocolMethodPayload for Integer {
    fn class(&self) -> ::Class {
        ::Class::Test
    }
    fn class_id(&self) -> u16 {
        120
    }
    fn class_name(&self) -> &'static str {
        "test"
    }
    fn method_id(&self) -> u16 {
        10
    }
    fn method_name(&self) -> &'static str {
        "integer"
    }
} // impl ::ProtocolMethodPayload for Integer
impl ::method::test::SetIntegerMethodFields for Integer {
    fn set_integer_1(&mut self, integer_1: u8) {
        self.set_integer_1(integer_1)
    } // set_integer_1()
    fn set_integer_2(&mut self, integer_2: u16) {
        self.set_integer_2(integer_2)
    } // set_integer_2()
    fn set_integer_3(&mut self, integer_3: u32) {
        self.set_integer_3(integer_3)
    } // set_integer_3()
    fn set_integer_4(&mut self, integer_4: u64) {
        self.set_integer_4(integer_4)
    } // set_integer_4()
    fn set_operation(&mut self, operation: u8) {
        self.set_operation(operation)
    } // set_operation()
} // impl ::method::test::SetIntegerMethodFields for Integer
impl<'a> From<Integer> for ClassMethod<'a> {
    fn from(from: Integer) -> Self {
        ClassMethod::Integer(from)
    } // fn from()
} // impl From<Integer> for ClassMethod

impl From<Integer> for super::SpecMethod<'static> {
    fn from(from: Integer) -> Self {
        super::SpecMethod::Test(from.into())
    } // fn default()
} // impl From<Integer> for ::super::SpecMethod
impl ::method::test::IntegerOkMethod for ::Qpid8_0 {
    type Payload = IntegerOk;
} // impl ::method::test::IntegerOkMethod for ::Qpid8_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct IntegerOk {
    result: u64,
} // struct IntegerOk

impl IntegerOk {
    pub fn new(result: u64) -> Self {
        IntegerOk { result: result } // IntegerOk
    } // fn new()
    impl_properties! {
(result, set_result) -> u64,
} // impl_properties
} // impl IntegerOk
impl Default for IntegerOk {
    fn default() -> Self {
        IntegerOk::new(0)
    } // fn default()
} // impl Default for IntegerOk

impl ::Encodable for IntegerOk {
    fn encoded_size(&self) -> usize {
        8
    } // encoded_size
    fn write_encoded_to<W>(&self, writer: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        try!(::Encodable::write_encoded_to(&self.result, writer)); // result

        ::std::result::Result::Ok(())
    } // fn write_encoded_to()
} // impl Encodable

#[test]
fn test_integer_ok_encodable_bytes_written_matches_len() {
    let payload: IntegerOk = Default::default();
    let expected_len = ::Encodable::encoded_size(&payload);
    let mut writer = ::std::io::Cursor::new(Vec::with_capacity(expected_len));
    ::Encodable::write_encoded_to(&payload, &mut writer).unwrap();
    let payload = writer.into_inner();

    if payload.len() != expected_len {
        panic!("Expected payload len {}, got {}, {:?}",
               expected_len,
               payload.len(),
               &payload[..]);
    }
}



impl ::ProtocolMethodPayload for IntegerOk {
    fn class(&self) -> ::Class {
        ::Class::Test
    }
    fn class_id(&self) -> u16 {
        120
    }
    fn class_name(&self) -> &'static str {
        "test"
    }
    fn method_id(&self) -> u16 {
        11
    }
    fn method_name(&self) -> &'static str {
        "integer-ok"
    }
} // impl ::ProtocolMethodPayload for IntegerOk
impl ::method::test::SetIntegerOkMethodFields for IntegerOk {
    fn set_result(&mut self, result: u64) {
        self.set_result(result)
    } // set_result()
} // impl ::method::test::SetIntegerOkMethodFields for IntegerOk
impl<'a> From<IntegerOk> for ClassMethod<'a> {
    fn from(from: IntegerOk) -> Self {
        ClassMethod::IntegerOk(from)
    } // fn from()
} // impl From<IntegerOk> for ClassMethod

impl From<IntegerOk> for super::SpecMethod<'static> {
    fn from(from: IntegerOk) -> Self {
        super::SpecMethod::Test(from.into())
    } // fn default()
} // impl From<IntegerOk> for ::super::SpecMethod
impl<'a> ::method::test::StringMethod<'a> for ::Qpid8_0 {
    type Payload = String<'a>;
} // impl<'a> ::method::test::StringMethod<'a> for ::Qpid8_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct String<'a> {
    string_1: ::std::borrow::Cow<'a, str>,
    string_2: ::std::borrow::Cow<'a, [u8]>,
    operation: u8,
} // struct String<'a>

impl<'a> String<'a> {
    pub fn new<S, S0>(string_1: S, string_2: S0, operation: u8) -> Self
        where S: Into<::std::borrow::Cow<'a, str>>,
              S0: Into<::std::borrow::Cow<'a, [u8]>>
    {
        String {
            string_1: string_1.into(),
            string_2: string_2.into(),
            operation: operation,
        } // String
    } // fn new()
    impl_properties! {
(string_1, string_1_mut, set_string_1) -> Cow<str>,
(string_2, string_2_mut, set_string_2) -> Cow<[u8]>,
(operation, set_operation) -> u8,
} // impl_properties
} // impl<'a> String<'a>
impl<'a> Default for String<'a> {
    fn default() -> Self {
        String::new("", &[][..], 0)
    } // fn default()
} // impl Default for String

impl<'a> ::Encodable for String<'a> {
    fn encoded_size(&self) -> usize {
        1 + ::Encodable::encoded_size(&self.string_1) + ::Encodable::encoded_size(&self.string_2)
    } // encoded_size
    fn write_encoded_to<W>(&self, writer: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        try!(::Encodable::write_encoded_to(&self.string_1, writer)); // string_1
        try!(::Encodable::write_encoded_to(&self.string_2, writer)); // string_2
        try!(::Encodable::write_encoded_to(&self.operation, writer)); // operation

        ::std::result::Result::Ok(())
    } // fn write_encoded_to()
} // impl Encodable

#[test]
fn test_string_encodable_bytes_written_matches_len() {
    let payload: String = Default::default();
    let expected_len = ::Encodable::encoded_size(&payload);
    let mut writer = ::std::io::Cursor::new(Vec::with_capacity(expected_len));
    ::Encodable::write_encoded_to(&payload, &mut writer).unwrap();
    let payload = writer.into_inner();

    if payload.len() != expected_len {
        panic!("Expected payload len {}, got {}, {:?}",
               expected_len,
               payload.len(),
               &payload[..]);
    }
}



impl<'a> ::ProtocolMethodPayload for String<'a> {
    fn class(&self) -> ::Class {
        ::Class::Test
    }
    fn class_id(&self) -> u16 {
        120
    }
    fn class_name(&self) -> &'static str {
        "test"
    }
    fn method_id(&self) -> u16 {
        20
    }
    fn method_name(&self) -> &'static str {
        "string"
    }
} // impl ::ProtocolMethodPayload for String<'a>
impl<'a> ::method::test::SetStringMethodFields<'a> for String<'a> {
    fn set_string_1<V>(&mut self, string_1: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_string_1(string_1.into())
    } // set_string_1()
    fn set_string_2<V>(&mut self, string_2: V)
        where V: Into<::std::borrow::Cow<'a, [u8]>>
    {
        self.set_string_2(string_2.into())
    } // set_string_2()
    fn set_operation(&mut self, operation: u8) {
        self.set_operation(operation)
    } // set_operation()
} // impl<'a> ::method::test::SetStringMethodFields<'a> for String<'a>
impl<'a> From<String<'a>> for ClassMethod<'a> {
    fn from(from: String<'a>) -> Self {
        ClassMethod::String(from)
    } // fn from()
} // impl From<String<'a>> for ClassMethod

impl<'a> From<String<'a>> for super::SpecMethod<'a> {
    fn from(from: String<'a>) -> Self {
        super::SpecMethod::Test(from.into())
    } // fn default()
} // impl From<String<'a>> for ::super::SpecMethod
impl<'a> ::method::test::StringOkMethod<'a> for ::Qpid8_0 {
    type Payload = StringOk<'a>;
} // impl<'a> ::method::test::StringOkMethod<'a> for ::Qpid8_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct StringOk<'a> {
    result: ::std::borrow::Cow<'a, [u8]>,
} // struct StringOk<'a>

impl<'a> StringOk<'a> {
    pub fn new<R>(result: R) -> Self
        where R: Into<::std::borrow::Cow<'a, [u8]>>
    {
        StringOk { result: result.into() } // StringOk
    } // fn new()
    impl_properties! {
(result, result_mut, set_result) -> Cow<[u8]>,
} // impl_properties
} // impl<'a> StringOk<'a>
impl<'a> Default for StringOk<'a> {
    fn default() -> Self {
        StringOk::new(&[][..])
    } // fn default()
} // impl Default for StringOk

impl<'a> ::Encodable for StringOk<'a> {
    fn encoded_size(&self) -> usize {
        0 + ::Encodable::encoded_size(&self.result)
    } // encoded_size
    fn write_encoded_to<W>(&self, writer: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        try!(::Encodable::write_encoded_to(&self.result, writer)); // result

        ::std::result::Result::Ok(())
    } // fn write_encoded_to()
} // impl Encodable

#[test]
fn test_string_ok_encodable_bytes_written_matches_len() {
    let payload: StringOk = Default::default();
    let expected_len = ::Encodable::encoded_size(&payload);
    let mut writer = ::std::io::Cursor::new(Vec::with_capacity(expected_len));
    ::Encodable::write_encoded_to(&payload, &mut writer).unwrap();
    let payload = writer.into_inner();

    if payload.len() != expected_len {
        panic!("Expected payload len {}, got {}, {:?}",
               expected_len,
               payload.len(),
               &payload[..]);
    }
}



impl<'a> ::ProtocolMethodPayload for StringOk<'a> {
    fn class(&self) -> ::Class {
        ::Class::Test
    }
    fn class_id(&self) -> u16 {
        120
    }
    fn class_name(&self) -> &'static str {
        "test"
    }
    fn method_id(&self) -> u16 {
        21
    }
    fn method_name(&self) -> &'static str {
        "string-ok"
    }
} // impl ::ProtocolMethodPayload for StringOk<'a>
impl<'a> ::method::test::SetStringOkMethodFields<'a> for StringOk<'a> {
    fn set_result<V>(&mut self, result: V)
        where V: Into<::std::borrow::Cow<'a, [u8]>>
    {
        self.set_result(result.into())
    } // set_result()
} // impl<'a> ::method::test::SetStringOkMethodFields<'a> for StringOk<'a>
impl<'a> From<StringOk<'a>> for ClassMethod<'a> {
    fn from(from: StringOk<'a>) -> Self {
        ClassMethod::StringOk(from)
    } // fn from()
} // impl From<StringOk<'a>> for ClassMethod

impl<'a> From<StringOk<'a>> for super::SpecMethod<'a> {
    fn from(from: StringOk<'a>) -> Self {
        super::SpecMethod::Test(from.into())
    } // fn default()
} // impl From<StringOk<'a>> for ::super::SpecMethod
impl<'a> ::method::test::TableMethod<'a> for ::Qpid8_0 {
    type Payload = Table<'a>;
} // impl<'a> ::method::test::TableMethod<'a> for ::Qpid8_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Table<'a> {
    table: ::field::TableEntries<'a>,
    integer_op: u8,
    string_op: u8,
} // struct Table<'a>

impl<'a> Table<'a> {
    pub fn new<T>(table: T, integer_op: u8, string_op: u8) -> Self
        where T: Into<::field::TableEntries<'a>>
    {
        Table {
            table: table.into(),
            integer_op: integer_op,
            string_op: string_op,
        } // Table
    } // fn new()
    impl_properties! {
(table, table_mut, set_table) -> &::field::TableEntries<'a>,
(integer_op, set_integer_op) -> u8,
(string_op, set_string_op) -> u8,
} // impl_properties
} // impl<'a> Table<'a>
impl<'a> Default for Table<'a> {
    fn default() -> Self {
        Table::new(::field::TableEntries::new(), 0, 0)
    } // fn default()
} // impl Default for Table

impl<'a> ::Encodable for Table<'a> {
    fn encoded_size(&self) -> usize {
        2 + ::Encodable::encoded_size(&self.table)
    } // encoded_size
    fn write_encoded_to<W>(&self, writer: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        try!(::Encodable::write_encoded_to(&self.table, writer)); // table
        try!(::Encodable::write_encoded_to(&self.integer_op, writer)); // integer_op
        try!(::Encodable::write_encoded_to(&self.string_op, writer)); // string_op

        ::std::result::Result::Ok(())
    } // fn write_encoded_to()
} // impl Encodable

#[test]
fn test_table_encodable_bytes_written_matches_len() {
    let payload: Table = Default::default();
    let expected_len = ::Encodable::encoded_size(&payload);
    let mut writer = ::std::io::Cursor::new(Vec::with_capacity(expected_len));
    ::Encodable::write_encoded_to(&payload, &mut writer).unwrap();
    let payload = writer.into_inner();

    if payload.len() != expected_len {
        panic!("Expected payload len {}, got {}, {:?}",
               expected_len,
               payload.len(),
               &payload[..]);
    }
}



impl<'a> ::ProtocolMethodPayload for Table<'a> {
    fn class(&self) -> ::Class {
        ::Class::Test
    }
    fn class_id(&self) -> u16 {
        120
    }
    fn class_name(&self) -> &'static str {
        "test"
    }
    fn method_id(&self) -> u16 {
        30
    }
    fn method_name(&self) -> &'static str {
        "table"
    }
} // impl ::ProtocolMethodPayload for Table<'a>
impl<'a> ::method::test::SetTableMethodFields<'a> for Table<'a> {
    fn set_table<V>(&mut self, table: V)
        where V: Into<::field::TableEntries<'a>>
    {
        self.set_table(table.into())
    } // set_table()
    fn set_integer_op(&mut self, integer_op: u8) {
        self.set_integer_op(integer_op)
    } // set_integer_op()
    fn set_string_op(&mut self, string_op: u8) {
        self.set_string_op(string_op)
    } // set_string_op()
} // impl<'a> ::method::test::SetTableMethodFields<'a> for Table<'a>
impl<'a> From<Table<'a>> for ClassMethod<'a> {
    fn from(from: Table<'a>) -> Self {
        ClassMethod::Table(from)
    } // fn from()
} // impl From<Table<'a>> for ClassMethod

impl<'a> From<Table<'a>> for super::SpecMethod<'a> {
    fn from(from: Table<'a>) -> Self {
        super::SpecMethod::Test(from.into())
    } // fn default()
} // impl From<Table<'a>> for ::super::SpecMethod
impl<'a> ::method::test::TableOkMethod<'a> for ::Qpid8_0 {
    type Payload = TableOk<'a>;
} // impl<'a> ::method::test::TableOkMethod<'a> for ::Qpid8_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct TableOk<'a> {
    integer_result: u64,
    string_result: ::std::borrow::Cow<'a, [u8]>,
} // struct TableOk<'a>

impl<'a> TableOk<'a> {
    pub fn new<S>(integer_result: u64, string_result: S) -> Self
        where S: Into<::std::borrow::Cow<'a, [u8]>>
    {
        TableOk {
            integer_result: integer_result,
            string_result: string_result.into(),
        } // TableOk
    } // fn new()
    impl_properties! {
(integer_result, set_integer_result) -> u64,
(string_result, string_result_mut, set_string_result) -> Cow<[u8]>,
} // impl_properties
} // impl<'a> TableOk<'a>
impl<'a> Default for TableOk<'a> {
    fn default() -> Self {
        TableOk::new(0, &[][..])
    } // fn default()
} // impl Default for TableOk

impl<'a> ::Encodable for TableOk<'a> {
    fn encoded_size(&self) -> usize {
        8 + ::Encodable::encoded_size(&self.string_result)
    } // encoded_size
    fn write_encoded_to<W>(&self, writer: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        try!(::Encodable::write_encoded_to(&self.integer_result, writer)); // integer_result
        try!(::Encodable::write_encoded_to(&self.string_result, writer)); // string_result

        ::std::result::Result::Ok(())
    } // fn write_encoded_to()
} // impl Encodable

#[test]
fn test_table_ok_encodable_bytes_written_matches_len() {
    let payload: TableOk = Default::default();
    let expected_len = ::Encodable::encoded_size(&payload);
    let mut writer = ::std::io::Cursor::new(Vec::with_capacity(expected_len));
    ::Encodable::write_encoded_to(&payload, &mut writer).unwrap();
    let payload = writer.into_inner();

    if payload.len() != expected_len {
        panic!("Expected payload len {}, got {}, {:?}",
               expected_len,
               payload.len(),
               &payload[..]);
    }
}



impl<'a> ::ProtocolMethodPayload for TableOk<'a> {
    fn class(&self) -> ::Class {
        ::Class::Test
    }
    fn class_id(&self) -> u16 {
        120
    }
    fn class_name(&self) -> &'static str {
        "test"
    }
    fn method_id(&self) -> u16 {
        31
    }
    fn method_name(&self) -> &'static str {
        "table-ok"
    }
} // impl ::ProtocolMethodPayload for TableOk<'a>
impl<'a> ::method::test::SetTableOkMethodFields<'a> for TableOk<'a> {
    fn set_integer_result(&mut self, integer_result: u64) {
        self.set_integer_result(integer_result)
    } // set_integer_result()
    fn set_string_result<V>(&mut self, string_result: V)
        where V: Into<::std::borrow::Cow<'a, [u8]>>
    {
        self.set_string_result(string_result.into())
    } // set_string_result()
} // impl<'a> ::method::test::SetTableOkMethodFields<'a> for TableOk<'a>
impl<'a> From<TableOk<'a>> for ClassMethod<'a> {
    fn from(from: TableOk<'a>) -> Self {
        ClassMethod::TableOk(from)
    } // fn from()
} // impl From<TableOk<'a>> for ClassMethod

impl<'a> From<TableOk<'a>> for super::SpecMethod<'a> {
    fn from(from: TableOk<'a>) -> Self {
        super::SpecMethod::Test(from.into())
    } // fn default()
} // impl From<TableOk<'a>> for ::super::SpecMethod

#[derive(Debug)]
pub enum ClassMethod<'a> {
    Content(Content),
    ContentOk(ContentOk),
    Integer(Integer),
    IntegerOk(IntegerOk),
    String(String<'a>),
    StringOk(StringOk<'a>),
    Table(Table<'a>),
    TableOk(TableOk<'a>),
} // enum ClassMethod


impl<'a> ::Encodable for ClassMethod<'a> {
    fn encoded_size(&self) -> usize {
        match *self {
            ClassMethod::Content(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::ContentOk(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::Integer(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::IntegerOk(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::String(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::StringOk(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::Table(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::TableOk(ref method) => ::Encodable::encoded_size(method),

        } // match *self

    } // fn encoded_size
    fn write_encoded_to<W>(&self, _: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        unimplemented!()
    } // fn write_encoded_to()
} // impl ::Encodable for ClassMethod<'a>

impl<'a> ::ProtocolMethodPayload for ClassMethod<'a> {
    fn class(&self) -> ::Class {
        match *self {
            ClassMethod::Content(ref method) => ::ProtocolMethodPayload::class(method),
            ClassMethod::ContentOk(ref method) => ::ProtocolMethodPayload::class(method),
            ClassMethod::Integer(ref method) => ::ProtocolMethodPayload::class(method),
            ClassMethod::IntegerOk(ref method) => ::ProtocolMethodPayload::class(method),
            ClassMethod::String(ref method) => ::ProtocolMethodPayload::class(method),
            ClassMethod::StringOk(ref method) => ::ProtocolMethodPayload::class(method),
            ClassMethod::Table(ref method) => ::ProtocolMethodPayload::class(method),
            ClassMethod::TableOk(ref method) => ::ProtocolMethodPayload::class(method),

        } // match *self

    } // fn class

    fn class_id(&self) -> u16 {
        match *self {
            ClassMethod::Content(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::ContentOk(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::Integer(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::IntegerOk(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::String(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::StringOk(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::Table(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::TableOk(ref method) => ::ProtocolMethodPayload::class_id(method),

        } // match *self

    } // fn class_id

    fn class_name(&self) -> &'static str {
        match *self {
            ClassMethod::Content(ref method) => ::ProtocolMethodPayload::class_name(method),
            ClassMethod::ContentOk(ref method) => ::ProtocolMethodPayload::class_name(method),
            ClassMethod::Integer(ref method) => ::ProtocolMethodPayload::class_name(method),
            ClassMethod::IntegerOk(ref method) => ::ProtocolMethodPayload::class_name(method),
            ClassMethod::String(ref method) => ::ProtocolMethodPayload::class_name(method),
            ClassMethod::StringOk(ref method) => ::ProtocolMethodPayload::class_name(method),
            ClassMethod::Table(ref method) => ::ProtocolMethodPayload::class_name(method),
            ClassMethod::TableOk(ref method) => ::ProtocolMethodPayload::class_name(method),

        } // match *self

    } // fn class_name

    fn method_id(&self) -> u16 {
        match *self {
            ClassMethod::Content(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::ContentOk(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::Integer(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::IntegerOk(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::String(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::StringOk(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::Table(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::TableOk(ref method) => ::ProtocolMethodPayload::method_id(method),

        } // match *self

    } // fn method_id

    fn method_name(&self) -> &'static str {
        match *self {
            ClassMethod::Content(ref method) => ::ProtocolMethodPayload::method_name(method),
            ClassMethod::ContentOk(ref method) => ::ProtocolMethodPayload::method_name(method),
            ClassMethod::Integer(ref method) => ::ProtocolMethodPayload::method_name(method),
            ClassMethod::IntegerOk(ref method) => ::ProtocolMethodPayload::method_name(method),
            ClassMethod::String(ref method) => ::ProtocolMethodPayload::method_name(method),
            ClassMethod::StringOk(ref method) => ::ProtocolMethodPayload::method_name(method),
            ClassMethod::Table(ref method) => ::ProtocolMethodPayload::method_name(method),
            ClassMethod::TableOk(ref method) => ::ProtocolMethodPayload::method_name(method),

        } // match *self

    } // fn method_name
} // impl ProtocolMethodPayload for ClassMethod
