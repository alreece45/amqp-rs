// Generated by build.rs script in amqp0-primitives
// Pre-generated files are used by default. Generation is done with the amqp0-codegen crate
//
// To regenerate, and not use pre-generated files, use: cargo --features="amqp0-build-primitives"
// To format and replace the pre-generated files, use: cargo --features="amqp0-pregen-primitives"
//
// EDITORS BEWARE: Your modifications may be overridden

// generated by primalgen::codegen::spec_module::class_mod::ClassModuleWriter
#![allow(too_many_arguments)]


// Generated by primalgen::spec::frame_payload_enum::ClassEnumWriter
#[derive(Debug)]
pub struct Header<'a> {
    content_type: Option<::std::borrow::Cow<'a, str>>,
    content_encoding: Option<::std::borrow::Cow<'a, str>>,
    headers: Option<::field::TableEntries<'a>>,
    priority: Option<u8>,
    timestamp: Option<u64>,
} // struct Header

impl<'a> Header<'a> {
    fn flag_bits(&self) -> ::bit_vec::BitVec {
        let mut flags = ::bit_vec::BitVec::from_elem(8, false);
        flags.set(0, self.content_type.is_some());
        flags.set(1, self.content_encoding.is_some());
        flags.set(2, self.headers.is_some());
        flags.set(3, self.priority.is_some());
        flags.set(4, self.timestamp.is_some());
        flags
    } // fn flag_bits()
    impl_properties! {
(content_type, content_type_mut, set_content_type, take_content_type) -> Option< Cow<str> >,
(content_encoding, content_encoding_mut, set_content_encoding, take_content_encoding) -> Option< Cow<str> >,
(headers, headers_mut, set_headers, take_headers) -> Option< &::field::TableEntries<'a> >,
(priority, priority_mut, set_priority, take_priority) -> Option< u8 >,
(timestamp, timestamp_mut, set_timestamp, take_timestamp) -> Option< u64 >,
} // impl_properties
} // impl Headers

impl<'a> ::Encodable for Header<'a> {
    fn encoded_size(&self) -> usize {
        16 + ::Encodable::encoded_size(&self.content_type) +
        ::Encodable::encoded_size(&self.content_encoding) +
        ::Encodable::encoded_size(&self.headers)
    } // encoded_size
    fn write_encoded_to<W>(&self, writer: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        try!(::Encodable::write_encoded_to(&self.flag_bits(), writer));

        try!(::Encodable::write_encoded_to(&self.content_type, writer));
        try!(::Encodable::write_encoded_to(&self.content_encoding, writer));
        try!(::Encodable::write_encoded_to(&self.headers, writer));
        try!(::Encodable::write_encoded_to(&self.priority, writer));
        try!(::Encodable::write_encoded_to(&self.timestamp, writer));

        ::std::result::Result::Ok(())
    } // fn write_encoded_to()
} // impl Encodable
impl<'a> ::method::stream::CancelMethod<'a> for ::Amqp9_0 {
    type Payload = Cancel<'a>;
} // impl<'a> ::method::stream::CancelMethod<'a> for ::Amqp9_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Cancel<'a> {
    consumer_tag: ::std::borrow::Cow<'a, str>,
    no_wait: bool,
} // struct Cancel<'a>

impl<'a> Cancel<'a> {
    pub fn new<C>(consumer_tag: C, no_wait: bool) -> Self
        where C: Into<::std::borrow::Cow<'a, str>>
    {
        Cancel {
            consumer_tag: consumer_tag.into(),
            no_wait: no_wait,
        } // Cancel
    } // fn new()
    impl_properties! {
(consumer_tag, consumer_tag_mut, set_consumer_tag) -> Cow<str>,
(no_wait, set_no_wait) -> bool,
} // impl_properties
} // impl<'a> Cancel<'a>
impl<'a> Default for Cancel<'a> {
    fn default() -> Self {
        Cancel::new("", false)
    } // fn default()
} // impl Default for Cancel

