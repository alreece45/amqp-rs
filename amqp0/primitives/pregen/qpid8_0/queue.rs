// Generated by build.rs script in amqp0-primitives
// Pre-generated files are used by default. Generation is done with the amqp0-codegen crate
//
// To regenerate, and not use pre-generated files, use: cargo --features="amqp0-build-primitives"
// To format and replace the pre-generated files, use: cargo --features="amqp0-pregen-primitives"
//
// EDITORS BEWARE: Your modifications may be overridden

// generated by primalgen::codegen::spec_module::class_mod::ClassModuleWriter
#![allow(too_many_arguments)]

impl<'a> ::method::queue::BindMethod<'a> for ::Qpid8_0 {
    type Payload = Bind<'a>;
} // impl<'a> ::method::queue::BindMethod<'a> for ::Qpid8_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Bind<'a> {
    ticket: u16,
    queue: ::std::borrow::Cow<'a, str>,
    exchange: ::std::borrow::Cow<'a, str>,
    routing_key: ::std::borrow::Cow<'a, str>,
    no_wait: bool,
    arguments: ::field::TableEntries<'a>,
} // struct Bind<'a>

impl<'a> Bind<'a> {
    pub fn new<Q, E, R, A>(ticket: u16,
                           queue: Q,
                           exchange: E,
                           routing_key: R,
                           no_wait: bool,
                           arguments: A)
                           -> Self
        where Q: Into<::std::borrow::Cow<'a, str>>,
              E: Into<::std::borrow::Cow<'a, str>>,
              R: Into<::std::borrow::Cow<'a, str>>,
              A: Into<::field::TableEntries<'a>>
    {
        Bind {
            ticket: ticket,
            queue: queue.into(),
            exchange: exchange.into(),
            routing_key: routing_key.into(),
            no_wait: no_wait,
            arguments: arguments.into(),
        } // Bind
    } // fn new()
    impl_properties! {
(ticket, set_ticket) -> u16,
(queue, queue_mut, set_queue) -> Cow<str>,
(exchange, exchange_mut, set_exchange) -> Cow<str>,
(routing_key, routing_key_mut, set_routing_key) -> Cow<str>,
(no_wait, set_no_wait) -> bool,
(arguments, arguments_mut, set_arguments) -> &::field::TableEntries<'a>,
} // impl_properties
} // impl<'a> Bind<'a>
impl<'a> Default for Bind<'a> {
    fn default() -> Self {
        Bind::new(0, "", "", "", false, ::field::TableEntries::new())
    } // fn default()
} // impl Default for Bind

impl<'a> ::Encodable for Bind<'a> {
    fn encoded_size(&self) -> usize {
        3 + ::Encodable::encoded_size(&self.queue) + ::Encodable::encoded_size(&self.exchange) +
        ::Encodable::encoded_size(&self.routing_key) +
        ::Encodable::encoded_size(&self.arguments)
    } // encoded_size
    fn write_encoded_to<W>(&self, writer: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        try!(::Encodable::write_encoded_to(&self.ticket, writer)); // ticket
        try!(::Encodable::write_encoded_to(&self.queue, writer)); // queue
        try!(::Encodable::write_encoded_to(&self.exchange, writer)); // exchange
        try!(::Encodable::write_encoded_to(&self.routing_key, writer)); // routing_key
        try!(::Encodable::write_encoded_to(&{
                                               let mut bits = ::bit_vec::BitVec::from_elem(8,
                                                                                           false);
                                               bits.set(7, self.no_wait);
                                               bits
                                           },
                                           writer));
        try!(::Encodable::write_encoded_to(&self.arguments, writer)); // arguments

        ::std::result::Result::Ok(())
    } // fn write_encoded_to()
} // impl Encodable

