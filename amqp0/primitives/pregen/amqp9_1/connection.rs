// Generated by build.rs script in amqp0-primitives
// Pre-generated files are used by default. Generation is done with the amqp0-codegen crate
//
// To regenerate, and not use pre-generated files, use: cargo --features="amqp0-build-primitives"
// To format and replace the pre-generated files, use: cargo --features="amqp0-pregen-primitives"
//
// EDITORS BEWARE: Your modifications may be overridden

// generated by primalgen::codegen::spec_module::class_mod::ClassModuleWriter
#![allow(too_many_arguments)]

impl<'a> ::method::connection::CloseMethod<'a> for ::Amqp9_1 {
    type Payload = Close<'a>;
} // impl<'a> ::method::connection::CloseMethod<'a> for ::Amqp9_1

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
    fn class_id(&self) -> u16 {
        10
    } // fn class_id()
    fn method_id(&self) -> u16 {
        50
    } // fn method_id()
} // impl ::Payload for Close
impl<'a> ::method::connection::SetCloseMethodFields<'a> for Close<'a> {
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
} // impl<'a> ::method::connection::SetCloseMethodFields<'a> for Close<'a>
impl ::method::connection::CloseOkMethod for ::Amqp9_1 {
    type Payload = CloseOk;
} // impl ::method::connection::CloseOkMethod for ::Amqp9_1

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
    fn class_id(&self) -> u16 {
        10
    } // fn class_id()
    fn method_id(&self) -> u16 {
        51
    } // fn method_id()
} // impl ::Payload for CloseOk
impl<'a> ::method::connection::OpenMethod<'a> for ::Amqp9_1 {
    type Payload = Open<'a>;
} // impl<'a> ::method::connection::OpenMethod<'a> for ::Amqp9_1

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Open<'a> {
    virtual_host: ::std::borrow::Cow<'a, str>,
} // struct Open<'a>

impl<'a> Open<'a> {
    pub fn new<V>(virtual_host: V) -> Self
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        Open { virtual_host: virtual_host.into() } // Open
    } // fn new()
    impl_properties! {
(virtual_host, virtual_host_mut, set_virtual_host) -> Cow<str>,
} // impl_properties
} // impl<'a> Open<'a>
impl<'a> Default for Open<'a> {
    fn default() -> Self {
        Open::new("")
    } // fn default()
} // impl Default for Open

impl<'a> ::Encodable for Open<'a> {
    fn encoded_size(&self) -> usize {
        2 + ::Encodable::encoded_size(&self.virtual_host)
    } // encoded_size
    fn write_encoded_to<W>(&self, writer: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        try!(::Encodable::write_encoded_to(&self.virtual_host, writer)); // virtual_host
        try!(::Encodable::write_encoded_to(&0u8, writer)); // reserved: reserved_1
        try!(::Encodable::write_encoded_to(&0u8, writer));

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



impl<'a> ::ProtocolMethodPayload for Open<'a> {
    fn class_id(&self) -> u16 {
        10
    } // fn class_id()
    fn method_id(&self) -> u16 {
        40
    } // fn method_id()
} // impl ::Payload for Open
impl<'a> ::method::connection::SetOpenMethodFields<'a> for Open<'a> {
    fn set_virtual_host<V>(&mut self, virtual_host: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_virtual_host(virtual_host.into())
    } // set_virtual_host()
} // impl<'a> ::method::connection::SetOpenMethodFields<'a> for Open<'a>
impl<'a> ::method::connection::OpenOkMethod<'a> for ::Amqp9_1 {
    type Payload = OpenOk;
} // impl<'a> ::method::connection::OpenOkMethod<'a> for ::Amqp9_1

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
    fn class_id(&self) -> u16 {
        10
    } // fn class_id()
    fn method_id(&self) -> u16 {
        41
    } // fn method_id()
} // impl ::Payload for OpenOk
impl<'a> ::method::connection::SetOpenOkMethodFields<'a> for OpenOk {} // impl<'a> ::method::connection::SetOpenOkMethodFields<'a> for OpenOk
impl<'a> ::method::connection::SecureMethod<'a> for ::Amqp9_1 {
    type Payload = Secure<'a>;
} // impl<'a> ::method::connection::SecureMethod<'a> for ::Amqp9_1

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Secure<'a> {
    challenge: ::std::borrow::Cow<'a, [u8]>,
} // struct Secure<'a>