impl<'a> ::Encodable for Cancel<'a> {
    fn encoded_size(&self) -> usize {
        1 + ::Encodable::encoded_size(&self.consumer_tag)
    } // encoded_size
    fn write_encoded_to<W>(&self, writer: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        try!(::Encodable::write_encoded_to(&self.consumer_tag, writer)); // consumer_tag
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
fn test_cancel_encodable_bytes_written_matches_len() {
    let payload: Cancel = Default::default();
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



impl<'a> ::ProtocolMethodPayload for Cancel<'a> {
    fn class_id(&self) -> u16 {
        80
    } // fn class_id()
    fn method_id(&self) -> u16 {
        30
    } // fn method_id()
} // impl ::Payload for Cancel
impl<'a> ::method::stream::SetCancelMethodFields<'a> for Cancel<'a> {
    fn set_consumer_tag<V>(&mut self, consumer_tag: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_consumer_tag(consumer_tag.into())
    } // set_consumer_tag()
    fn set_no_wait(&mut self, no_wait: bool) {
        self.set_no_wait(no_wait)
    } // set_no_wait()
} // impl<'a> ::method::stream::SetCancelMethodFields<'a> for Cancel<'a>
impl<'a> ::method::stream::CancelOkMethod<'a> for ::Amqp9_0 {
    type Payload = CancelOk<'a>;
} // impl<'a> ::method::stream::CancelOkMethod<'a> for ::Amqp9_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct CancelOk<'a> {
    consumer_tag: ::std::borrow::Cow<'a, str>,
} // struct CancelOk<'a>

impl<'a> CancelOk<'a> {
    pub fn new<C>(consumer_tag: C) -> Self
        where C: Into<::std::borrow::Cow<'a, str>>
    {
        CancelOk { consumer_tag: consumer_tag.into() } // CancelOk
    } // fn new()
    impl_properties! {
(consumer_tag, consumer_tag_mut, set_consumer_tag) -> Cow<str>,
} // impl_properties
} // impl<'a> CancelOk<'a>
impl<'a> Default for CancelOk<'a> {
    fn default() -> Self {
        CancelOk::new("")
    } // fn default()
} // impl Default for CancelOk

impl<'a> ::Encodable for CancelOk<'a> {
    fn encoded_size(&self) -> usize {
        0 + ::Encodable::encoded_size(&self.consumer_tag)
    } // encoded_size
    fn write_encoded_to<W>(&self, writer: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        try!(::Encodable::write_encoded_to(&self.consumer_tag, writer)); // consumer_tag

        ::std::result::Result::Ok(())
    } // fn write_encoded_to()
} // impl Encodable

#[test]
fn test_cancel_ok_encodable_bytes_written_matches_len() {
    let payload: CancelOk = Default::default();
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



impl<'a> ::ProtocolMethodPayload for CancelOk<'a> {
    fn class_id(&self) -> u16 {
        80
    } // fn class_id()
    fn method_id(&self) -> u16 {
        31
    } // fn method_id()
} // impl ::Payload for CancelOk
impl<'a> ::method::stream::SetCancelOkMethodFields<'a> for CancelOk<'a> {
    fn set_consumer_tag<V>(&mut self, consumer_tag: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_consumer_tag(consumer_tag.into())
    } // set_consumer_tag()
} // impl<'a> ::method::stream::SetCancelOkMethodFields<'a> for CancelOk<'a>
impl<'a> ::method::stream::ConsumeMethod<'a> for ::Amqp9_0 {
    type Payload = Consume<'a>;
} // impl<'a> ::method::stream::ConsumeMethod<'a> for ::Amqp9_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Consume<'a> {
    ticket: u16,
    queue: ::std::borrow::Cow<'a, str>,
    consumer_tag: ::std::borrow::Cow<'a, str>,
    no_local: bool,
    exclusive: bool,
    no_wait: bool,
    filter: ::field::TableEntries<'a>,
} // struct Consume<'a>

