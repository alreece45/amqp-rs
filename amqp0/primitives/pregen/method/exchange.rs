// Generated by build.rs script in amqp0-primitives
// Pre-generated files are used by default. Generation is done with the amqp0-codegen crate
//
// To regenerate, and not use pre-generated files, use: cargo --features="amqp0-build-primitives"
// To format and replace the pre-generated files, use: cargo --features="amqp0-pregen-primitives"
//
// EDITORS BEWARE: Your modifications may be overridden
pub trait BindMethod<'a> {
    type Payload: Default + SetBindMethodFields<'a>;
} // pub trait BindMethod<'a>

pub trait SetBindMethodFields<'a> {
    fn set_arguments<V>(&mut self, _: V) where V: Into<::field::TableEntries<'a>> {}
    fn set_destination<V>(&mut self, _: V) where V: Into<::std::borrow::Cow<'a, str>> {}
    fn set_no_wait(&mut self, _: bool) {}
    fn set_routing_key<V>(&mut self, _: V) where V: Into<::std::borrow::Cow<'a, str>> {}
    fn set_source<V>(&mut self, _: V) where V: Into<::std::borrow::Cow<'a, str>> {}
} // pub trait SetBindMethodFields<'a>

pub struct BindBuilder<T>
    where T: ::Encodable
{
    payload: T,
} // struct BindBuilder


impl<T> BindBuilder<T>
    where T: Default + ::Encodable
{
    pub fn new() -> Self {
        Default::default()
    }
} // impl Builder (new)

impl<T> BindBuilder<T>
    where T: ::Encodable
{
    pub fn build(self) -> T {
        self.payload
    }
} // impl<T> BindBuilder<T>

impl<T> Default for BindBuilder<T>
    where T: ::Encodable + Default
{
    fn default() -> Self {
        BindBuilder { payload: Default::default() }
    }
} // impl Default for BindBuilder
impl<'a, T> BindBuilder<T>
    where T: ::Encodable + ::Content<'a> + SetBindMethodFields<'a>
{
    pub fn arguments<V>(mut self, arguments: V) -> Self
        where V: Into<::field::TableEntries<'a>>
    {
        SetBindMethodFields::set_arguments(&mut self.payload, arguments.into());
        self
    } // set_arguments()
    pub fn destination<V>(mut self, destination: V) -> Self
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        SetBindMethodFields::set_destination(&mut self.payload, destination.into());
        self
    } // set_destination()
    pub fn no_wait(mut self, no_wait: bool) -> Self {
        SetBindMethodFields::set_no_wait(&mut self.payload, no_wait);
        self
    } // set_no_wait()
    pub fn routing_key<V>(mut self, routing_key: V) -> Self
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        SetBindMethodFields::set_routing_key(&mut self.payload, routing_key.into());
        self
    } // set_routing_key()
    pub fn source<V>(mut self, source: V) -> Self
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        SetBindMethodFields::set_source(&mut self.payload, source.into());
        self
    } // set_source()
} // impl<'a, T> BindBuilder<T>
pub trait BindOkMethod {
    type Payload: Default;
} // pub trait BindOkMethod

pub struct BindOkBuilder<T>
    where T: ::Encodable
{
    payload: T,
} // struct BindOkBuilder


impl<T> BindOkBuilder<T>
    where T: Default + ::Encodable
{
    pub fn new() -> Self {
        Default::default()
    }
} // impl Builder (new)

impl<T> BindOkBuilder<T>
    where T: ::Encodable
{
    pub fn build(self) -> T {
        self.payload
    }
} // impl<T> BindOkBuilder<T>

impl<T> Default for BindOkBuilder<T>
    where T: ::Encodable + Default
{
    fn default() -> Self {
        BindOkBuilder { payload: Default::default() }
    }
} // impl Default for BindOkBuilder
pub trait BoundMethod<'a> {
    type Payload: Default + SetBoundMethodFields<'a>;
} // pub trait BoundMethod<'a>

pub trait SetBoundMethodFields<'a> {
    fn set_exchange<V>(&mut self, _: V) where V: Into<::std::borrow::Cow<'a, str>> {}
    fn set_queue<V>(&mut self, _: V) where V: Into<::std::borrow::Cow<'a, str>> {}
    fn set_routing_key<V>(&mut self, _: V) where V: Into<::std::borrow::Cow<'a, str>> {}
} // pub trait SetBoundMethodFields<'a>

pub struct BoundBuilder<T>
    where T: ::Encodable
{
    payload: T,
} // struct BoundBuilder


impl<T> BoundBuilder<T>
    where T: Default + ::Encodable
{
    pub fn new() -> Self {
        Default::default()
    }
} // impl Builder (new)