#[test]
fn test_bind_encodable_bytes_written_matches_len() {
    let payload: Bind = Default::default();
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



impl<'a> ::ProtocolMethodPayload for Bind<'a> {
    fn class(&self) -> ::Class {
        ::Class::Queue
    }
    fn class_id(&self) -> u16 {
        50
    }
    fn class_name(&self) -> &'static str {
        "queue"
    }
    fn method_id(&self) -> u16 {
        20
    }
    fn method_name(&self) -> &'static str {
        "bind"
    }
} // impl ::ProtocolMethodPayload for Bind<'a>
impl<'a> ::method::queue::SetBindMethodFields<'a> for Bind<'a> {
    fn set_ticket(&mut self, ticket: u16) {
        self.set_ticket(ticket)
    } // set_ticket()
    fn set_queue<V>(&mut self, queue: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_queue(queue.into())
    } // set_queue()
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
    fn set_no_wait(&mut self, no_wait: bool) {
        self.set_no_wait(no_wait)
    } // set_no_wait()
    fn set_arguments<V>(&mut self, arguments: V)
        where V: Into<::field::TableEntries<'a>>
    {
        self.set_arguments(arguments.into())
    } // set_arguments()
} // impl<'a> ::method::queue::SetBindMethodFields<'a> for Bind<'a>
impl<'a> From<Bind<'a>> for ClassMethod<'a> {
    fn from(from: Bind<'a>) -> Self {
        ClassMethod::Bind(from)
    } // fn from()
} // impl From<Bind<'a>> for ClassMethod

impl<'a> From<Bind<'a>> for super::SpecMethod<'a> {
    fn from(from: Bind<'a>) -> Self {
        super::SpecMethod::Queue(from.into())
    } // fn default()
} // impl From<Bind<'a>> for ::super::SpecMethod
impl ::method::queue::BindOkMethod for ::Qpid8_0 {
    type Payload = BindOk;
} // impl ::method::queue::BindOkMethod for ::Qpid8_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct BindOk;

impl BindOk {
    pub fn new() -> Self {
        BindOk
    } // fn new()
} // impl BindOk
impl Default for BindOk {
    fn default() -> Self {
        BindOk::new()
    } // fn default()
} // impl Default for BindOk

impl ::Encodable for BindOk {
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
fn test_bind_ok_encodable_bytes_written_matches_len() {
    let payload: BindOk = Default::default();
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



impl ::ProtocolMethodPayload for BindOk {
    fn class(&self) -> ::Class {
        ::Class::Queue
    }
    fn class_id(&self) -> u16 {
        50
    }
    fn class_name(&self) -> &'static str {
        "queue"
    }
    fn method_id(&self) -> u16 {
        21
    }
    fn method_name(&self) -> &'static str {
        "bind-ok"
    }
} // impl ::ProtocolMethodPayload for BindOk
impl<'a> From<BindOk> for ClassMethod<'a> {
    fn from(from: BindOk) -> Self {
        ClassMethod::BindOk(from)
    } // fn from()
} // impl From<BindOk> for ClassMethod

impl From<BindOk> for super::SpecMethod<'static> {
    fn from(from: BindOk) -> Self {
        super::SpecMethod::Queue(from.into())
    } // fn default()
} // impl From<BindOk> for ::super::SpecMethod
impl<'a> ::method::queue::DeclareMethod<'a> for ::Qpid8_0 {
    type Payload = Declare<'a>;
} // impl<'a> ::method::queue::DeclareMethod<'a> for ::Qpid8_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Declare<'a> {
    ticket: u16,
    queue: ::std::borrow::Cow<'a, str>,
    passive: bool,
    durable: bool,
    exclusive: bool,
    auto_delete: bool,
    no_wait: bool,
    arguments: ::field::TableEntries<'a>,
} // struct Declare<'a>

