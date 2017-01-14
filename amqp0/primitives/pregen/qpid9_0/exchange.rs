// Generated by build.rs script in amqp0-primitives
// Pre-generated files are used by default. Generation is done with the amqp0-codegen crate
//
// To regenerate, and not use pre-generated files, use: cargo --features="amqp0-build-primitives"
// To format and replace the pre-generated files, use: cargo --features="amqp0-pregen-primitives"
//
// EDITORS BEWARE: Your modifications may be overridden

// generated by primalgen::codegen::spec_module::class_mod::ClassModuleWriter
#![allow(too_many_arguments)]

impl<'a> ::method::exchange::BoundMethod<'a> for ::Qpid9_0 {
    type Payload = Bound<'a>;
} // impl<'a> ::method::exchange::BoundMethod<'a> for ::Qpid9_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Bound<'a> {
    exchange: ::std::borrow::Cow<'a, str>,
    routing_key: ::std::borrow::Cow<'a, str>,
    queue: ::std::borrow::Cow<'a, str>,
} // struct Bound<'a>

impl<'a> Bound<'a> {
    pub fn new<E, R, Q>(exchange: E, routing_key: R, queue: Q) -> Self
        where E: Into<::std::borrow::Cow<'a, str>>,
              R: Into<::std::borrow::Cow<'a, str>>,
              Q: Into<::std::borrow::Cow<'a, str>>
    {
        Bound {
            exchange: exchange.into(),
            routing_key: routing_key.into(),
            queue: queue.into(),
        } // Bound
    } // fn new()
    impl_properties! {
(exchange, exchange_mut, set_exchange) -> Cow<str>,
(routing_key, routing_key_mut, set_routing_key) -> Cow<str>,
(queue, queue_mut, set_queue) -> Cow<str>,
} // impl_properties
} // impl<'a> Bound<'a>
impl<'a> Default for Bound<'a> {
    fn default() -> Self {
        Bound::new("", "", "")
    } // fn default()
} // impl Default for Bound

impl<'a> ::Encodable for Bound<'a> {
    fn encoded_size(&self) -> usize {
        0 + ::Encodable::encoded_size(&self.exchange) +
        ::Encodable::encoded_size(&self.routing_key) +
        ::Encodable::encoded_size(&self.queue)
    } // encoded_size
    fn write_encoded_to<W>(&self, writer: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        try!(::Encodable::write_encoded_to(&self.exchange, writer)); // exchange
        try!(::Encodable::write_encoded_to(&self.routing_key, writer)); // routing_key
        try!(::Encodable::write_encoded_to(&self.queue, writer)); // queue

        ::std::result::Result::Ok(())
    } // fn write_encoded_to()
} // impl Encodable

#[test]
fn test_bound_encodable_bytes_written_matches_len() {
    let payload: Bound = Default::default();
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



impl<'a> ::ProtocolMethodPayload for Bound<'a> {
    fn class_id(&self) -> u16 {
        40
    } // fn class_id()
    fn method_id(&self) -> u16 {
        22
    } // fn method_id()
} // impl ::Payload for Bound
impl<'a> ::method::exchange::SetBoundMethodFields<'a> for Bound<'a> {
    fn set_exchange<V>(&mut self, exchange: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_exchange(exchange.into())
    } // set_exchange()
    fn set_routing_key<V>(&mut self, routing_key: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_routing_key(routing_key.into())
    } // set_routing_key()
    fn set_queue<V>(&mut self, queue: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_queue(queue.into())
    } // set_queue()
} // impl<'a> ::method::exchange::SetBoundMethodFields<'a> for Bound<'a>
impl<'a> ::method::exchange::BoundOkMethod<'a> for ::Qpid9_0 {
    type Payload = BoundOk<'a>;
} // impl<'a> ::method::exchange::BoundOkMethod<'a> for ::Qpid9_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct BoundOk<'a> {
    reply_code: u16,
    reply_text: ::std::borrow::Cow<'a, str>,
} // struct BoundOk<'a>