impl<T> BoundBuilder<T>
    where T: ::Encodable
{
    pub fn build(self) -> T {
        self.payload
    }
} // impl<T> BoundBuilder<T>

impl<T> Default for BoundBuilder<T>
    where T: ::Encodable + Default
{
    fn default() -> Self {
        BoundBuilder { payload: Default::default() }
    }
} // impl Default for BoundBuilder
impl<'a, T> BoundBuilder<T>
    where T: ::Encodable + ::Content<'a> + SetBoundMethodFields<'a>
{
    pub fn exchange<V>(mut self, exchange: V) -> Self
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        SetBoundMethodFields::set_exchange(&mut self.payload, exchange.into());
        self
    } // set_exchange()
    pub fn queue<V>(mut self, queue: V) -> Self
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        SetBoundMethodFields::set_queue(&mut self.payload, queue.into());
        self
    } // set_queue()
    pub fn routing_key<V>(mut self, routing_key: V) -> Self
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        SetBoundMethodFields::set_routing_key(&mut self.payload, routing_key.into());
        self
    } // set_routing_key()
} // impl<'a, T> BoundBuilder<T>
pub trait BoundOkMethod<'a> {
    type Payload: Default + SetBoundOkMethodFields<'a>;
} // pub trait BoundOkMethod<'a>

pub trait SetBoundOkMethodFields<'a> {
    fn set_reply_code(&mut self, _: u16) {}
    fn set_reply_text<V>(&mut self, _: V) where V: Into<::std::borrow::Cow<'a, str>> {}
} // pub trait SetBoundOkMethodFields<'a>

pub struct BoundOkBuilder<T>
    where T: ::Encodable
{
    payload: T,
} // struct BoundOkBuilder


impl<T> BoundOkBuilder<T>
    where T: Default + ::Encodable
{
    pub fn new() -> Self {
        Default::default()
    }
} // impl Builder (new)

impl<T> BoundOkBuilder<T>
    where T: ::Encodable
{
    pub fn build(self) -> T {
        self.payload
    }
} // impl<T> BoundOkBuilder<T>

impl<T> Default for BoundOkBuilder<T>
    where T: ::Encodable + Default
{
    fn default() -> Self {
        BoundOkBuilder { payload: Default::default() }
    }
} // impl Default for BoundOkBuilder
impl<'a, T> BoundOkBuilder<T>
    where T: ::Encodable + ::Content<'a> + SetBoundOkMethodFields<'a>
{
    pub fn reply_code(mut self, reply_code: u16) -> Self {
        SetBoundOkMethodFields::set_reply_code(&mut self.payload, reply_code);
        self
    } // set_reply_code()
    pub fn reply_text<V>(mut self, reply_text: V) -> Self
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        SetBoundOkMethodFields::set_reply_text(&mut self.payload, reply_text.into());
        self
    } // set_reply_text()
} // impl<'a, T> BoundOkBuilder<T>
pub trait DeclareMethod<'a> {
    type Payload: Default + SetDeclareMethodFields<'a>;
} // pub trait DeclareMethod<'a>

pub trait SetDeclareMethodFields<'a> {
    fn set_arguments<V>(&mut self, _: V) where V: Into<::field::TableEntries<'a>> {}
    fn set_auto_delete(&mut self, _: bool) {}
    fn set_durable(&mut self, _: bool) {}
    fn set_exchange<V>(&mut self, _: V) where V: Into<::std::borrow::Cow<'a, str>> {}
    fn set_internal(&mut self, _: bool) {}
    fn set_no_wait(&mut self, _: bool) {}
    fn set_passive(&mut self, _: bool) {}
    fn set_ticket(&mut self, _: u16) {}
    fn set_ty<V>(&mut self, _: V) where V: Into<::std::borrow::Cow<'a, str>> {}
} // pub trait SetDeclareMethodFields<'a>

pub struct DeclareBuilder<T>
    where T: ::Encodable
{
    payload: T,
} // struct DeclareBuilder


impl<T> DeclareBuilder<T>
    where T: Default + ::Encodable
{
    pub fn new() -> Self {
        Default::default()
    }
} // impl Builder (new)

impl<T> DeclareBuilder<T>
    where T: ::Encodable
{
    pub fn build(self) -> T {
        self.payload
    }
} // impl<T> DeclareBuilder<T>