impl<'a> Declare<'a> {
    pub fn new<Q, A>(ticket: u16,
                     queue: Q,
                     passive: bool,
                     durable: bool,
                     exclusive: bool,
                     auto_delete: bool,
                     no_wait: bool,
                     arguments: A)
                     -> Self
        where Q: Into<::std::borrow::Cow<'a, str>>,
              A: Into<::field::TableEntries<'a>>
    {
        Declare {
            ticket: ticket,
            queue: queue.into(),
            passive: passive,
            durable: durable,
            exclusive: exclusive,
            auto_delete: auto_delete,
            no_wait: no_wait,
            arguments: arguments.into(),
        } // Declare
    } // fn new()
    impl_properties! {
(ticket, set_ticket) -> u16,
(queue, queue_mut, set_queue) -> Cow<str>,
(passive, set_passive) -> bool,
(durable, set_durable) -> bool,
(exclusive, set_exclusive) -> bool,
(auto_delete, set_auto_delete) -> bool,
(no_wait, set_no_wait) -> bool,
(arguments, arguments_mut, set_arguments) -> &::field::TableEntries<'a>,
} // impl_properties
} // impl<'a> Declare<'a>
impl<'a> Default for Declare<'a> {
    fn default() -> Self {
        Declare::new(0,
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
        3 + ::Encodable::encoded_size(&self.queue) + ::Encodable::encoded_size(&self.arguments)
    } // encoded_size
    fn write_encoded_to<W>(&self, writer: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        try!(::Encodable::write_encoded_to(&self.ticket, writer)); // ticket
        try!(::Encodable::write_encoded_to(&self.queue, writer)); // queue
        try!(::Encodable::write_encoded_to(&{
                                               let mut bits = ::bit_vec::BitVec::from_elem(8,
                                                                                           false);
                                               bits.set(7, self.passive);
                                               bits.set(6, self.durable);
                                               bits.set(5, self.exclusive);
                                               bits.set(4, self.auto_delete);
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
    fn class(&self) -> ::Class {
        ::Class::Queue
    }
    fn class_id(&self) -> u16 {
        50
    }
    fn class_name(&self) -> &'static str {
        "queue"
    }
    fn method_id(&self) -> u16 {
        10
    }
    fn method_name(&self) -> &'static str {
        "declare"
    }
} // impl ::ProtocolMethodPayload for Declare<'a>
impl<'a> ::method::queue::SetDeclareMethodFields<'a> for Declare<'a> {
    fn set_ticket(&mut self, ticket: u16) {
        self.set_ticket(ticket)
    } // set_ticket()
    fn set_queue<V>(&mut self, queue: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_queue(queue.into())
    } // set_queue()
    fn set_passive(&mut self, passive: bool) {
        self.set_passive(passive)
    } // set_passive()
    fn set_durable(&mut self, durable: bool) {
        self.set_durable(durable)
    } // set_durable()
    fn set_exclusive(&mut self, exclusive: bool) {
        self.set_exclusive(exclusive)
    } // set_exclusive()
    fn set_auto_delete(&mut self, auto_delete: bool) {
        self.set_auto_delete(auto_delete)
    } // set_auto_delete()
    fn set_no_wait(&mut self, no_wait: bool) {
        self.set_no_wait(no_wait)
    } // set_no_wait()
    fn set_arguments<V>(&mut self, arguments: V)
        where V: Into<::field::TableEntries<'a>>
    {
        self.set_arguments(arguments.into())
    } // set_arguments()
} // impl<'a> ::method::queue::SetDeclareMethodFields<'a> for Declare<'a>
impl<'a> From<Declare<'a>> for ClassMethod<'a> {
    fn from(from: Declare<'a>) -> Self {
        ClassMethod::Declare(from)
    } // fn from()
} // impl From<Declare<'a>> for ClassMethod

impl<'a> From<Declare<'a>> for super::SpecMethod<'a> {
    fn from(from: Declare<'a>) -> Self {
        super::SpecMethod::Queue(from.into())
    } // fn default()
} // impl From<Declare<'a>> for ::super::SpecMethod
impl<'a> ::method::queue::DeclareOkMethod<'a> for ::Qpid8_0 {
    type Payload = DeclareOk<'a>;
} // impl<'a> ::method::queue::DeclareOkMethod<'a> for ::Qpid8_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct DeclareOk<'a> {
    queue: ::std::borrow::Cow<'a, str>,
    message_count: u32,
    consumer_count: u32,
} // struct DeclareOk<'a>

impl<'a> DeclareOk<'a> {
    pub fn new<Q>(queue: Q, message_count: u32, consumer_count: u32) -> Self
        where Q: Into<::std::borrow::Cow<'a, str>>
    {
        DeclareOk {
            queue: queue.into(),
            message_count: message_count,
            consumer_count: consumer_count,
        } // DeclareOk
    } // fn new()
    impl_properties! {
(queue, queue_mut, set_queue) -> Cow<str>,
(message_count, set_message_count) -> u32,
(consumer_count, set_consumer_count) -> u32,
} // impl_properties
} // impl<'a> DeclareOk<'a>
impl<'a> Default for DeclareOk<'a> {
    fn default() -> Self {
        DeclareOk::new("", 0, 0)
    } // fn default()
} // impl Default for DeclareOk