impl<'a> Consume<'a> {
    pub fn new<Q, C, F>(ticket: u16,
                        queue: Q,
                        consumer_tag: C,
                        no_local: bool,
                        exclusive: bool,
                        no_wait: bool,
                        filter: F)
                        -> Self
        where Q: Into<::std::borrow::Cow<'a, str>>,
              C: Into<::std::borrow::Cow<'a, str>>,
              F: Into<::field::TableEntries<'a>>
    {
        Consume {
            ticket: ticket,
            queue: queue.into(),
            consumer_tag: consumer_tag.into(),
            no_local: no_local,
            exclusive: exclusive,
            no_wait: no_wait,
            filter: filter.into(),
        } // Consume
    } // fn new()
    impl_properties! {
(ticket, set_ticket) -> u16,
(queue, queue_mut, set_queue) -> Cow<str>,
(consumer_tag, consumer_tag_mut, set_consumer_tag) -> Cow<str>,
(no_local, set_no_local) -> bool,
(exclusive, set_exclusive) -> bool,
(no_wait, set_no_wait) -> bool,
(filter, filter_mut, set_filter) -> &::field::TableEntries<'a>,
} // impl_properties
} // impl<'a> Consume<'a>
impl<'a> Default for Consume<'a> {
    fn default() -> Self {
        Consume::new(0, "", "", false, false, false, ::field::TableEntries::new())
    } // fn default()
} // impl Default for Consume

impl<'a> ::Encodable for Consume<'a> {
    fn encoded_size(&self) -> usize {
        3 + ::Encodable::encoded_size(&self.queue) + ::Encodable::encoded_size(&self.consumer_tag) +
        ::Encodable::encoded_size(&self.filter)
    } // encoded_size
    fn write_encoded_to<W>(&self, writer: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        try!(::Encodable::write_encoded_to(&self.ticket, writer)); // ticket
        try!(::Encodable::write_encoded_to(&self.queue, writer)); // queue
        try!(::Encodable::write_encoded_to(&self.consumer_tag, writer)); // consumer_tag
        try!(::Encodable::write_encoded_to(&{
                                               let mut bits = ::bit_vec::BitVec::from_elem(8,
                                                                                           false);
                                               bits.set(7, self.no_local);
                                               bits.set(6, self.exclusive);
                                               bits.set(5, self.no_wait);
                                               bits
                                           },
                                           writer));
        try!(::Encodable::write_encoded_to(&self.filter, writer)); // filter

        ::std::result::Result::Ok(())
    } // fn write_encoded_to()
} // impl Encodable

#[test]
fn test_consume_encodable_bytes_written_matches_len() {
    let payload: Consume = Default::default();
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



impl<'a> ::ProtocolMethodPayload for Consume<'a> {
    fn class_id(&self) -> u16 {
        80
    } // fn class_id()
    fn method_id(&self) -> u16 {
        20
    } // fn method_id()
} // impl ::Payload for Consume
impl<'a> ::method::stream::SetConsumeMethodFields<'a> for Consume<'a> {
    fn set_ticket(&mut self, ticket: u16) {
        self.set_ticket(ticket)
    } // set_ticket()
    fn set_queue<V>(&mut self, queue: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_queue(queue.into())
    } // set_queue()
    fn set_consumer_tag<V>(&mut self, consumer_tag: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_consumer_tag(consumer_tag.into())
    } // set_consumer_tag()
    fn set_no_local(&mut self, no_local: bool) {
        self.set_no_local(no_local)
    } // set_no_local()
    fn set_exclusive(&mut self, exclusive: bool) {
        self.set_exclusive(exclusive)
    } // set_exclusive()
    fn set_no_wait(&mut self, no_wait: bool) {
        self.set_no_wait(no_wait)
    } // set_no_wait()
    fn set_filter<V>(&mut self, filter: V)
        where V: Into<::field::TableEntries<'a>>
    {
        self.set_filter(filter.into())
    } // set_filter()
} // impl<'a> ::method::stream::SetConsumeMethodFields<'a> for Consume<'a>
impl<'a> ::method::stream::ConsumeOkMethod<'a> for ::Amqp9_0 {
    type Payload = ConsumeOk<'a>;
} // impl<'a> ::method::stream::ConsumeOkMethod<'a> for ::Amqp9_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct ConsumeOk<'a> {
    consumer_tag: ::std::borrow::Cow<'a, str>,
} // struct ConsumeOk<'a>