impl<'a> BoundOk<'a> {
    pub fn new<R>(reply_code: u16, reply_text: R) -> Self
        where R: Into<::std::borrow::Cow<'a, str>>
    {
        BoundOk {
            reply_code: reply_code,
            reply_text: reply_text.into(),
        } // BoundOk
    } // fn new()
    impl_properties! {
(reply_code, set_reply_code) -> u16,
(reply_text, reply_text_mut, set_reply_text) -> Cow<str>,
} // impl_properties
} // impl<'a> BoundOk<'a>
impl<'a> Default for BoundOk<'a> {
    fn default() -> Self {
        BoundOk::new(0, "")
    } // fn default()
} // impl Default for BoundOk

impl<'a> ::Encodable for BoundOk<'a> {
    fn encoded_size(&self) -> usize {
        2 + ::Encodable::encoded_size(&self.reply_text)
    } // encoded_size
    fn write_encoded_to<W>(&self, writer: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        try!(::Encodable::write_encoded_to(&self.reply_code, writer)); // reply_code
        try!(::Encodable::write_encoded_to(&self.reply_text, writer)); // reply_text

        ::std::result::Result::Ok(())
    } // fn write_encoded_to()
} // impl Encodable

#[test]
fn test_bound_ok_encodable_bytes_written_matches_len() {
    let payload: BoundOk = Default::default();
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



impl<'a> ::ProtocolMethodPayload for BoundOk<'a> {
    fn class_id(&self) -> u16 {
        40
    } // fn class_id()
    fn method_id(&self) -> u16 {
        23
    } // fn method_id()
} // impl ::Payload for BoundOk
impl<'a> ::method::exchange::SetBoundOkMethodFields<'a> for BoundOk<'a> {
    fn set_reply_code(&mut self, reply_code: u16) {
        self.set_reply_code(reply_code)
    } // set_reply_code()
    fn set_reply_text<V>(&mut self, reply_text: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_reply_text(reply_text.into())
    } // set_reply_text()
} // impl<'a> ::method::exchange::SetBoundOkMethodFields<'a> for BoundOk<'a>
impl<'a> ::method::exchange::DeclareMethod<'a> for ::Qpid9_0 {
    type Payload = Declare<'a>;
} // impl<'a> ::method::exchange::DeclareMethod<'a> for ::Qpid9_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Declare<'a> {
    ticket: u16,
    exchange: ::std::borrow::Cow<'a, str>,
    ty: ::std::borrow::Cow<'a, str>,
    passive: bool,
    durable: bool,
    auto_delete: bool,
    internal: bool,
    no_wait: bool,
    arguments: ::field::TableEntries<'a>,
} // struct Declare<'a>

impl<'a> Declare<'a> {
    pub fn new<E, T, A>(ticket: u16,
                        exchange: E,
                        ty: T,
                        passive: bool,
                        durable: bool,
                        auto_delete: bool,
                        internal: bool,
                        no_wait: bool,
                        arguments: A)
                        -> Self
        where E: Into<::std::borrow::Cow<'a, str>>,
              T: Into<::std::borrow::Cow<'a, str>>,
              A: Into<::field::TableEntries<'a>>
    {
        Declare {
            ticket: ticket,
            exchange: exchange.into(),
            ty: ty.into(),
            passive: passive,
            durable: durable,
            auto_delete: auto_delete,
            internal: internal,
            no_wait: no_wait,
            arguments: arguments.into(),
        } // Declare
    } // fn new()
    impl_properties! {
(ticket, set_ticket) -> u16,
(exchange, exchange_mut, set_exchange) -> Cow<str>,
(ty, ty_mut, set_ty) -> Cow<str>,
(passive, set_passive) -> bool,
(durable, set_durable) -> bool,
(auto_delete, set_auto_delete) -> bool,
(internal, set_internal) -> bool,
(no_wait, set_no_wait) -> bool,
(arguments, arguments_mut, set_arguments) -> &::field::TableEntries<'a>,
} // impl_properties
} // impl<'a> Declare<'a>
impl<'a> Default for Declare<'a> {
    fn default() -> Self {
        Declare::new(0,
                     "",
                     "",
                     false,
                     false,
                     false,
                     false,
                     false,
                     ::field::TableEntries::new())
    } // fn default()
} // impl Default for Declare

