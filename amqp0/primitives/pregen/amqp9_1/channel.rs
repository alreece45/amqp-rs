// Generated by build.rs script in the amqp0-primitives crate.
// Pre-generated files are used by default. Generation is done with the amqp0-codegen crate.
//
// To regenerate, ignoring the pre-generated files, use: cargo --features="amqp0-build-primitives"
// To format and replace the pre-generated files, use: cargo --features="amqp0-pregen-primitives"
//
// EDITORS BEWARE: Your modifications may be overridden or removed.

// generated by primalgen::codegen::spec_module::class_mod::ClassModuleWriter
#![allow(too_many_arguments)]

impl<'a> ::method::channel::CloseMethod<'a> for ::Amqp9_1 {
    type Payload = Close<'a>;
} // impl<'a> ::method::channel::CloseMethod<'a> for ::Amqp9_1

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Close<'a> {
    reply_code: u16,
    reply_text: ::std::borrow::Cow<'a, str>,
    class_id: u16,
    method_id: u16,
} // struct Close<'a>

impl<'a> Close<'a> {
    pub fn new<R>(reply_code: u16, reply_text: R, class_id: u16, method_id: u16) -> Self
        where R: Into<::std::borrow::Cow<'a, str>>
    {
        Close {
            reply_code: reply_code,
            reply_text: reply_text.into(),
            class_id: class_id,
            method_id: method_id,
        } // Close
    } // fn new()
    impl_properties! {
(reply_code, set_reply_code) -> u16,
(reply_text, reply_text_mut, set_reply_text) -> Cow<str>,
(class_id, set_class_id) -> u16,
(method_id, set_method_id) -> u16,
} // impl_properties
} // impl<'a> Close<'a>
impl<'a> Default for Close<'a> {
    fn default() -> Self {
        Close::new(0, "", 0, 0)
    } // fn default()
} // impl Default for Close

impl<'a> ::Encodable for Close<'a> {
    fn encoded_size(&self) -> usize {
        6 + ::Encodable::encoded_size(&self.reply_text)
    } // encoded_size
    fn write_encoded_to<W>(&self, writer: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        try!(::Encodable::write_encoded_to(&self.reply_code, writer)); // reply_code
        try!(::Encodable::write_encoded_to(&self.reply_text, writer)); // reply_text
        try!(::Encodable::write_encoded_to(&self.class_id, writer)); // class_id
        try!(::Encodable::write_encoded_to(&self.method_id, writer)); // method_id

        ::std::result::Result::Ok(())
    } // fn write_encoded_to()
} // impl Encodable

#[test]
fn test_close_encodable_bytes_written_matches_len() {
    let payload: Close = Default::default();
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



impl<'a> ::ProtocolMethodPayload for Close<'a> {
    fn class(&self) -> ::Class {
        ::Class::Channel
    }
    fn class_id(&self) -> u16 {
        20
    }
    fn class_name(&self) -> &'static str {
        "channel"
    }
    fn method_id(&self) -> u16 {
        40
    }
    fn method_name(&self) -> &'static str {
        "close"
    }
} // impl ::ProtocolMethodPayload for Close<'a>
impl<'a> ::method::channel::SetCloseMethodFields<'a> for Close<'a> {
    fn set_reply_code(&mut self, reply_code: u16) {
        self.set_reply_code(reply_code)
    } // set_reply_code()
    fn set_reply_text<V>(&mut self, reply_text: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_reply_text(reply_text.into())
    } // set_reply_text()
    fn set_class_id(&mut self, class_id: u16) {
        self.set_class_id(class_id)
    } // set_class_id()
    fn set_method_id(&mut self, method_id: u16) {
        self.set_method_id(method_id)
    } // set_method_id()
} // impl<'a> ::method::channel::SetCloseMethodFields<'a> for Close<'a>
impl<'a> From<Close<'a>> for ClassMethod<'a> {
    fn from(from: Close<'a>) -> Self {
        ClassMethod::Close(from)
    } // fn from()
} // impl From<Close<'a>> for ClassMethod

impl<'a> From<Close<'a>> for super::SpecMethod<'a> {
    fn from(from: Close<'a>) -> Self {
        super::SpecMethod::Channel(from.into())
    } // fn default()
} // impl From<Close<'a>> for ::super::SpecMethod
impl ::method::channel::CloseOkMethod for ::Amqp9_1 {
    type Payload = CloseOk;
} // impl ::method::channel::CloseOkMethod for ::Amqp9_1

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct CloseOk;