impl<T> Default for DeclareBuilder<T>
    where T: ::Encodable + Default
{
    fn default() -> Self {
        DeclareBuilder { payload: Default::default() }
    }
} // impl Default for DeclareBuilder
impl<'a, T> DeclareBuilder<T>
    where T: ::Encodable + ::Content<'a> + SetDeclareMethodFields<'a>
{
    pub fn arguments<V>(mut self, arguments: V) -> Self
        where V: Into<::field::TableEntries<'a>>
    {
        SetDeclareMethodFields::set_arguments(&mut self.payload, arguments.into());
        self
    } // set_arguments()
    pub fn auto_delete(mut self, auto_delete: bool) -> Self {
        SetDeclareMethodFields::set_auto_delete(&mut self.payload, auto_delete);
        self
    } // set_auto_delete()
    pub fn durable(mut self, durable: bool) -> Self {
        SetDeclareMethodFields::set_durable(&mut self.payload, durable);
        self
    } // set_durable()
    pub fn exchange<V>(mut self, exchange: V) -> Self
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        SetDeclareMethodFields::set_exchange(&mut self.payload, exchange.into());
        self
    } // set_exchange()
    pub fn internal(mut self, internal: bool) -> Self {
        SetDeclareMethodFields::set_internal(&mut self.payload, internal);
        self
    } // set_internal()
    pub fn no_wait(mut self, no_wait: bool) -> Self {
        SetDeclareMethodFields::set_no_wait(&mut self.payload, no_wait);
        self
    } // set_no_wait()
    pub fn passive(mut self, passive: bool) -> Self {
        SetDeclareMethodFields::set_passive(&mut self.payload, passive);
        self
    } // set_passive()
    pub fn ticket(mut self, ticket: u16) -> Self {
        SetDeclareMethodFields::set_ticket(&mut self.payload, ticket);
        self
    } // set_ticket()
    pub fn ty<V>(mut self, ty: V) -> Self
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        SetDeclareMethodFields::set_ty(&mut self.payload, ty.into());
        self
    } // set_ty()
} // impl<'a, T> DeclareBuilder<T>
pub trait DeclareOkMethod {
    type Payload: Default;
} // pub trait DeclareOkMethod

pub struct DeclareOkBuilder<T>
    where T: ::Encodable
{
    payload: T,
} // struct DeclareOkBuilder


impl<T> DeclareOkBuilder<T>
    where T: Default + ::Encodable
{
    pub fn new() -> Self {
        Default::default()
    }
} // impl Builder (new)

impl<T> DeclareOkBuilder<T>
    where T: ::Encodable
{
    pub fn build(self) -> T {
        self.payload
    }
} // impl<T> DeclareOkBuilder<T>

impl<T> Default for DeclareOkBuilder<T>
    where T: ::Encodable + Default
{
    fn default() -> Self {
        DeclareOkBuilder { payload: Default::default() }
    }
} // impl Default for DeclareOkBuilder
pub trait DeleteMethod<'a> {
    type Payload: Default + SetDeleteMethodFields<'a>;
} // pub trait DeleteMethod<'a>

pub trait SetDeleteMethodFields<'a> {
    fn set_exchange<V>(&mut self, _: V) where V: Into<::std::borrow::Cow<'a, str>> {}
    fn set_if_unused(&mut self, _: bool) {}
    fn set_no_wait(&mut self, _: bool) {}
    fn set_ticket(&mut self, _: u16) {}
} // pub trait SetDeleteMethodFields<'a>

pub struct DeleteBuilder<T>
    where T: ::Encodable
{
    payload: T,
} // struct DeleteBuilder


impl<T> DeleteBuilder<T>
    where T: Default + ::Encodable
{
    pub fn new() -> Self {
        Default::default()
    }
} // impl Builder (new)

impl<T> DeleteBuilder<T>
    where T: ::Encodable
{
    pub fn build(self) -> T {
        self.payload
    }
} // impl<T> DeleteBuilder<T>

impl<T> Default for DeleteBuilder<T>
    where T: ::Encodable + Default
{
    fn default() -> Self {
        DeleteBuilder { payload: Default::default() }
    }
} // impl Default for DeleteBuilder
impl<'a, T> DeleteBuilder<T>
    where T: ::Encodable + ::Content<'a> + SetDeleteMethodFields<'a>
{
    pub fn exchange<V>(mut self, exchange: V) -> Self
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        SetDeleteMethodFields::set_exchange(&mut self.payload, exchange.into());
        self
    } // set_exchange()
    pub fn if_unused(mut self, if_unused: bool) -> Self {
        SetDeleteMethodFields::set_if_unused(&mut self.payload, if_unused);
        self
    } // set_if_unused()
    pub fn no_wait(mut self, no_wait: bool) -> Self {
        SetDeleteMethodFields::set_no_wait(&mut self.payload, no_wait);
        self
    } // set_no_wait()
    pub fn ticket(mut self, ticket: u16) -> Self {
        SetDeleteMethodFields::set_ticket(&mut self.payload, ticket);
        self
    } // set_ticket()
} // impl<'a, T> DeleteBuilder<T>
pub trait DeleteOkMethod {
    type Payload: Default;
} // pub trait DeleteOkMethod