impl<'a> ConsumeOk<'a> {
    pub fn new<C>(consumer_tag: C) -> Self
        where C: Into<::std::borrow::Cow<'a, str>>
    {
        ConsumeOk { consumer_tag: consumer_tag.into() } // ConsumeOk
    } // fn new()
    impl_properties! {
(consumer_tag, consumer_tag_mut, set_consumer_tag) -> Cow<str>,
} // impl_properties
} // impl<'a> ConsumeOk<'a>
impl<'a> Default for ConsumeOk<'a> {
    fn default() -> Self {
        ConsumeOk::new("")
    } // fn default()
} // impl Default for ConsumeOk

impl<'a> ::Encodable for ConsumeOk<'a> {
    fn encoded_size(&self) -> usize {
        0 + ::Encodable::encoded_size(&self.consumer_tag)
    } // encoded_size
    fn write_encoded_to<W>(&self, writer: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        try!(::Encodable::write_encoded_to(&self.consumer_tag, writer)); // consumer_tag

        ::std::result::Result::Ok(())
    } // fn write_encoded_to()
} // impl Encodable

#[test]
fn test_consume_ok_encodable_bytes_written_matches_len() {
    let payload: ConsumeOk = Default::default();
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



impl<'a> ::ProtocolMethodPayload for ConsumeOk<'a> {
    fn class_id(&self) -> u16 {
        80
    } // fn class_id()
    fn method_id(&self) -> u16 {
        21
    } // fn method_id()
} // impl ::Payload for ConsumeOk
impl<'a> ::method::stream::SetConsumeOkMethodFields<'a> for ConsumeOk<'a> {
    fn set_consumer_tag<V>(&mut self, consumer_tag: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_consumer_tag(consumer_tag.into())
    } // set_consumer_tag()
} // impl<'a> ::method::stream::SetConsumeOkMethodFields<'a> for ConsumeOk<'a>
impl<'a> ::method::stream::DeliverMethod<'a> for ::Amqp9_0 {
    type Payload = Deliver<'a>;
} // impl<'a> ::method::stream::DeliverMethod<'a> for ::Amqp9_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Deliver<'a> {
    consumer_tag: ::std::borrow::Cow<'a, str>,
    delivery_tag: u64,
    exchange: ::std::borrow::Cow<'a, str>,
    queue: ::std::borrow::Cow<'a, str>,
} // struct Deliver<'a>

impl<'a> Deliver<'a> {
    pub fn new<C, E, Q>(consumer_tag: C, delivery_tag: u64, exchange: E, queue: Q) -> Self
        where C: Into<::std::borrow::Cow<'a, str>>,
              E: Into<::std::borrow::Cow<'a, str>>,
              Q: Into<::std::borrow::Cow<'a, str>>
    {
        Deliver {
            consumer_tag: consumer_tag.into(),
            delivery_tag: delivery_tag,
            exchange: exchange.into(),
            queue: queue.into(),
        } // Deliver
    } // fn new()
    impl_properties! {
(consumer_tag, consumer_tag_mut, set_consumer_tag) -> Cow<str>,
(delivery_tag, set_delivery_tag) -> u64,
(exchange, exchange_mut, set_exchange) -> Cow<str>,
(queue, queue_mut, set_queue) -> Cow<str>,
} // impl_properties
} // impl<'a> Deliver<'a>
impl<'a> Default for Deliver<'a> {
    fn default() -> Self {
        Deliver::new("", 0, "", "")
    } // fn default()
} // impl Default for Deliver

impl<'a> ::Encodable for Deliver<'a> {
    fn encoded_size(&self) -> usize {
        8 + ::Encodable::encoded_size(&self.consumer_tag) +
        ::Encodable::encoded_size(&self.exchange) + ::Encodable::encoded_size(&self.queue)
    } // encoded_size
    fn write_encoded_to<W>(&self, writer: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        try!(::Encodable::write_encoded_to(&self.consumer_tag, writer)); // consumer_tag
        try!(::Encodable::write_encoded_to(&self.delivery_tag, writer)); // delivery_tag
        try!(::Encodable::write_encoded_to(&self.exchange, writer)); // exchange
        try!(::Encodable::write_encoded_to(&self.queue, writer)); // queue

        ::std::result::Result::Ok(())
    } // fn write_encoded_to()
} // impl Encodable