impl<'a> Secure<'a> {
    pub fn new<C>(challenge: C) -> Self
        where C: Into<::std::borrow::Cow<'a, [u8]>>
    {
        Secure { challenge: challenge.into() } // Secure
    } // fn new()
    impl_properties! {
(challenge, challenge_mut, set_challenge) -> Cow<[u8]>,
} // impl_properties
} // impl<'a> Secure<'a>
impl<'a> Default for Secure<'a> {
    fn default() -> Self {
        Secure::new(&[][..])
    } // fn default()
} // impl Default for Secure

impl<'a> ::Encodable for Secure<'a> {
    fn encoded_size(&self) -> usize {
        0 + ::Encodable::encoded_size(&self.challenge)
    } // encoded_size
    fn write_encoded_to<W>(&self, writer: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        try!(::Encodable::write_encoded_to(&self.challenge, writer)); // challenge

        ::std::result::Result::Ok(())
    } // fn write_encoded_to()
} // impl Encodable

#[test]
fn test_secure_encodable_bytes_written_matches_len() {
    let payload: Secure = Default::default();
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



impl<'a> ::ProtocolMethodPayload for Secure<'a> {
    fn class_id(&self) -> u16 {
        10
    } // fn class_id()
    fn method_id(&self) -> u16 {
        20
    } // fn method_id()
} // impl ::Payload for Secure
impl<'a> ::method::connection::SetSecureMethodFields<'a> for Secure<'a> {
    fn set_challenge<V>(&mut self, challenge: V)
        where V: Into<::std::borrow::Cow<'a, [u8]>>
    {
        self.set_challenge(challenge.into())
    } // set_challenge()
} // impl<'a> ::method::connection::SetSecureMethodFields<'a> for Secure<'a>
impl<'a> ::method::connection::SecureOkMethod<'a> for ::Amqp9_1 {
    type Payload = SecureOk<'a>;
} // impl<'a> ::method::connection::SecureOkMethod<'a> for ::Amqp9_1

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct SecureOk<'a> {
    response: ::std::borrow::Cow<'a, [u8]>,
} // struct SecureOk<'a>

impl<'a> SecureOk<'a> {
    pub fn new<R>(response: R) -> Self
        where R: Into<::std::borrow::Cow<'a, [u8]>>
    {
        SecureOk { response: response.into() } // SecureOk
    } // fn new()
    impl_properties! {
(response, response_mut, set_response) -> Cow<[u8]>,
} // impl_properties
} // impl<'a> SecureOk<'a>
impl<'a> Default for SecureOk<'a> {
    fn default() -> Self {
        SecureOk::new(&[][..])
    } // fn default()
} // impl Default for SecureOk

impl<'a> ::Encodable for SecureOk<'a> {
    fn encoded_size(&self) -> usize {
        0 + ::Encodable::encoded_size(&self.response)
    } // encoded_size
    fn write_encoded_to<W>(&self, writer: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        try!(::Encodable::write_encoded_to(&self.response, writer)); // response

        ::std::result::Result::Ok(())
    } // fn write_encoded_to()
} // impl Encodable

#[test]
fn test_secure_ok_encodable_bytes_written_matches_len() {
    let payload: SecureOk = Default::default();
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



impl<'a> ::ProtocolMethodPayload for SecureOk<'a> {
    fn class_id(&self) -> u16 {
        10
    } // fn class_id()
    fn method_id(&self) -> u16 {
        21
    } // fn method_id()
} // impl ::Payload for SecureOk
impl<'a> ::method::connection::SetSecureOkMethodFields<'a> for SecureOk<'a> {
    fn set_response<V>(&mut self, response: V)
        where V: Into<::std::borrow::Cow<'a, [u8]>>
    {
        self.set_response(response.into())
    } // set_response()
} // impl<'a> ::method::connection::SetSecureOkMethodFields<'a> for SecureOk<'a>
impl<'a> ::method::connection::StartMethod<'a> for ::Amqp9_1 {
    type Payload = Start<'a>;
} // impl<'a> ::method::connection::StartMethod<'a> for ::Amqp9_1

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Start<'a> {
    version_major: u8,
    version_minor: u8,
    server_properties: ::field::TableEntries<'a>,
    mechanisms: ::std::borrow::Cow<'a, [u8]>,
    locales: ::std::borrow::Cow<'a, [u8]>,
} // struct Start<'a>