pub struct DeleteOkBuilder<T>
    where T: ::Encodable
{
    payload: T,
} // struct DeleteOkBuilder


impl<T> DeleteOkBuilder<T>
    where T: Default + ::Encodable
{
    pub fn new() -> Self {
        Default::default()
    }
} // impl Builder (new)

impl<T> DeleteOkBuilder<T>
    where T: ::Encodable
{
    pub fn build(self) -> T {
        self.payload
    }
} // impl<T> DeleteOkBuilder<T>

impl<T> Default for DeleteOkBuilder<T>
    where T: ::Encodable + Default
{
    fn default() -> Self {
        DeleteOkBuilder { payload: Default::default() }
    }
} // impl Default for DeleteOkBuilder
pub trait UnbindMethod<'a> {
    type Payload: Default + SetUnbindMethodFields<'a>;
} // pub trait UnbindMethod<'a>

pub trait SetUnbindMethodFields<'a> {
    fn set_arguments<V>(&mut self, _: V) where V: Into<::field::TableEntries<'a>> {}
    fn set_destination<V>(&mut self, _: V) where V: Into<::std::borrow::Cow<'a, str>> {}
    fn set_no_wait(&mut self, _: bool) {}
    fn set_routing_key<V>(&mut self, _: V) where V: Into<::std::borrow::Cow<'a, str>> {}
    fn set_source<V>(&mut self, _: V) where V: Into<::std::borrow::Cow<'a, str>> {}
} // pub trait SetUnbindMethodFields<'a>

pub struct UnbindBuilder<T>
    where T: ::Encodable
{
    payload: T,
} // struct UnbindBuilder


impl<T> UnbindBuilder<T>
    where T: Default + ::Encodable
{
    pub fn new() -> Self {
        Default::default()
    }
} // impl Builder (new)

impl<T> UnbindBuilder<T>
    where T: ::Encodable
{
    pub fn build(self) -> T {
        self.payload
    }
} // impl<T> UnbindBuilder<T>

impl<T> Default for UnbindBuilder<T>
    where T: ::Encodable + Default
{
    fn default() -> Self {
        UnbindBuilder { payload: Default::default() }
    }
} // impl Default for UnbindBuilder
impl<'a, T> UnbindBuilder<T>
    where T: ::Encodable + ::Content<'a> + SetUnbindMethodFields<'a>
{
    pub fn arguments<V>(mut self, arguments: V) -> Self
        where V: Into<::field::TableEntries<'a>>
    {
        SetUnbindMethodFields::set_arguments(&mut self.payload, arguments.into());
        self
    } // set_arguments()
    pub fn destination<V>(mut self, destination: V) -> Self
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        SetUnbindMethodFields::set_destination(&mut self.payload, destination.into());
        self
    } // set_destination()
    pub fn no_wait(mut self, no_wait: bool) -> Self {
        SetUnbindMethodFields::set_no_wait(&mut self.payload, no_wait);
        self
    } // set_no_wait()
    pub fn routing_key<V>(mut self, routing_key: V) -> Self
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        SetUnbindMethodFields::set_routing_key(&mut self.payload, routing_key.into());
        self
    } // set_routing_key()
    pub fn source<V>(mut self, source: V) -> Self
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        SetUnbindMethodFields::set_source(&mut self.payload, source.into());
        self
    } // set_source()
} // impl<'a, T> UnbindBuilder<T>
pub trait UnbindOkMethod {
    type Payload: Default;
} // pub trait UnbindOkMethod

pub struct UnbindOkBuilder<T>
    where T: ::Encodable
{
    payload: T,
} // struct UnbindOkBuilder


impl<T> UnbindOkBuilder<T>
    where T: Default + ::Encodable
{
    pub fn new() -> Self {
        Default::default()
    }
} // impl Builder (new)

impl<T> UnbindOkBuilder<T>
    where T: ::Encodable
{
    pub fn build(self) -> T {
        self.payload
    }
} // impl<T> UnbindOkBuilder<T>

impl<T> Default for UnbindOkBuilder<T>
    where T: ::Encodable + Default
{
    fn default() -> Self {
        UnbindOkBuilder { payload: Default::default() }
    }
} // impl Default for UnbindOkBuilder