#[test]
fn test_deliver_encodable_bytes_written_matches_len() {
    let payload: Deliver = Default::default();
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



impl<'a> ::ProtocolMethodPayload for Deliver<'a> {
    fn class_id(&self) -> u16 {
        80
    } // fn class_id()
    fn method_id(&self) -> u16 {
        60
    } // fn method_id()
} // impl ::Payload for Deliver
impl<'a> ::method::stream::SetDeliverMethodFields<'a> for Deliver<'a> {
    fn set_consumer_tag<V>(&mut self, consumer_tag: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_consumer_tag(consumer_tag.into())
    } // set_consumer_tag()
    fn set_delivery_tag(&mut self, delivery_tag: u64) {
        self.set_delivery_tag(delivery_tag)
    } // set_delivery_tag()
    fn set_exchange<V>(&mut self, exchange: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_exchange(exchange.into())
    } // set_exchange()
    fn set_queue<V>(&mut self, queue: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_queue(queue.into())
    } // set_queue()
} // impl<'a> ::method::stream::SetDeliverMethodFields<'a> for Deliver<'a>
impl<'a> ::method::stream::PublishMethod<'a> for ::Amqp9_0 {
    type Payload = Publish<'a>;
} // impl<'a> ::method::stream::PublishMethod<'a> for ::Amqp9_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Publish<'a> {
    ticket: u16,
    exchange: ::std::borrow::Cow<'a, str>,
    routing_key: ::std::borrow::Cow<'a, str>,
    mandatory: bool,
    immediate: bool,
} // struct Publish<'a>

impl<'a> Publish<'a> {
    pub fn new<E, R>(ticket: u16,
                     exchange: E,
                     routing_key: R,
                     mandatory: bool,
                     immediate: bool)
                     -> Self
        where E: Into<::std::borrow::Cow<'a, str>>,
              R: Into<::std::borrow::Cow<'a, str>>
    {
        Publish {
            ticket: ticket,
            exchange: exchange.into(),
            routing_key: routing_key.into(),
            mandatory: mandatory,
            immediate: immediate,
        } // Publish
    } // fn new()
    impl_properties! {
(ticket, set_ticket) -> u16,
(exchange, exchange_mut, set_exchange) -> Cow<str>,
(routing_key, routing_key_mut, set_routing_key) -> Cow<str>,
(mandatory, set_mandatory) -> bool,
(immediate, set_immediate) -> bool,
} // impl_properties
} // impl<'a> Publish<'a>
impl<'a> Default for Publish<'a> {
    fn default() -> Self {
        Publish::new(0, "", "", false, false)
    } // fn default()
} // impl Default for Publish

impl<'a> ::Encodable for Publish<'a> {
    fn encoded_size(&self) -> usize {
        3 + ::Encodable::encoded_size(&self.exchange) + ::Encodable::encoded_size(&self.routing_key)
    } // encoded_size
    fn write_encoded_to<W>(&self, writer: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        try!(::Encodable::write_encoded_to(&self.ticket, writer)); // ticket
        try!(::Encodable::write_encoded_to(&self.exchange, writer)); // exchange
        try!(::Encodable::write_encoded_to(&self.routing_key, writer)); // routing_key
        try!(::Encodable::write_encoded_to(&{
                                               let mut bits = ::bit_vec::BitVec::from_elem(8,
                                                                                           false);
                                               bits.set(7, self.mandatory);
                                               bits.set(6, self.immediate);
                                               bits
                                           },
                                           writer));

        ::std::result::Result::Ok(())
    } // fn write_encoded_to()
} // impl Encodable

#[test]
fn test_publish_encodable_bytes_written_matches_len() {
    let payload: Publish = Default::default();
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



impl<'a> ::ProtocolMethodPayload for Publish<'a> {
    fn class_id(&self) -> u16 {
        80
    } // fn class_id()
    fn method_id(&self) -> u16 {
        40
    } // fn method_id()
} // impl ::Payload for Publish
impl<'a> ::method::stream::SetPublishMethodFields<'a> for Publish<'a> {
    fn set_ticket(&mut self, ticket: u16) {
        self.set_ticket(ticket)
    } // set_ticket()
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
    fn set_mandatory(&mut self, mandatory: bool) {
        self.set_mandatory(mandatory)
    } // set_mandatory()
    fn set_immediate(&mut self, immediate: bool) {
        self.set_immediate(immediate)
    } // set_immediate()
} // impl<'a> ::method::stream::SetPublishMethodFields<'a> for Publish<'a>
impl ::method::stream::QosMethod for ::Amqp9_0 {
    type Payload = Qos;
} // impl ::method::stream::QosMethod for ::Amqp9_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Qos {
    prefetch_size: u32,
    prefetch_count: u16,
    consume_rate: u32,
    global: bool,
} // struct Qos