impl CloseOk {
    pub fn new() -> Self {
        CloseOk
    } // fn new()
} // impl CloseOk
impl Default for CloseOk {
    fn default() -> Self {
        CloseOk::new()
    } // fn default()
} // impl Default for CloseOk

impl ::Encodable for CloseOk {
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
fn test_close_ok_encodable_bytes_written_matches_len() {
    let payload: CloseOk = Default::default();
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



impl ::ProtocolMethodPayload for CloseOk {
    fn class(&self) -> ::Class {
        ::Class::Channel
    }
    fn class_id(&self) -> u16 {
        20
    }
    fn class_name(&self) -> &'static str {
        "channel"
    }
    fn method_id(&self) -> u16 {
        41
    }
    fn method_name(&self) -> &'static str {
        "close-ok"
    }
} // impl ::ProtocolMethodPayload for CloseOk
impl<'a> From<CloseOk> for ClassMethod<'a> {
    fn from(from: CloseOk) -> Self {
        ClassMethod::CloseOk(from)
    } // fn from()
} // impl From<CloseOk> for ClassMethod

impl From<CloseOk> for super::SpecMethod<'static> {
    fn from(from: CloseOk) -> Self {
        super::SpecMethod::Channel(from.into())
    } // fn default()
} // impl From<CloseOk> for ::super::SpecMethod
impl ::method::channel::FlowMethod for ::Amqp9_1 {
    type Payload = Flow;
} // impl ::method::channel::FlowMethod for ::Amqp9_1

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Flow {
    active: bool,
} // struct Flow

impl Flow {
    pub fn new(active: bool) -> Self {
        Flow { active: active } // Flow
    } // fn new()
    impl_properties! {
(active, set_active) -> bool,
} // impl_properties
} // impl Flow
impl Default for Flow {
    fn default() -> Self {
        Flow::new(false)
    } // fn default()
} // impl Default for Flow

impl ::Encodable for Flow {
    fn encoded_size(&self) -> usize {
        1
    } // encoded_size
    fn write_encoded_to<W>(&self, writer: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        try!(::Encodable::write_encoded_to(&{
                                               let mut bits = ::bit_vec::BitVec::from_elem(8,
                                                                                           false);
                                               bits.set(7, self.active);
                                               bits
                                           },
                                           writer));

        ::std::result::Result::Ok(())
    } // fn write_encoded_to()
} // impl Encodable

#[test]
fn test_flow_encodable_bytes_written_matches_len() {
    let payload: Flow = Default::default();
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



impl ::ProtocolMethodPayload for Flow {
    fn class(&self) -> ::Class {
        ::Class::Channel
    }
    fn class_id(&self) -> u16 {
        20
    }
    fn class_name(&self) -> &'static str {
        "channel"
    }
    fn method_id(&self) -> u16 {
        20
    }
    fn method_name(&self) -> &'static str {
        "flow"
    }
} // impl ::ProtocolMethodPayload for Flow
impl ::method::channel::SetFlowMethodFields for Flow {
    fn set_active(&mut self, active: bool) {
        self.set_active(active)
    } // set_active()
} // impl ::method::channel::SetFlowMethodFields for Flow
impl<'a> From<Flow> for ClassMethod<'a> {
    fn from(from: Flow) -> Self {
        ClassMethod::Flow(from)
    } // fn from()
} // impl From<Flow> for ClassMethod

impl From<Flow> for super::SpecMethod<'static> {
    fn from(from: Flow) -> Self {
        super::SpecMethod::Channel(from.into())
    } // fn default()
} // impl From<Flow> for ::super::SpecMethod
impl ::method::channel::FlowOkMethod for ::Amqp9_1 {
    type Payload = FlowOk;
} // impl ::method::channel::FlowOkMethod for ::Amqp9_1

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct FlowOk {
    active: bool,
} // struct FlowOk

impl FlowOk {
    pub fn new(active: bool) -> Self {
        FlowOk { active: active } // FlowOk
    } // fn new()
    impl_properties! {
(active, set_active) -> bool,
} // impl_properties
} // impl FlowOk
impl Default for FlowOk {
    fn default() -> Self {
        FlowOk::new(false)
    } // fn default()
} // impl Default for FlowOk