impl<'a> ::Encodable for Declare<'a> {
    fn encoded_size(&self) -> usize {
        3 + ::Encodable::encoded_size(&self.exchange) + ::Encodable::encoded_size(&self.ty) +
        ::Encodable::encoded_size(&self.arguments)
    } // encoded_size
    fn write_encoded_to<W>(&self, writer: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        try!(::Encodable::write_encoded_to(&self.ticket, writer)); // ticket
        try!(::Encodable::write_encoded_to(&self.exchange, writer)); // exchange
        try!(::Encodable::write_encoded_to(&self.ty, writer)); // ty
        try!(::Encodable::write_encoded_to(&{
                                               let mut bits = ::bit_vec::BitVec::from_elem(8,
                                                                                           false);
                                               bits.set(7, self.passive);
                                               bits.set(6, self.durable);
                                               bits.set(5, self.auto_delete);
                                               bits.set(4, self.internal);
                                               bits.set(3, self.no_wait);
                                               bits
                                           },
                                           writer));
        try!(::Encodable::write_encoded_to(&self.arguments, writer)); // arguments

        ::std::result::Result::Ok(())
    } // fn write_encoded_to()
} // impl Encodable

#[test]
fn test_declare_encodable_bytes_written_matches_len() {
    let payload: Declare = Default::default();
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



impl<'a> ::ProtocolMethodPayload for Declare<'a> {
    fn class_id(&self) -> u16 {
        40
    } // fn class_id()
    fn method_id(&self) -> u16 {
        10
    } // fn method_id()
} // impl ::Payload for Declare
impl<'a> ::method::exchange::SetDeclareMethodFields<'a> for Declare<'a> {
    fn set_ticket(&mut self, ticket: u16) {
        self.set_ticket(ticket)
    } // set_ticket()
    fn set_exchange<V>(&mut self, exchange: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_exchange(exchange.into())
    } // set_exchange()
    fn set_ty<V>(&mut self, ty: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_ty(ty.into())
    } // set_ty()
    fn set_passive(&mut self, passive: bool) {
        self.set_passive(passive)
    } // set_passive()
    fn set_durable(&mut self, durable: bool) {
        self.set_durable(durable)
    } // set_durable()
    fn set_auto_delete(&mut self, auto_delete: bool) {
        self.set_auto_delete(auto_delete)
    } // set_auto_delete()
    fn set_internal(&mut self, internal: bool) {
        self.set_internal(internal)
    } // set_internal()
    fn set_no_wait(&mut self, no_wait: bool) {
        self.set_no_wait(no_wait)
    } // set_no_wait()
    fn set_arguments<V>(&mut self, arguments: V)
        where V: Into<::field::TableEntries<'a>>
    {
        self.set_arguments(arguments.into())
    } // set_arguments()
} // impl<'a> ::method::exchange::SetDeclareMethodFields<'a> for Declare<'a>
impl ::method::exchange::DeclareOkMethod for ::Qpid9_0 {
    type Payload = DeclareOk;
} // impl ::method::exchange::DeclareOkMethod for ::Qpid9_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct DeclareOk;

impl DeclareOk {
    pub fn new() -> Self {
        DeclareOk
    } // fn new()
} // impl DeclareOk
impl Default for DeclareOk {
    fn default() -> Self {
        DeclareOk::new()
    } // fn default()
} // impl Default for DeclareOk