impl Qos {
    pub fn new(prefetch_size: u32, prefetch_count: u16, consume_rate: u32, global: bool) -> Self {
        Qos {
            prefetch_size: prefetch_size,
            prefetch_count: prefetch_count,
            consume_rate: consume_rate,
            global: global,
        } // Qos
    } // fn new()
    impl_properties! {
(prefetch_size, set_prefetch_size) -> u32,
(prefetch_count, set_prefetch_count) -> u16,
(consume_rate, set_consume_rate) -> u32,
(global, set_global) -> bool,
} // impl_properties
} // impl Qos
impl Default for Qos {
    fn default() -> Self {
        Qos::new(0, 0, 0, false)
    } // fn default()
} // impl Default for Qos

impl ::Encodable for Qos {
    fn encoded_size(&self) -> usize {
        11
    } // encoded_size
    fn write_encoded_to<W>(&self, writer: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        try!(::Encodable::write_encoded_to(&self.prefetch_size, writer)); // prefetch_size
        try!(::Encodable::write_encoded_to(&self.prefetch_count, writer)); // prefetch_count
        try!(::Encodable::write_encoded_to(&self.consume_rate, writer)); // consume_rate
        try!(::Encodable::write_encoded_to(&{
                                               let mut bits = ::bit_vec::BitVec::from_elem(8,
                                                                                           false);
                                               bits.set(7, self.global);
                                               bits
                                           },
                                           writer));

        ::std::result::Result::Ok(())
    } // fn write_encoded_to()
} // impl Encodable