impl ::Encodable for FlowOk {
    fn encoded_size(&self) -> usize {
        1
    } // encoded_size
    fn write_encoded_to<W>(&self, writer: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        try!(::Encodable::write_encoded_to(&{
                                               let mut bits = ::bit_vec::BitVec::from_elem(8,
                                                                                           false);
                                               bits.set(7, self.active);
                                               bits
                                           },
                                           writer));

        ::std::result::Result::Ok(())
    } // fn write_encoded_to()
} // impl Encodable

#[test]
fn test_flow_ok_encodable_bytes_written_matches_len() {
    let payload: FlowOk = Default::default();
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



impl ::ProtocolMethodPayload for FlowOk {
    fn class(&self) -> ::Class {
        ::Class::Channel
    }
    fn class_id(&self) -> u16 {
        20
    }
    fn class_name(&self) -> &'static str {
        "channel"
    }
    fn method_id(&self) -> u16 {
        21
    }
    fn method_name(&self) -> &'static str {
        "flow-ok"
    }
} // impl ::ProtocolMethodPayload for FlowOk
impl ::method::channel::SetFlowOkMethodFields for FlowOk {
    fn set_active(&mut self, active: bool) {
        self.set_active(active)
    } // set_active()
} // impl ::method::channel::SetFlowOkMethodFields for FlowOk
impl<'a> From<FlowOk> for ClassMethod<'a> {
    fn from(from: FlowOk) -> Self {
        ClassMethod::FlowOk(from)
    } // fn from()
} // impl From<FlowOk> for ClassMethod

impl From<FlowOk> for super::SpecMethod<'static> {
    fn from(from: FlowOk) -> Self {
        super::SpecMethod::Channel(from.into())
    } // fn default()
} // impl From<FlowOk> for ::super::SpecMethod
impl<'a> ::method::channel::OpenMethod<'a> for ::Amqp9_1 {
    type Payload = Open;
} // impl<'a> ::method::channel::OpenMethod<'a> for ::Amqp9_1

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Open;

impl Open {
    pub fn new() -> Self {
        Open
    } // fn new()
} // impl Open
impl Default for Open {
    fn default() -> Self {
        Open::new()
    } // fn default()
} // impl Default for Open

impl ::Encodable for Open {
    fn encoded_size(&self) -> usize {
        1
    } // encoded_size
    fn write_encoded_to<W>(&self, writer: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        try!(::Encodable::write_encoded_to(&0u8, writer)); // reserved: reserved_1

        ::std::result::Result::Ok(())
    } // fn write_encoded_to()
} // impl Encodable

#[test]
fn test_open_encodable_bytes_written_matches_len() {
    let payload: Open = Default::default();
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



impl ::ProtocolMethodPayload for Open {
    fn class(&self) -> ::Class {
        ::Class::Channel
    }
    fn class_id(&self) -> u16 {
        20
    }
    fn class_name(&self) -> &'static str {
        "channel"
    }
    fn method_id(&self) -> u16 {
        10
    }
    fn method_name(&self) -> &'static str {
        "open"
    }
} // impl ::ProtocolMethodPayload for Open
impl<'a> ::method::channel::SetOpenMethodFields<'a> for Open {} // impl<'a> ::method::channel::SetOpenMethodFields<'a> for Open
impl<'a> From<Open> for ClassMethod<'a> {
    fn from(from: Open) -> Self {
        ClassMethod::Open(from)
    } // fn from()
} // impl From<Open> for ClassMethod

impl From<Open> for super::SpecMethod<'static> {
    fn from(from: Open) -> Self {
        super::SpecMethod::Channel(from.into())
    } // fn default()
} // impl From<Open> for ::super::SpecMethod
impl<'a> ::method::channel::OpenOkMethod<'a> for ::Amqp9_1 {
    type Payload = OpenOk;
} // impl<'a> ::method::channel::OpenOkMethod<'a> for ::Amqp9_1

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct OpenOk;

impl OpenOk {
    pub fn new() -> Self {
        OpenOk
    } // fn new()
} // impl OpenOk
impl Default for OpenOk {
    fn default() -> Self {
        OpenOk::new()
    } // fn default()
} // impl Default for OpenOk

impl ::Encodable for OpenOk {
    fn encoded_size(&self) -> usize {
        4
    } // encoded_size
    fn write_encoded_to<W>(&self, writer: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        try!(::Encodable::write_encoded_to(&0u32, writer)); // reserved: reserved_1

        ::std::result::Result::Ok(())
    } // fn write_encoded_to()
} // impl Encodable