impl<'a> ::Encodable for DeclareOk<'a> {
    fn encoded_size(&self) -> usize {
        8 + ::Encodable::encoded_size(&self.queue)
    } // encoded_size
    fn write_encoded_to<W>(&self, writer: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        try!(::Encodable::write_encoded_to(&self.queue, writer)); // queue
        try!(::Encodable::write_encoded_to(&self.message_count, writer)); // message_count
        try!(::Encodable::write_encoded_to(&self.consumer_count, writer)); // consumer_count

        ::std::result::Result::Ok(())
    } // fn write_encoded_to()
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



impl<'a> ::ProtocolMethodPayload for DeclareOk<'a> {
    fn class(&self) -> ::Class {
        ::Class::Queue
    }
    fn class_id(&self) -> u16 {
        50
    }
    fn class_name(&self) -> &'static str {
        "queue"
    }
    fn method_id(&self) -> u16 {
        11
    }
    fn method_name(&self) -> &'static str {
        "declare-ok"
    }
} // impl ::ProtocolMethodPayload for DeclareOk<'a>
impl<'a> ::method::queue::SetDeclareOkMethodFields<'a> for DeclareOk<'a> {
    fn set_queue<V>(&mut self, queue: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_queue(queue.into())
    } // set_queue()
    fn set_message_count(&mut self, message_count: u32) {
        self.set_message_count(message_count)
    } // set_message_count()
    fn set_consumer_count(&mut self, consumer_count: u32) {
        self.set_consumer_count(consumer_count)
    } // set_consumer_count()
} // impl<'a> ::method::queue::SetDeclareOkMethodFields<'a> for DeclareOk<'a>
impl<'a> From<DeclareOk<'a>> for ClassMethod<'a> {
    fn from(from: DeclareOk<'a>) -> Self {
        ClassMethod::DeclareOk(from)
    } // fn from()
} // impl From<DeclareOk<'a>> for ClassMethod

impl<'a> From<DeclareOk<'a>> for super::SpecMethod<'a> {
    fn from(from: DeclareOk<'a>) -> Self {
        super::SpecMethod::Queue(from.into())
    } // fn default()
} // impl From<DeclareOk<'a>> for ::super::SpecMethod
impl<'a> ::method::queue::DeleteMethod<'a> for ::Qpid8_0 {
    type Payload = Delete<'a>;
} // impl<'a> ::method::queue::DeleteMethod<'a> for ::Qpid8_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Delete<'a> {
    ticket: u16,
    queue: ::std::borrow::Cow<'a, str>,
    if_unused: bool,
    if_empty: bool,
    no_wait: bool,
} // struct Delete<'a>

impl<'a> Delete<'a> {
    pub fn new<Q>(ticket: u16, queue: Q, if_unused: bool, if_empty: bool, no_wait: bool) -> Self
        where Q: Into<::std::borrow::Cow<'a, str>>
    {
        Delete {
            ticket: ticket,
            queue: queue.into(),
            if_unused: if_unused,
            if_empty: if_empty,
            no_wait: no_wait,
        } // Delete
    } // fn new()
    impl_properties! {
(ticket, set_ticket) -> u16,
(queue, queue_mut, set_queue) -> Cow<str>,
(if_unused, set_if_unused) -> bool,
(if_empty, set_if_empty) -> bool,
(no_wait, set_no_wait) -> bool,
} // impl_properties
} // impl<'a> Delete<'a>
impl<'a> Default for Delete<'a> {
    fn default() -> Self {
        Delete::new(0, "", false, false, false)
    } // fn default()
} // impl Default for Delete

impl<'a> ::Encodable for Delete<'a> {
    fn encoded_size(&self) -> usize {
        3 + ::Encodable::encoded_size(&self.queue)
    } // encoded_size
    fn write_encoded_to<W>(&self, writer: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        try!(::Encodable::write_encoded_to(&self.ticket, writer)); // ticket
        try!(::Encodable::write_encoded_to(&self.queue, writer)); // queue
        try!(::Encodable::write_encoded_to(&{
                                               let mut bits = ::bit_vec::BitVec::from_elem(8,
                                                                                           false);
                                               bits.set(7, self.if_unused);
                                               bits.set(6, self.if_empty);
                                               bits.set(5, self.no_wait);
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
    fn class(&self) -> ::Class {
        ::Class::Queue
    }
    fn class_id(&self) -> u16 {
        50
    }
    fn class_name(&self) -> &'static str {
        "queue"
    }
    fn method_id(&self) -> u16 {
        40
    }
    fn method_name(&self) -> &'static str {
        "delete"
    }
} // impl ::ProtocolMethodPayload for Delete<'a>
impl<'a> ::method::queue::SetDeleteMethodFields<'a> for Delete<'a> {
    fn set_ticket(&mut self, ticket: u16) {
        self.set_ticket(ticket)
    } // set_ticket()
    fn set_queue<V>(&mut self, queue: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_queue(queue.into())
    } // set_queue()
    fn set_if_unused(&mut self, if_unused: bool) {
        self.set_if_unused(if_unused)
    } // set_if_unused()
    fn set_if_empty(&mut self, if_empty: bool) {
        self.set_if_empty(if_empty)
    } // set_if_empty()
    fn set_no_wait(&mut self, no_wait: bool) {
        self.set_no_wait(no_wait)
    } // set_no_wait()
} // impl<'a> ::method::queue::SetDeleteMethodFields<'a> for Delete<'a>
impl<'a> From<Delete<'a>> for ClassMethod<'a> {
    fn from(from: Delete<'a>) -> Self {
        ClassMethod::Delete(from)
    } // fn from()
} // impl From<Delete<'a>> for ClassMethod