impl<'a> Start<'a> {
    pub fn new<S, M, L>(version_major: u8,
                        version_minor: u8,
                        server_properties: S,
                        mechanisms: M,
                        locales: L)
                        -> Self
        where S: Into<::field::TableEntries<'a>>,
              M: Into<::std::borrow::Cow<'a, [u8]>>,
              L: Into<::std::borrow::Cow<'a, [u8]>>
    {
        Start {
            version_major: version_major,
            version_minor: version_minor,
            server_properties: server_properties.into(),
            mechanisms: mechanisms.into(),
            locales: locales.into(),
        } // Start
    } // fn new()
    impl_properties! {
(version_major, set_version_major) -> u8,
(version_minor, set_version_minor) -> u8,
(server_properties, server_properties_mut, set_server_properties) -> &::field::TableEntries<'a>,
(mechanisms, mechanisms_mut, set_mechanisms) -> Cow<[u8]>,
(locales, locales_mut, set_locales) -> Cow<[u8]>,
} // impl_properties
} // impl<'a> Start<'a>
impl<'a> Default for Start<'a> {
    fn default() -> Self {
        Start::new(0, 0, ::field::TableEntries::new(), &[][..], &[][..])
    } // fn default()
} // impl Default for Start

impl<'a> ::Encodable for Start<'a> {
    fn encoded_size(&self) -> usize {
        2 + ::Encodable::encoded_size(&self.server_properties) +
        ::Encodable::encoded_size(&self.mechanisms) +
        ::Encodable::encoded_size(&self.locales)
    } // encoded_size
    fn write_encoded_to<W>(&self, writer: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        try!(::Encodable::write_encoded_to(&self.version_major, writer)); // version_major
        try!(::Encodable::write_encoded_to(&self.version_minor, writer)); // version_minor
        try!(::Encodable::write_encoded_to(&self.server_properties, writer)); // server_properties
        try!(::Encodable::write_encoded_to(&self.mechanisms, writer)); // mechanisms
        try!(::Encodable::write_encoded_to(&self.locales, writer)); // locales

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
    fn class_id(&self) -> u16 {
        10
    } // fn class_id()
    fn method_id(&self) -> u16 {
        10
    } // fn method_id()
} // impl ::Payload for Start
impl<'a> ::method::connection::SetStartMethodFields<'a> for Start<'a> {
    fn set_version_major(&mut self, version_major: u8) {
        self.set_version_major(version_major)
    } // set_version_major()
    fn set_version_minor(&mut self, version_minor: u8) {
        self.set_version_minor(version_minor)
    } // set_version_minor()
    fn set_server_properties<V>(&mut self, server_properties: V)
        where V: Into<::field::TableEntries<'a>>
    {
        self.set_server_properties(server_properties.into())
    } // set_server_properties()
    fn set_mechanisms<V>(&mut self, mechanisms: V)
        where V: Into<::std::borrow::Cow<'a, [u8]>>
    {
        self.set_mechanisms(mechanisms.into())
    } // set_mechanisms()
    fn set_locales<V>(&mut self, locales: V)
        where V: Into<::std::borrow::Cow<'a, [u8]>>
    {
        self.set_locales(locales.into())
    } // set_locales()
} // impl<'a> ::method::connection::SetStartMethodFields<'a> for Start<'a>
impl<'a> ::method::connection::StartOkMethod<'a> for ::Amqp9_1 {
    type Payload = StartOk<'a>;
} // impl<'a> ::method::connection::StartOkMethod<'a> for ::Amqp9_1

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct StartOk<'a> {
    client_properties: ::field::TableEntries<'a>,
    mechanism: ::std::borrow::Cow<'a, str>,
    response: ::std::borrow::Cow<'a, [u8]>,
    locale: ::std::borrow::Cow<'a, str>,
} // struct StartOk<'a>