#[test]
fn test_open_ok_encodable_bytes_written_matches_len() {
    let payload: OpenOk = Default::default();
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



impl ::ProtocolMethodPayload for OpenOk {
    fn class(&self) -> ::Class {
        ::Class::Channel
    }
    fn class_id(&self) -> u16 {
        20
    }
    fn class_name(&self) -> &'static str {
        "channel"
    }
    fn method_id(&self) -> u16 {
        11
    }
    fn method_name(&self) -> &'static str {
        "open-ok"
    }
} // impl ::ProtocolMethodPayload for OpenOk
impl<'a> ::method::channel::SetOpenOkMethodFields<'a> for OpenOk {} // impl<'a> ::method::channel::SetOpenOkMethodFields<'a> for OpenOk
impl<'a> From<OpenOk> for ClassMethod<'a> {
    fn from(from: OpenOk) -> Self {
        ClassMethod::OpenOk(from)
    } // fn from()
} // impl From<OpenOk> for ClassMethod

impl From<OpenOk> for super::SpecMethod<'static> {
    fn from(from: OpenOk) -> Self {
        super::SpecMethod::Channel(from.into())
    } // fn default()
} // impl From<OpenOk> for ::super::SpecMethod

#[derive(Debug)]
pub enum ClassMethod<'a> {
    Close(Close<'a>),
    CloseOk(CloseOk),
    Flow(Flow),
    FlowOk(FlowOk),
    Open(Open),
    OpenOk(OpenOk),
} // enum ClassMethod


impl<'a> ::Encodable for ClassMethod<'a> {
    fn encoded_size(&self) -> usize {
        match *self {
            ClassMethod::Close(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::CloseOk(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::Flow(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::FlowOk(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::Open(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::OpenOk(ref method) => ::Encodable::encoded_size(method),

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
            ClassMethod::Close(ref method) => ::ProtocolMethodPayload::class(method),
            ClassMethod::CloseOk(ref method) => ::ProtocolMethodPayload::class(method),
            ClassMethod::Flow(ref method) => ::ProtocolMethodPayload::class(method),
            ClassMethod::FlowOk(ref method) => ::ProtocolMethodPayload::class(method),
            ClassMethod::Open(ref method) => ::ProtocolMethodPayload::class(method),
            ClassMethod::OpenOk(ref method) => ::ProtocolMethodPayload::class(method),

        } // match *self

    } // fn class

    fn class_id(&self) -> u16 {
        match *self {
            ClassMethod::Close(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::CloseOk(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::Flow(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::FlowOk(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::Open(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::OpenOk(ref method) => ::ProtocolMethodPayload::class_id(method),

        } // match *self

    } // fn class_id

    fn class_name(&self) -> &'static str {
        match *self {
            ClassMethod::Close(ref method) => ::ProtocolMethodPayload::class_name(method),
            ClassMethod::CloseOk(ref method) => ::ProtocolMethodPayload::class_name(method),
            ClassMethod::Flow(ref method) => ::ProtocolMethodPayload::class_name(method),
            ClassMethod::FlowOk(ref method) => ::ProtocolMethodPayload::class_name(method),
            ClassMethod::Open(ref method) => ::ProtocolMethodPayload::class_name(method),
            ClassMethod::OpenOk(ref method) => ::ProtocolMethodPayload::class_name(method),

        } // match *self

    } // fn class_name

    fn method_id(&self) -> u16 {
        match *self {
            ClassMethod::Close(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::CloseOk(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::Flow(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::FlowOk(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::Open(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::OpenOk(ref method) => ::ProtocolMethodPayload::method_id(method),

        } // match *self

    } // fn method_id

    fn method_name(&self) -> &'static str {
        match *self {
            ClassMethod::Close(ref method) => ::ProtocolMethodPayload::method_name(method),
            ClassMethod::CloseOk(ref method) => ::ProtocolMethodPayload::method_name(method),
            ClassMethod::Flow(ref method) => ::ProtocolMethodPayload::method_name(method),
            ClassMethod::FlowOk(ref method) => ::ProtocolMethodPayload::method_name(method),
            ClassMethod::Open(ref method) => ::ProtocolMethodPayload::method_name(method),
            ClassMethod::OpenOk(ref method) => ::ProtocolMethodPayload::method_name(method),

        } // match *self

    } // fn method_name
} // impl ProtocolMethodPayload for ClassMethod