impl ::Encodable for DeclareOk {
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
fn test_declare_ok_encodable_bytes_written_matches_len() {
    let payload: DeclareOk = Default::default();
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



impl ::ProtocolMethodPayload for DeclareOk {
    fn class_id(&self) -> u16 {
        40
    } // fn class_id()
    fn method_id(&self) -> u16 {
        11
    } // fn method_id()
} // impl ::Payload for DeclareOk
impl<'a> ::method::exchange::DeleteMethod<'a> for ::Qpid9_0 {
    type Payload = Delete<'a>;
} // impl<'a> ::method::exchange::DeleteMethod<'a> for ::Qpid9_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Delete<'a> {
    ticket: u16,
    exchange: ::std::borrow::Cow<'a, str>,
    if_unused: bool,
    no_wait: bool,
} // struct Delete<'a>

impl<'a> Delete<'a> {
    pub fn new<E>(ticket: u16, exchange: E, if_unused: bool, no_wait: bool) -> Self
        where E: Into<::std::borrow::Cow<'a, str>>
    {
        Delete {
            ticket: ticket,
            exchange: exchange.into(),
            if_unused: if_unused,
            no_wait: no_wait,
        } // Delete
    } // fn new()
    impl_properties! {
(ticket, set_ticket) -> u16,
(exchange, exchange_mut, set_exchange) -> Cow<str>,
(if_unused, set_if_unused) -> bool,
(no_wait, set_no_wait) -> bool,
} // impl_properties
} // impl<'a> Delete<'a>
impl<'a> Default for Delete<'a> {
    fn default() -> Self {
        Delete::new(0, "", false, false)
    } // fn default()
} // impl Default for Delete

impl<'a> ::Encodable for Delete<'a> {
    fn encoded_size(&self) -> usize {
        3 + ::Encodable::encoded_size(&self.exchange)
    } // encoded_size
    fn write_encoded_to<W>(&self, writer: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        try!(::Encodable::write_encoded_to(&self.ticket, writer)); // ticket
        try!(::Encodable::write_encoded_to(&self.exchange, writer)); // exchange
        try!(::Encodable::write_encoded_to(&{
                                               let mut bits = ::bit_vec::BitVec::from_elem(8,
                                                                                           false);
                                               bits.set(7, self.if_unused);
                                               bits.set(6, self.no_wait);
                                               bits
                                           },
                                           writer));

        ::std::result::Result::Ok(())
    } // fn write_encoded_to()
} // impl Encodable

#[test]
fn test_delete_encodable_bytes_written_matches_len() {
    let payload: Delete = Default::default();
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



impl<'a> ::ProtocolMethodPayload for Delete<'a> {
    fn class_id(&self) -> u16 {
        40
    } // fn class_id()
    fn method_id(&self) -> u16 {
        20
    } // fn method_id()
} // impl ::Payload for Delete
impl<'a> ::method::exchange::SetDeleteMethodFields<'a> for Delete<'a> {
    fn set_ticket(&mut self, ticket: u16) {
        self.set_ticket(ticket)
    } // set_ticket()
    fn set_exchange<V>(&mut self, exchange: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_exchange(exchange.into())
    } // set_exchange()
    fn set_if_unused(&mut self, if_unused: bool) {
        self.set_if_unused(if_unused)
    } // set_if_unused()
    fn set_no_wait(&mut self, no_wait: bool) {
        self.set_no_wait(no_wait)
    } // set_no_wait()
} // impl<'a> ::method::exchange::SetDeleteMethodFields<'a> for Delete<'a>
impl ::method::exchange::DeleteOkMethod for ::Qpid9_0 {
    type Payload = DeleteOk;
} // impl ::method::exchange::DeleteOkMethod for ::Qpid9_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct DeleteOk;

impl DeleteOk {
    pub fn new() -> Self {
        DeleteOk
    } // fn new()
} // impl DeleteOk
impl Default for DeleteOk {
    fn default() -> Self {
        DeleteOk::new()
    } // fn default()
} // impl Default for DeleteOk

impl ::Encodable for DeleteOk {
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
fn test_delete_ok_encodable_bytes_written_matches_len() {
    let payload: DeleteOk = Default::default();
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



impl ::ProtocolMethodPayload for DeleteOk {
    fn class_id(&self) -> u16 {
        40
    } // fn class_id()
    fn method_id(&self) -> u16 {
        21
    } // fn method_id()
} // impl ::Payload for DeleteOk

#[derive(Debug)]
pub enum ClassMethod<'a> {
    Bound(Bound<'a>),
    BoundOk(BoundOk<'a>),
    Declare(Declare<'a>),
    DeclareOk(DeclareOk),
    Delete(Delete<'a>),
    DeleteOk(DeleteOk),
} // enum ClassMethod


impl<'a> ::Encodable for ClassMethod<'a> {
    fn encoded_size(&self) -> usize {
        match *self {
            ClassMethod::Bound(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::BoundOk(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::Declare(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::DeclareOk(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::Delete(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::DeleteOk(ref method) => ::Encodable::encoded_size(method),

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
            ClassMethod::Bound(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::BoundOk(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::Declare(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::DeclareOk(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::Delete(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::DeleteOk(ref method) => ::ProtocolMethodPayload::class_id(method),

        } // match *self

    } // fn class_id

    fn method_id(&self) -> u16 {
        match *self {
            ClassMethod::Bound(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::BoundOk(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::Declare(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::DeclareOk(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::Delete(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::DeleteOk(ref method) => ::ProtocolMethodPayload::method_id(method),

        } // match *self

    } // fn method_id
} // impl ProtocolMethodPayload for ClassMethod