#[test]
fn test_qos_encodable_bytes_written_matches_len() {
    let payload: Qos = Default::default();
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



impl ::ProtocolMethodPayload for Qos {
    fn class_id(&self) -> u16 {
        80
    } // fn class_id()
    fn method_id(&self) -> u16 {
        10
    } // fn method_id()
} // impl ::Payload for Qos
impl ::method::stream::SetQosMethodFields for Qos {
    fn set_prefetch_size(&mut self, prefetch_size: u32) {
        self.set_prefetch_size(prefetch_size)
    } // set_prefetch_size()
    fn set_prefetch_count(&mut self, prefetch_count: u16) {
        self.set_prefetch_count(prefetch_count)
    } // set_prefetch_count()
    fn set_consume_rate(&mut self, consume_rate: u32) {
        self.set_consume_rate(consume_rate)
    } // set_consume_rate()
    fn set_global(&mut self, global: bool) {
        self.set_global(global)
    } // set_global()
} // impl ::method::stream::SetQosMethodFields for Qos
impl ::method::stream::QosOkMethod for ::Amqp9_0 {
    type Payload = QosOk;
} // impl ::method::stream::QosOkMethod for ::Amqp9_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct QosOk;

impl QosOk {
    pub fn new() -> Self {
        QosOk
    } // fn new()
} // impl QosOk
impl Default for QosOk {
    fn default() -> Self {
        QosOk::new()
    } // fn default()
} // impl Default for QosOk

impl ::Encodable for QosOk {
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
fn test_qos_ok_encodable_bytes_written_matches_len() {
    let payload: QosOk = Default::default();
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



impl ::ProtocolMethodPayload for QosOk {
    fn class_id(&self) -> u16 {
        80
    } // fn class_id()
    fn method_id(&self) -> u16 {
        11
    } // fn method_id()
} // impl ::Payload for QosOk
impl<'a> ::method::stream::ReturnMethod<'a> for ::Amqp9_0 {
    type Payload = Return<'a>;
} // impl<'a> ::method::stream::ReturnMethod<'a> for ::Amqp9_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Return<'a> {
    reply_code: u16,
    reply_text: ::std::borrow::Cow<'a, str>,
    exchange: ::std::borrow::Cow<'a, str>,
    routing_key: ::std::borrow::Cow<'a, str>,
} // struct Return<'a>

impl<'a> Return<'a> {
    pub fn new<R, E, R0>(reply_code: u16, reply_text: R, exchange: E, routing_key: R0) -> Self
        where R: Into<::std::borrow::Cow<'a, str>>,
              E: Into<::std::borrow::Cow<'a, str>>,
              R0: Into<::std::borrow::Cow<'a, str>>
    {
        Return {
            reply_code: reply_code,
            reply_text: reply_text.into(),
            exchange: exchange.into(),
            routing_key: routing_key.into(),
        } // Return
    } // fn new()
    impl_properties! {
(reply_code, set_reply_code) -> u16,
(reply_text, reply_text_mut, set_reply_text) -> Cow<str>,
(exchange, exchange_mut, set_exchange) -> Cow<str>,
(routing_key, routing_key_mut, set_routing_key) -> Cow<str>,
} // impl_properties
} // impl<'a> Return<'a>
impl<'a> Default for Return<'a> {
    fn default() -> Self {
        Return::new(0, "", "", "")
    } // fn default()
} // impl Default for Return

impl<'a> ::Encodable for Return<'a> {
    fn encoded_size(&self) -> usize {
        2 + ::Encodable::encoded_size(&self.reply_text) +
        ::Encodable::encoded_size(&self.exchange) +
        ::Encodable::encoded_size(&self.routing_key)
    } // encoded_size
    fn write_encoded_to<W>(&self, writer: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        try!(::Encodable::write_encoded_to(&self.reply_code, writer)); // reply_code
        try!(::Encodable::write_encoded_to(&self.reply_text, writer)); // reply_text
        try!(::Encodable::write_encoded_to(&self.exchange, writer)); // exchange
        try!(::Encodable::write_encoded_to(&self.routing_key, writer)); // routing_key

        ::std::result::Result::Ok(())
    } // fn write_encoded_to()
} // impl Encodable

#[test]
fn test_return_encodable_bytes_written_matches_len() {
    let payload: Return = Default::default();
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



impl<'a> ::ProtocolMethodPayload for Return<'a> {
    fn class_id(&self) -> u16 {
        80
    } // fn class_id()
    fn method_id(&self) -> u16 {
        50
    } // fn method_id()
} // impl ::Payload for Return
impl<'a> ::method::stream::SetReturnMethodFields<'a> for Return<'a> {
    fn set_reply_code(&mut self, reply_code: u16) {
        self.set_reply_code(reply_code)
    } // set_reply_code()
    fn set_reply_text<V>(&mut self, reply_text: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_reply_text(reply_text.into())
    } // set_reply_text()
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
} // impl<'a> ::method::stream::SetReturnMethodFields<'a> for Return<'a>

#[derive(Debug)]
pub enum ClassMethod<'a> {
    Cancel(Cancel<'a>),
    CancelOk(CancelOk<'a>),
    Consume(Consume<'a>),
    ConsumeOk(ConsumeOk<'a>),
    Deliver(Deliver<'a>),
    Publish(Publish<'a>),
    Qos(Qos),
    QosOk(QosOk),
    Return(Return<'a>),
} // enum ClassMethod


impl<'a> ::Encodable for ClassMethod<'a> {
    fn encoded_size(&self) -> usize {
        match *self {
            ClassMethod::Cancel(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::CancelOk(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::Consume(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::ConsumeOk(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::Deliver(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::Publish(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::Qos(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::QosOk(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::Return(ref method) => ::Encodable::encoded_size(method),

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
            ClassMethod::Cancel(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::CancelOk(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::Consume(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::ConsumeOk(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::Deliver(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::Publish(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::Qos(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::QosOk(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::Return(ref method) => ::ProtocolMethodPayload::class_id(method),

        } // match *self

    } // fn class_id

    fn method_id(&self) -> u16 {
        match *self {
            ClassMethod::Cancel(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::CancelOk(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::Consume(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::ConsumeOk(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::Deliver(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::Publish(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::Qos(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::QosOk(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::Return(ref method) => ::ProtocolMethodPayload::method_id(method),

        } // match *self

    } // fn method_id
} // impl ProtocolMethodPayload for ClassMethod