impl<'a> From<Delete<'a>> for super::SpecMethod<'a> {
    fn from(from: Delete<'a>) -> Self {
        super::SpecMethod::Queue(from.into())
    } // fn default()
} // impl From<Delete<'a>> for ::super::SpecMethod
impl ::method::queue::DeleteOkMethod for ::Qpid8_0 {
    type Payload = DeleteOk;
} // impl ::method::queue::DeleteOkMethod for ::Qpid8_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct DeleteOk {
    message_count: u32,
} // struct DeleteOk

impl DeleteOk {
    pub fn new(message_count: u32) -> Self {
        DeleteOk { message_count: message_count } // DeleteOk
    } // fn new()
    impl_properties! {
(message_count, set_message_count) -> u32,
} // impl_properties
} // impl DeleteOk
impl Default for DeleteOk {
    fn default() -> Self {
        DeleteOk::new(0)
    } // fn default()
} // impl Default for DeleteOk

impl ::Encodable for DeleteOk {
    fn encoded_size(&self) -> usize {
        4
    } // encoded_size
    fn write_encoded_to<W>(&self, writer: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        try!(::Encodable::write_encoded_to(&self.message_count, writer)); // message_count

        ::std::result::Result::Ok(())
    } // fn write_encoded_to()
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
    fn class(&self) -> ::Class {
        ::Class::Queue
    }
    fn class_id(&self) -> u16 {
        50
    }
    fn class_name(&self) -> &'static str {
        "queue"
    }
    fn method_id(&self) -> u16 {
        41
    }
    fn method_name(&self) -> &'static str {
        "delete-ok"
    }
} // impl ::ProtocolMethodPayload for DeleteOk
impl ::method::queue::SetDeleteOkMethodFields for DeleteOk {
    fn set_message_count(&mut self, message_count: u32) {
        self.set_message_count(message_count)
    } // set_message_count()
} // impl ::method::queue::SetDeleteOkMethodFields for DeleteOk
impl<'a> From<DeleteOk> for ClassMethod<'a> {
    fn from(from: DeleteOk) -> Self {
        ClassMethod::DeleteOk(from)
    } // fn from()
} // impl From<DeleteOk> for ClassMethod

impl From<DeleteOk> for super::SpecMethod<'static> {
    fn from(from: DeleteOk) -> Self {
        super::SpecMethod::Queue(from.into())
    } // fn default()
} // impl From<DeleteOk> for ::super::SpecMethod
impl<'a> ::method::queue::PurgeMethod<'a> for ::Qpid8_0 {
    type Payload = Purge<'a>;
} // impl<'a> ::method::queue::PurgeMethod<'a> for ::Qpid8_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Purge<'a> {
    ticket: u16,
    queue: ::std::borrow::Cow<'a, str>,
    no_wait: bool,
} // struct Purge<'a>

impl<'a> Purge<'a> {
    pub fn new<Q>(ticket: u16, queue: Q, no_wait: bool) -> Self
        where Q: Into<::std::borrow::Cow<'a, str>>
    {
        Purge {
            ticket: ticket,
            queue: queue.into(),
            no_wait: no_wait,
        } // Purge
    } // fn new()
    impl_properties! {
(ticket, set_ticket) -> u16,
(queue, queue_mut, set_queue) -> Cow<str>,
(no_wait, set_no_wait) -> bool,
} // impl_properties
} // impl<'a> Purge<'a>
impl<'a> Default for Purge<'a> {
    fn default() -> Self {
        Purge::new(0, "", false)
    } // fn default()
} // impl Default for Purge

