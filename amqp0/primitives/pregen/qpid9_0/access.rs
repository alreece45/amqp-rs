// Generated by build.rs script in amqp0-primitives
// Pre-generated files are used by default. Generation is done with the amqp0-codegen crate
//
// To regenerate, and not use pre-generated files, use: cargo --features="amqp0-build-primitives"
// To format and replace the pre-generated files, use: cargo --features="amqp0-pregen-primitives"
//
// EDITORS BEWARE: Your modifications may be overridden

// generated by primalgen::codegen::spec_module::class_mod::ClassModuleWriter
#![allow(too_many_arguments)]

impl<'a> ::method::access::RequestMethod<'a> for ::Qpid9_0 {
    type Payload = Request<'a>;
} // impl<'a> ::method::access::RequestMethod<'a> for ::Qpid9_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Request<'a> {
    realm: ::std::borrow::Cow<'a, str>,
    exclusive: bool,
    passive: bool,
    active: bool,
    write: bool,
    read: bool,
} // struct Request<'a>

impl<'a> Request<'a> {
    pub fn new<R>(realm: R,
                  exclusive: bool,
                  passive: bool,
                  active: bool,
                  write: bool,
                  read: bool)
                  -> Self
        where R: Into<::std::borrow::Cow<'a, str>>
    {
        Request {
            realm: realm.into(),
            exclusive: exclusive,
            passive: passive,
            active: active,
            write: write,
            read: read,
        } // Request
    } // fn new()
    impl_properties! {
(realm, realm_mut, set_realm) -> Cow<str>,
(exclusive, set_exclusive) -> bool,
(passive, set_passive) -> bool,
(active, set_active) -> bool,
(write, set_write) -> bool,
(read, set_read) -> bool,
} // impl_properties
} // impl<'a> Request<'a>
impl<'a> Default for Request<'a> {
    fn default() -> Self {
        Request::new("", false, false, false, false, false)
    } // fn default()
} // impl Default for Request

impl<'a> ::Encodable for Request<'a> {
    fn encoded_size(&self) -> usize {
        1 + ::Encodable::encoded_size(&self.realm)
    } // encoded_size
    fn write_encoded_to<W>(&self, writer: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        try!(::Encodable::write_encoded_to(&self.realm, writer)); // realm
        try!(::Encodable::write_encoded_to(&{
                                               let mut bits = ::bit_vec::BitVec::from_elem(8,
                                                                                           false);
                                               bits.set(7, self.exclusive);
                                               bits.set(6, self.passive);
                                               bits.set(5, self.active);
                                               bits.set(4, self.write);
                                               bits.set(3, self.read);
                                               bits
                                           },
                                           writer));

        ::std::result::Result::Ok(())
    } // fn write_encoded_to()
} // impl Encodable

#[test]
fn test_request_encodable_bytes_written_matches_len() {
    let payload: Request = Default::default();
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



impl<'a> ::ProtocolMethodPayload for Request<'a> {
    fn class_id(&self) -> u16 {
        30
    } // fn class_id()
    fn method_id(&self) -> u16 {
        10
    } // fn method_id()
} // impl ::Payload for Request
impl<'a> ::method::access::SetRequestMethodFields<'a> for Request<'a> {
    fn set_realm<V>(&mut self, realm: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_realm(realm.into())
    } // set_realm()
    fn set_exclusive(&mut self, exclusive: bool) {
        self.set_exclusive(exclusive)
    } // set_exclusive()
    fn set_passive(&mut self, passive: bool) {
        self.set_passive(passive)
    } // set_passive()
    fn set_active(&mut self, active: bool) {
        self.set_active(active)
    } // set_active()
    fn set_write(&mut self, write: bool) {
        self.set_write(write)
    } // set_write()
    fn set_read(&mut self, read: bool) {
        self.set_read(read)
    } // set_read()
} // impl<'a> ::method::access::SetRequestMethodFields<'a> for Request<'a>
impl ::method::access::RequestOkMethod for ::Qpid9_0 {
    type Payload = RequestOk;
} // impl ::method::access::RequestOkMethod for ::Qpid9_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct RequestOk {
    ticket: u16,
} // struct RequestOk

impl RequestOk {
    pub fn new(ticket: u16) -> Self {
        RequestOk { ticket: ticket } // RequestOk
    } // fn new()
    impl_properties! {
(ticket, set_ticket) -> u16,
} // impl_properties
} // impl RequestOk
impl Default for RequestOk {
    fn default() -> Self {
        RequestOk::new(0)
    } // fn default()
} // impl Default for RequestOk

impl ::Encodable for RequestOk {
    fn encoded_size(&self) -> usize {
        2
    } // encoded_size
    fn write_encoded_to<W>(&self, writer: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        try!(::Encodable::write_encoded_to(&self.ticket, writer)); // ticket

        ::std::result::Result::Ok(())
    } // fn write_encoded_to()
} // impl Encodable

#[test]
fn test_request_ok_encodable_bytes_written_matches_len() {
    let payload: RequestOk = Default::default();
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



impl ::ProtocolMethodPayload for RequestOk {
    fn class_id(&self) -> u16 {
        30
    } // fn class_id()
    fn method_id(&self) -> u16 {
        11
    } // fn method_id()
} // impl ::Payload for RequestOk
impl ::method::access::SetRequestOkMethodFields for RequestOk {
    fn set_ticket(&mut self, ticket: u16) {
        self.set_ticket(ticket)
    } // set_ticket()
} // impl ::method::access::SetRequestOkMethodFields for RequestOk

#[derive(Debug)]
pub enum ClassMethod<'a> {
    Request(Request<'a>),
    RequestOk(RequestOk),
} // enum ClassMethod


impl<'a> ::Encodable for ClassMethod<'a> {
    fn encoded_size(&self) -> usize {
        match *self {
            ClassMethod::Request(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::RequestOk(ref method) => ::Encodable::encoded_size(method),

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
            ClassMethod::Request(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::RequestOk(ref method) => ::ProtocolMethodPayload::class_id(method),

        } // match *self

    } // fn class_id

    fn method_id(&self) -> u16 {
        match *self {
            ClassMethod::Request(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::RequestOk(ref method) => ::ProtocolMethodPayload::method_id(method),

        } // match *self

    } // fn method_id
} // impl ProtocolMethodPayload for ClassMethod
