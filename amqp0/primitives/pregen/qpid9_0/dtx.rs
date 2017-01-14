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

impl ::ProtocolMethodPayload for Select {
    fn class_id(&self) -> u16 {
        100
    } // fn class_id()
    fn method_id(&self) -> u16 {
        10
    } // fn method_id()
} // impl ::Payload for Select
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

impl ::ProtocolMethodPayload for SelectOk {
    fn class_id(&self) -> u16 {
        100
    } // fn class_id()
    fn method_id(&self) -> u16 {
        11
    } // fn method_id()
} // impl ::Payload for SelectOk
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
        try!(::Encodable::write_encoded_to(&self.dtx_identifier, writer));

        ::std::result::Result::Ok(())
    } // fn write_encoded_to()
} // impl Encodable

impl<'a> ::ProtocolMethodPayload for Start<'a> {
    fn class_id(&self) -> u16 {
        100
    } // fn class_id()
    fn method_id(&self) -> u16 {
        20
    } // fn method_id()
} // impl ::Payload for Start
impl<'a> ::method::dtx::SetStartMethodFields<'a> for Start<'a> {
    fn set_dtx_identifier<V>(&mut self, dtx_identifier: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_dtx_identifier(dtx_identifier.into())
    } // set_dtx_identifier()
} // impl<'a> ::method::dtx::SetStartMethodFields<'a> for Start<'a>
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

impl ::ProtocolMethodPayload for StartOk {
    fn class_id(&self) -> u16 {
        100
    } // fn class_id()
    fn method_id(&self) -> u16 {
        21
    } // fn method_id()
} // impl ::Payload for StartOk

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
    fn class_id(&self) -> u16 {
        match *self {
            ClassMethod::Select(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::SelectOk(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::Start(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::StartOk(ref method) => ::ProtocolMethodPayload::class_id(method),

        } // match *self

    } // fn class_id

    fn method_id(&self) -> u16 {
        match *self {
            ClassMethod::Select(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::SelectOk(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::Start(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::StartOk(ref method) => ::ProtocolMethodPayload::method_id(method),

        } // match *self

    } // fn method_id
} // impl ProtocolMethodPayload for ClassMethod