impl<'a> ::Encodable for Purge<'a> {
    fn encoded_size(&self) -> usize {
        3 + ::Encodable::encoded_size(&self.queue)
    } // encoded_size
    fn write_encoded_to<W>(&self, writer: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        try!(::Encodable::write_encoded_to(&self.ticket, writer)); // ticket
        try!(::Encodable::write_encoded_to(&self.queue, writer)); // queue
        try!(::Encodable::write_encoded_to(&{
                                               let mut bits = ::bit_vec::BitVec::from_elem(8,
                                                                                           false);
                                               bits.set(7, self.no_wait);
                                               bits
                                           },
                                           writer));

        ::std::result::Result::Ok(())
    } // fn write_encoded_to()
} // impl Encodable

#[test]
fn test_purge_encodable_bytes_written_matches_len() {
    let payload: Purge = Default::default();
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



impl<'a> ::ProtocolMethodPayload for Purge<'a> {
    fn class(&self) -> ::Class {
        ::Class::Queue
    }
    fn class_id(&self) -> u16 {
        50
    }
    fn class_name(&self) -> &'static str {
        "queue"
    }
    fn method_id(&self) -> u16 {
        30
    }
    fn method_name(&self) -> &'static str {
        "purge"
    }
} // impl ::ProtocolMethodPayload for Purge<'a>
impl<'a> ::method::queue::SetPurgeMethodFields<'a> for Purge<'a> {
    fn set_ticket(&mut self, ticket: u16) {
        self.set_ticket(ticket)
    } // set_ticket()
    fn set_queue<V>(&mut self, queue: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_queue(queue.into())
    } // set_queue()
    fn set_no_wait(&mut self, no_wait: bool) {
        self.set_no_wait(no_wait)
    } // set_no_wait()
} // impl<'a> ::method::queue::SetPurgeMethodFields<'a> for Purge<'a>
impl<'a> From<Purge<'a>> for ClassMethod<'a> {
    fn from(from: Purge<'a>) -> Self {
        ClassMethod::Purge(from)
    } // fn from()
} // impl From<Purge<'a>> for ClassMethod

impl<'a> From<Purge<'a>> for super::SpecMethod<'a> {
    fn from(from: Purge<'a>) -> Self {
        super::SpecMethod::Queue(from.into())
    } // fn default()
} // impl From<Purge<'a>> for ::super::SpecMethod
impl ::method::queue::PurgeOkMethod for ::Qpid8_0 {
    type Payload = PurgeOk;
} // impl ::method::queue::PurgeOkMethod for ::Qpid8_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct PurgeOk {
    message_count: u32,
} // struct PurgeOk

impl PurgeOk {
    pub fn new(message_count: u32) -> Self {
        PurgeOk { message_count: message_count } // PurgeOk
    } // fn new()
    impl_properties! {
(message_count, set_message_count) -> u32,
} // impl_properties
} // impl PurgeOk
impl Default for PurgeOk {
    fn default() -> Self {
        PurgeOk::new(0)
    } // fn default()
} // impl Default for PurgeOk

impl ::Encodable for PurgeOk {
    fn encoded_size(&self) -> usize {
        4
    } // encoded_size
    fn write_encoded_to<W>(&self, writer: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        try!(::Encodable::write_encoded_to(&self.message_count, writer)); // message_count

        ::std::result::Result::Ok(())
    } // fn write_encoded_to()
} // impl Encodable

#[test]
fn test_purge_ok_encodable_bytes_written_matches_len() {
    let payload: PurgeOk = Default::default();
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



impl ::ProtocolMethodPayload for PurgeOk {
    fn class(&self) -> ::Class {
        ::Class::Queue
    }
    fn class_id(&self) -> u16 {
        50
    }
    fn class_name(&self) -> &'static str {
        "queue"
    }
    fn method_id(&self) -> u16 {
        31
    }
    fn method_name(&self) -> &'static str {
        "purge-ok"
    }
} // impl ::ProtocolMethodPayload for PurgeOk
impl ::method::queue::SetPurgeOkMethodFields for PurgeOk {
    fn set_message_count(&mut self, message_count: u32) {
        self.set_message_count(message_count)
    } // set_message_count()
} // impl ::method::queue::SetPurgeOkMethodFields for PurgeOk
impl<'a> From<PurgeOk> for ClassMethod<'a> {
    fn from(from: PurgeOk) -> Self {
        ClassMethod::PurgeOk(from)
    } // fn from()
} // impl From<PurgeOk> for ClassMethod