impl<'a> StartOk<'a> {
    pub fn new<C, M, R, L>(client_properties: C, mechanism: M, response: R, locale: L) -> Self
        where C: Into<::field::TableEntries<'a>>,
              M: Into<::std::borrow::Cow<'a, str>>,
              R: Into<::std::borrow::Cow<'a, [u8]>>,
              L: Into<::std::borrow::Cow<'a, str>>
    {
        StartOk {
            client_properties: client_properties.into(),
            mechanism: mechanism.into(),
            response: response.into(),
            locale: locale.into(),
        } // StartOk
    } // fn new()
    impl_properties! {
(client_properties, client_properties_mut, set_client_properties) -> &::field::TableEntries<'a>,
(mechanism, mechanism_mut, set_mechanism) -> Cow<str>,
(response, response_mut, set_response) -> Cow<[u8]>,
(locale, locale_mut, set_locale) -> Cow<str>,
} // impl_properties
} // impl<'a> StartOk<'a>
impl<'a> Default for StartOk<'a> {
    fn default() -> Self {
        StartOk::new(::field::TableEntries::new(), "", &[][..], "")
    } // fn default()
} // impl Default for StartOk

impl<'a> ::Encodable for StartOk<'a> {
    fn encoded_size(&self) -> usize {
        0 + ::Encodable::encoded_size(&self.client_properties) +
        ::Encodable::encoded_size(&self.mechanism) +
        ::Encodable::encoded_size(&self.response) + ::Encodable::encoded_size(&self.locale)
    } // encoded_size
    fn write_encoded_to<W>(&self, writer: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        try!(::Encodable::write_encoded_to(&self.client_properties, writer)); // client_properties
        try!(::Encodable::write_encoded_to(&self.mechanism, writer)); // mechanism
        try!(::Encodable::write_encoded_to(&self.response, writer)); // response
        try!(::Encodable::write_encoded_to(&self.locale, writer)); // locale

        ::std::result::Result::Ok(())
    } // fn write_encoded_to()
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



impl<'a> ::ProtocolMethodPayload for StartOk<'a> {
    fn class_id(&self) -> u16 {
        10
    } // fn class_id()
    fn method_id(&self) -> u16 {
        11
    } // fn method_id()
} // impl ::Payload for StartOk
impl<'a> ::method::connection::SetStartOkMethodFields<'a> for StartOk<'a> {
    fn set_client_properties<V>(&mut self, client_properties: V)
        where V: Into<::field::TableEntries<'a>>
    {
        self.set_client_properties(client_properties.into())
    } // set_client_properties()
    fn set_mechanism<V>(&mut self, mechanism: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_mechanism(mechanism.into())
    } // set_mechanism()
    fn set_response<V>(&mut self, response: V)
        where V: Into<::std::borrow::Cow<'a, [u8]>>
    {
        self.set_response(response.into())
    } // set_response()
    fn set_locale<V>(&mut self, locale: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_locale(locale.into())
    } // set_locale()
} // impl<'a> ::method::connection::SetStartOkMethodFields<'a> for StartOk<'a>
impl ::method::connection::TuneMethod for ::Amqp9_1 {
    type Payload = Tune;
} // impl ::method::connection::TuneMethod for ::Amqp9_1

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Tune {
    channel_max: u16,
    frame_max: u32,
    heartbeat: u16,
} // struct Tune

impl Tune {
    pub fn new(channel_max: u16, frame_max: u32, heartbeat: u16) -> Self {
        Tune {
            channel_max: channel_max,
            frame_max: frame_max,
            heartbeat: heartbeat,
        } // Tune
    } // fn new()
    impl_properties! {
(channel_max, set_channel_max) -> u16,
(frame_max, set_frame_max) -> u32,
(heartbeat, set_heartbeat) -> u16,
} // impl_properties
} // impl Tune
impl Default for Tune {
    fn default() -> Self {
        Tune::new(0, 0, 0)
    } // fn default()
} // impl Default for Tune

impl ::Encodable for Tune {
    fn encoded_size(&self) -> usize {
        8
    } // encoded_size
    fn write_encoded_to<W>(&self, writer: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        try!(::Encodable::write_encoded_to(&self.channel_max, writer)); // channel_max
        try!(::Encodable::write_encoded_to(&self.frame_max, writer)); // frame_max
        try!(::Encodable::write_encoded_to(&self.heartbeat, writer)); // heartbeat

        ::std::result::Result::Ok(())
    } // fn write_encoded_to()
} // impl Encodable

