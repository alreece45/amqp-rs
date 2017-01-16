// Generated by build.rs script in amqp0-primitives
// Pre-generated files are used by default. Generation is done with the amqp0-codegen crate
//
// To regenerate, and not use pre-generated files, use: cargo --features="amqp0-build-primitives"
// To format and replace the pre-generated files, use: cargo --features="amqp0-pregen-primitives"
//
// EDITORS BEWARE: Your modifications may be overridden

// generated by primalgen::codegen::spec_module::class_mod::ClassModuleWriter
#![allow(too_many_arguments)]

impl ::method::dtx::SelectMethod for ::Qpid9_0 {
    type Payload = Select;
} // impl ::method::dtx::SelectMethod for ::Qpid9_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Select;

impl Select {
    pub fn new() -> Self {
        Select
    } // fn new()
} // impl Select
impl Default for Select {
    fn default() -> Self {
        Select::new()
    } // fn default()
} // impl Default for Select

impl ::Encodable for Select {
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
fn test_select_encodable_bytes_written_matches_len() {
    let payload: Select = Default::default();
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



impl ::ProtocolMethodPayload for Select {
    fn class(&self) -> ::Class {
        ::Class::Dtx
    }
    fn class_id(&self) -> u16 {
        100
    }
    fn class_name(&self) -> &'static str {
        "dtx"
    }
    fn method_id(&self) -> u16 {
        10
    }
    fn method_name(&self) -> &'static str {
        "select"
    }
} // impl ::ProtocolMethodPayload for Select
impl<'a> From<Select> for ClassMethod<'a> {
    fn from(from: Select) -> Self {
        ClassMethod::Select(from)
    } // fn from()
} // impl From<Select> for ClassMethod

impl From<Select> for super::SpecMethod<'static> {
    fn from(from: Select) -> Self {
        super::SpecMethod::Dtx(from.into())
    } // fn default()
} // impl From<Select> for ::super::SpecMethod
impl ::method::dtx::SelectOkMethod for ::Qpid9_0 {
    type Payload = SelectOk;
} // impl ::method::dtx::SelectOkMethod for ::Qpid9_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct SelectOk;

impl SelectOk {
    pub fn new() -> Self {
        SelectOk
    } // fn new()
} // impl SelectOk
impl Default for SelectOk {
    fn default() -> Self {
        SelectOk::new()
    } // fn default()
} // impl Default for SelectOk

impl ::Encodable for SelectOk {
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
fn test_select_ok_encodable_bytes_written_matches_len() {
    let payload: SelectOk = Default::default();
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



impl ::ProtocolMethodPayload for SelectOk {
    fn class(&self) -> ::Class {
        ::Class::Dtx
    }
    fn class_id(&self) -> u16 {
        100
    }
    fn class_name(&self) -> &'static str {
        "dtx"
    }
    fn method_id(&self) -> u16 {
        11
    }
    fn method_name(&self) -> &'static str {
        "select-ok"
    }
} // impl ::ProtocolMethodPayload for SelectOk
impl<'a> From<SelectOk> for ClassMethod<'a> {
    fn from(from: SelectOk) -> Self {
        ClassMethod::SelectOk(from)
    } // fn from()
} // impl From<SelectOk> for ClassMethod

impl From<SelectOk> for super::SpecMethod<'static> {
    fn from(from: SelectOk) -> Self {
        super::SpecMethod::Dtx(from.into())
    } // fn default()
} // impl From<SelectOk> for ::super::SpecMethod
impl<'a> ::method::dtx::StartMethod<'a> for ::Qpid9_0 {
    type Payload = Start<'a>;
} // impl<'a> ::method::dtx::StartMethod<'a> for ::Qpid9_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Start<'a> {
    dtx_identifier: ::std::borrow::Cow<'a, str>,
} // struct Start<'a>

impl<'a> Start<'a> {
    pub fn new<D>(dtx_identifier: D) -> Self
        where D: Into<::std::borrow::Cow<'a, str>>
    {
        Start { dtx_identifier: dtx_identifier.into() } // Start
    } // fn new()
    impl_properties! {
(dtx_identifier, dtx_identifier_mut, set_dtx_identifier) -> Cow<str>,
} // impl_properties
} // impl<'a> Start<'a>
impl<'a> Default for Start<'a> {
    fn default() -> Self {
        Start::new("")
    } // fn default()
} // impl Default for Start

impl<'a> ::Encodable for Start<'a> {
    fn encoded_size(&self) -> usize {
        0 + ::Encodable::encoded_size(&self.dtx_identifier)
    } // encoded_size
    fn write_encoded_to<W>(&self, writer: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        try!(::Encodable::write_encoded_to(&self.dtx_identifier, writer)); // dtx_identifier

        ::std::result::Result::Ok(())
    } // fn write_encoded_to()
} // impl Encodable