impl From<PurgeOk> for super::SpecMethod<'static> {
    fn from(from: PurgeOk) -> Self {
        super::SpecMethod::Queue(from.into())
    } // fn default()
} // impl From<PurgeOk> for ::super::SpecMethod

#[derive(Debug)]
pub enum ClassMethod<'a> {
    Bind(Bind<'a>),
    BindOk(BindOk),
    Declare(Declare<'a>),
    DeclareOk(DeclareOk<'a>),
    Delete(Delete<'a>),
    DeleteOk(DeleteOk),
    Purge(Purge<'a>),
    PurgeOk(PurgeOk),
} // enum ClassMethod


impl<'a> ::Encodable for ClassMethod<'a> {
    fn encoded_size(&self) -> usize {
        match *self {
            ClassMethod::Bind(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::BindOk(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::Declare(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::DeclareOk(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::Delete(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::DeleteOk(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::Purge(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::PurgeOk(ref method) => ::Encodable::encoded_size(method),

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
            ClassMethod::Bind(ref method) => ::ProtocolMethodPayload::class(method),
            ClassMethod::BindOk(ref method) => ::ProtocolMethodPayload::class(method),
            ClassMethod::Declare(ref method) => ::ProtocolMethodPayload::class(method),
            ClassMethod::DeclareOk(ref method) => ::ProtocolMethodPayload::class(method),
            ClassMethod::Delete(ref method) => ::ProtocolMethodPayload::class(method),
            ClassMethod::DeleteOk(ref method) => ::ProtocolMethodPayload::class(method),
            ClassMethod::Purge(ref method) => ::ProtocolMethodPayload::class(method),
            ClassMethod::PurgeOk(ref method) => ::ProtocolMethodPayload::class(method),

        } // match *self

    } // fn class

    fn class_id(&self) -> u16 {
        match *self {
            ClassMethod::Bind(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::BindOk(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::Declare(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::DeclareOk(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::Delete(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::DeleteOk(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::Purge(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::PurgeOk(ref method) => ::ProtocolMethodPayload::class_id(method),

        } // match *self

    } // fn class_id

    fn class_name(&self) -> &'static str {
        match *self {
            ClassMethod::Bind(ref method) => ::ProtocolMethodPayload::class_name(method),
            ClassMethod::BindOk(ref method) => ::ProtocolMethodPayload::class_name(method),
            ClassMethod::Declare(ref method) => ::ProtocolMethodPayload::class_name(method),
            ClassMethod::DeclareOk(ref method) => ::ProtocolMethodPayload::class_name(method),
            ClassMethod::Delete(ref method) => ::ProtocolMethodPayload::class_name(method),
            ClassMethod::DeleteOk(ref method) => ::ProtocolMethodPayload::class_name(method),
            ClassMethod::Purge(ref method) => ::ProtocolMethodPayload::class_name(method),
            ClassMethod::PurgeOk(ref method) => ::ProtocolMethodPayload::class_name(method),

        } // match *self

    } // fn class_name

    fn method_id(&self) -> u16 {
        match *self {
            ClassMethod::Bind(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::BindOk(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::Declare(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::DeclareOk(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::Delete(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::DeleteOk(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::Purge(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::PurgeOk(ref method) => ::ProtocolMethodPayload::method_id(method),

        } // match *self

    } // fn method_id

    fn method_name(&self) -> &'static str {
        match *self {
            ClassMethod::Bind(ref method) => ::ProtocolMethodPayload::method_name(method),
            ClassMethod::BindOk(ref method) => ::ProtocolMethodPayload::method_name(method),
            ClassMethod::Declare(ref method) => ::ProtocolMethodPayload::method_name(method),
            ClassMethod::DeclareOk(ref method) => ::ProtocolMethodPayload::method_name(method),
            ClassMethod::Delete(ref method) => ::ProtocolMethodPayload::method_name(method),
            ClassMethod::DeleteOk(ref method) => ::ProtocolMethodPayload::method_name(method),
            ClassMethod::Purge(ref method) => ::ProtocolMethodPayload::method_name(method),
            ClassMethod::PurgeOk(ref method) => ::ProtocolMethodPayload::method_name(method),

        } // match *self

    } // fn method_name
} // impl ProtocolMethodPayload for ClassMethod