#[test]
fn test_tune_encodable_bytes_written_matches_len() {
    let payload: Tune = Default::default();
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



impl ::ProtocolMethodPayload for Tune {
    fn class_id(&self) -> u16 {
        10
    } // fn class_id()
    fn method_id(&self) -> u16 {
        30
    } // fn method_id()
} // impl ::Payload for Tune
impl ::method::connection::SetTuneMethodFields for Tune {
    fn set_channel_max(&mut self, channel_max: u16) {
        self.set_channel_max(channel_max)
    } // set_channel_max()
    fn set_frame_max(&mut self, frame_max: u32) {
        self.set_frame_max(frame_max)
    } // set_frame_max()
    fn set_heartbeat(&mut self, heartbeat: u16) {
        self.set_heartbeat(heartbeat)
    } // set_heartbeat()
} // impl ::method::connection::SetTuneMethodFields for Tune
impl ::method::connection::TuneOkMethod for ::Amqp9_1 {
    type Payload = TuneOk;
} // impl ::method::connection::TuneOkMethod for ::Amqp9_1

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct TuneOk {
    channel_max: u16,
    frame_max: u32,
    heartbeat: u16,
} // struct TuneOk

impl TuneOk {
    pub fn new(channel_max: u16, frame_max: u32, heartbeat: u16) -> Self {
        TuneOk {
            channel_max: channel_max,
            frame_max: frame_max,
            heartbeat: heartbeat,
        } // TuneOk
    } // fn new()
    impl_properties! {
(channel_max, set_channel_max) -> u16,
(frame_max, set_frame_max) -> u32,
(heartbeat, set_heartbeat) -> u16,
} // impl_properties
} // impl TuneOk
impl Default for TuneOk {
    fn default() -> Self {
        TuneOk::new(0, 0, 0)
    } // fn default()
} // impl Default for TuneOk

impl ::Encodable for TuneOk {
    fn encoded_size(&self) -> usize {
        8
    } // encoded_size
    fn write_encoded_to<W>(&self, writer: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        try!(::Encodable::write_encoded_to(&self.channel_max, writer)); // channel_max
        try!(::Encodable::write_encoded_to(&self.frame_max, writer)); // frame_max
        try!(::Encodable::write_encoded_to(&self.heartbeat, writer)); // heartbeat

        ::std::result::Result::Ok(())
    } // fn write_encoded_to()
} // impl Encodable

#[test]
fn test_tune_ok_encodable_bytes_written_matches_len() {
    let payload: TuneOk = Default::default();
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



impl ::ProtocolMethodPayload for TuneOk {
    fn class_id(&self) -> u16 {
        10
    } // fn class_id()
    fn method_id(&self) -> u16 {
        31
    } // fn method_id()
} // impl ::Payload for TuneOk
impl ::method::connection::SetTuneOkMethodFields for TuneOk {
    fn set_channel_max(&mut self, channel_max: u16) {
        self.set_channel_max(channel_max)
    } // set_channel_max()
    fn set_frame_max(&mut self, frame_max: u32) {
        self.set_frame_max(frame_max)
    } // set_frame_max()
    fn set_heartbeat(&mut self, heartbeat: u16) {
        self.set_heartbeat(heartbeat)
    } // set_heartbeat()
} // impl ::method::connection::SetTuneOkMethodFields for TuneOk

#[derive(Debug)]
pub enum ClassMethod<'a> {
    Close(Close<'a>),
    CloseOk(CloseOk),
    Open(Open<'a>),
    OpenOk(OpenOk),
    Secure(Secure<'a>),
    SecureOk(SecureOk<'a>),
    Start(Start<'a>),
    StartOk(StartOk<'a>),
    Tune(Tune),
    TuneOk(TuneOk),
} // enum ClassMethod


impl<'a> ::Encodable for ClassMethod<'a> {
    fn encoded_size(&self) -> usize {
        match *self {
            ClassMethod::Close(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::CloseOk(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::Open(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::OpenOk(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::Secure(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::SecureOk(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::Start(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::StartOk(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::Tune(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::TuneOk(ref method) => ::Encodable::encoded_size(method),

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
            ClassMethod::Close(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::CloseOk(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::Open(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::OpenOk(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::Secure(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::SecureOk(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::Start(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::StartOk(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::Tune(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::TuneOk(ref method) => ::ProtocolMethodPayload::class_id(method),

        } // match *self

    } // fn class_id

    fn method_id(&self) -> u16 {
        match *self {
            ClassMethod::Close(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::CloseOk(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::Open(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::OpenOk(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::Secure(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::SecureOk(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::Start(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::StartOk(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::Tune(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::TuneOk(ref method) => ::ProtocolMethodPayload::method_id(method),

        } // match *self

    } // fn method_id
} // impl ProtocolMethodPayload for ClassMethod