#[test]
fn test_start_encodable_bytes_written_matches_len() {
    let payload: Start = Default::default();
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



impl<'a> ::ProtocolMethodPayload for Start<'a> {
    fn class(&self) -> ::Class {
        ::Class::Dtx
    }
    fn class_id(&self) -> u16 {
        100
    }
    fn class_name(&self) -> &'static str {
        "dtx"
    }
    fn method_id(&self) -> u16 {
        20
    }
    fn method_name(&self) -> &'static str {
        "start"
    }
} // impl ::ProtocolMethodPayload for Start<'a>
impl<'a> ::method::dtx::SetStartMethodFields<'a> for Start<'a> {
    fn set_dtx_identifier<V>(&mut self, dtx_identifier: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_dtx_identifier(dtx_identifier.into())
    } // set_dtx_identifier()
} // impl<'a> ::method::dtx::SetStartMethodFields<'a> for Start<'a>
impl<'a> From<Start<'a>> for ClassMethod<'a> {
    fn from(from: Start<'a>) -> Self {
        ClassMethod::Start(from)
    } // fn from()
} // impl From<Start<'a>> for ClassMethod

impl<'a> From<Start<'a>> for super::SpecMethod<'a> {
    fn from(from: Start<'a>) -> Self {
        super::SpecMethod::Dtx(from.into())
    } // fn default()
} // impl From<Start<'a>> for ::super::SpecMethod
impl ::method::dtx::StartOkMethod for ::Qpid9_0 {
    type Payload = StartOk;
} // impl ::method::dtx::StartOkMethod for ::Qpid9_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct StartOk;

impl StartOk {
    pub fn new() -> Self {
        StartOk
    } // fn new()
} // impl StartOk
impl Default for StartOk {
    fn default() -> Self {
        StartOk::new()
    } // fn default()
} // impl Default for StartOk

impl ::Encodable for StartOk {
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
fn test_start_ok_encodable_bytes_written_matches_len() {
    let payload: StartOk = Default::default();
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



impl ::ProtocolMethodPayload for StartOk {
    fn class(&self) -> ::Class {
        ::Class::Dtx
    }
    fn class_id(&self) -> u16 {
        100
    }
    fn class_name(&self) -> &'static str {
        "dtx"
    }
    fn method_id(&self) -> u16 {
        21
    }
    fn method_name(&self) -> &'static str {
        "start-ok"
    }
} // impl ::ProtocolMethodPayload for StartOk
impl<'a> From<StartOk> for ClassMethod<'a> {
    fn from(from: StartOk) -> Self {
        ClassMethod::StartOk(from)
    } // fn from()
} // impl From<StartOk> for ClassMethod

impl From<StartOk> for super::SpecMethod<'static> {
    fn from(from: StartOk) -> Self {
        super::SpecMethod::Dtx(from.into())
    } // fn default()
} // impl From<StartOk> for ::super::SpecMethod

#[derive(Debug)]
pub enum ClassMethod<'a> {
    Select(Select),
    SelectOk(SelectOk),
    Start(Start<'a>),
    StartOk(StartOk),
} // enum ClassMethod


impl<'a> ::Encodable for ClassMethod<'a> {
    fn encoded_size(&self) -> usize {
        match *self {
            ClassMethod::Select(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::SelectOk(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::Start(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::StartOk(ref method) => ::Encodable::encoded_size(method),

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
            ClassMethod::Select(ref method) => ::ProtocolMethodPayload::class(method),
            ClassMethod::SelectOk(ref method) => ::ProtocolMethodPayload::class(method),
            ClassMethod::Start(ref method) => ::ProtocolMethodPayload::class(method),
            ClassMethod::StartOk(ref method) => ::ProtocolMethodPayload::class(method),

        } // match *self

    } // fn class

    fn class_id(&self) -> u16 {
        match *self {
            ClassMethod::Select(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::SelectOk(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::Start(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::StartOk(ref method) => ::ProtocolMethodPayload::class_id(method),

        } // match *self

    } // fn class_id

    fn class_name(&self) -> &'static str {
        match *self {
            ClassMethod::Select(ref method) => ::ProtocolMethodPayload::class_name(method),
            ClassMethod::SelectOk(ref method) => ::ProtocolMethodPayload::class_name(method),
            ClassMethod::Start(ref method) => ::ProtocolMethodPayload::class_name(method),
            ClassMethod::StartOk(ref method) => ::ProtocolMethodPayload::class_name(method),

        } // match *self

    } // fn class_name

    fn method_id(&self) -> u16 {
        match *self {
            ClassMethod::Select(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::SelectOk(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::Start(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::StartOk(ref method) => ::ProtocolMethodPayload::method_id(method),

        } // match *self

    } // fn method_id

    fn method_name(&self) -> &'static str {
        match *self {
            ClassMethod::Select(ref method) => ::ProtocolMethodPayload::method_name(method),
            ClassMethod::SelectOk(ref method) => ::ProtocolMethodPayload::method_name(method),
            ClassMethod::Start(ref method) => ::ProtocolMethodPayload::method_name(method),
            ClassMethod::StartOk(ref method) => ::ProtocolMethodPayload::method_name(method),

        } // match *self

    } // fn method_name
} // impl ProtocolMethodPayload for ClassMethod
