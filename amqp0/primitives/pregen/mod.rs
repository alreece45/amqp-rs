// Generated by build.rs script in the amqp0-primitives crate.
// Pre-generated files are used by default. Generation is done with the amqp0-codegen crate.
//
// To regenerate, ignoring the pre-generated files, use: cargo --features="amqp0-build-primitives"
// To format and replace the pre-generated files, use: cargo --features="amqp0-pregen-primitives"
//
// EDITORS BEWARE: Your modifications may be overridden or removed.

pub mod method;
pub mod message;
pub mod amqp9_1;
pub mod amqp9_0;
pub mod amqp8_0;
pub mod rabbitmq9_1;
pub mod qpid9_0;
pub mod qpid8_0;

// Frame types ids shared among multiple specs
//
pub const FRAME_METHOD: u8 = 1;
pub const FRAME_HEADER: u8 = 2;
pub const FRAME_BODY: u8 = 3;
pub const FRAME_OOB_METHOD: u8 = 4;
pub const FRAME_OOB_HEADER: u8 = 5;
pub const FRAME_OOB_BODY: u8 = 6;
pub const FRAME_TRACE: u8 = 7;
pub const FRAME_HEARTBEAT: u8 = 8;

// Index values for classes shared among multiple specs
//
// Sometimes, the index value is repeated in different classes, but these are not reused
// within a single protocol
//
// Classes are currently only considered common if they are used in more than one
// spec. This behavior *may* change in the future as more specs are added.
//
pub const CLASS_CONNECTION: u16 = 10;
pub const CLASS_CHANNEL: u16 = 20;
pub const CLASS_ACCESS: u16 = 30;
pub const CLASS_EXCHANGE: u16 = 40;
pub const CLASS_QUEUE: u16 = 50;
pub const CLASS_BASIC: u16 = 60;
pub const CLASS_FILE: u16 = 70;
pub const CLASS_STREAM: u16 = 80;
pub const CLASS_CONFIRM: u16 = 85;
pub const CLASS_TX: u16 = 90;
pub const CLASS_DTX: u16 = 100;
pub const CLASS_TUNNEL: u16 = 110;
pub const CLASS_MESSAGE: u16 = 120;
pub const CLASS_TEST: u16 = 120;

// Index values for methods common among the different specs
//
// Methods are only considered common when:
//
//   * The index value is consistent across all of the specs
//   * The method is used in more than one primalgen.spec
//
// This may change in the future-- in that case, methods *may* be removed, or
// one of the requirements may be relaxed.
//
pub const METHOD_ACCESS_REQUEST: u16 = 10;
pub const METHOD_ACCESS_REQUEST_OK: u16 = 11;

pub const METHOD_BASIC_QOS: u16 = 10;
pub const METHOD_BASIC_QOS_OK: u16 = 11;
pub const METHOD_BASIC_CONSUME: u16 = 20;
pub const METHOD_BASIC_CONSUME_OK: u16 = 21;
pub const METHOD_BASIC_CANCEL: u16 = 30;
pub const METHOD_BASIC_CANCEL_OK: u16 = 31;
pub const METHOD_BASIC_PUBLISH: u16 = 40;
pub const METHOD_BASIC_RETURN: u16 = 50;
pub const METHOD_BASIC_DELIVER: u16 = 60;
pub const METHOD_BASIC_GET: u16 = 70;
pub const METHOD_BASIC_GET_OK: u16 = 71;
pub const METHOD_BASIC_GET_EMPTY: u16 = 72;
pub const METHOD_BASIC_ACK: u16 = 80;
pub const METHOD_BASIC_REJECT: u16 = 90;
pub const METHOD_BASIC_RECOVER_ASYNC: u16 = 100;
pub const METHOD_BASIC_RECOVER_SYNC_OK: u16 = 101;
pub const METHOD_BASIC_RECOVER_SYNC: u16 = 102;
pub const METHOD_BASIC_NACK: u16 = 120;

pub const METHOD_CHANNEL_OPEN: u16 = 10;
pub const METHOD_CHANNEL_OPEN_OK: u16 = 11;
pub const METHOD_CHANNEL_FLOW: u16 = 20;
pub const METHOD_CHANNEL_FLOW_OK: u16 = 21;
pub const METHOD_CHANNEL_ALERT: u16 = 30;
pub const METHOD_CHANNEL_CLOSE: u16 = 40;
pub const METHOD_CHANNEL_CLOSE_OK: u16 = 41;
pub const METHOD_CHANNEL_RESUME: u16 = 50;
pub const METHOD_CHANNEL_PING: u16 = 60;
pub const METHOD_CHANNEL_PONG: u16 = 70;
pub const METHOD_CHANNEL_OK: u16 = 80;

pub const METHOD_CONFIRM_SELECT: u16 = 10;
pub const METHOD_CONFIRM_SELECT_OK: u16 = 11;

pub const METHOD_CONNECTION_START: u16 = 10;
pub const METHOD_CONNECTION_START_OK: u16 = 11;
pub const METHOD_CONNECTION_SECURE: u16 = 20;
pub const METHOD_CONNECTION_SECURE_OK: u16 = 21;
pub const METHOD_CONNECTION_TUNE: u16 = 30;
pub const METHOD_CONNECTION_TUNE_OK: u16 = 31;
pub const METHOD_CONNECTION_OPEN: u16 = 40;
pub const METHOD_CONNECTION_OPEN_OK: u16 = 41;
pub const METHOD_CONNECTION_BLOCKED: u16 = 60;
pub const METHOD_CONNECTION_UNBLOCKED: u16 = 61;

pub const METHOD_DTX_SELECT: u16 = 10;
pub const METHOD_DTX_SELECT_OK: u16 = 11;
pub const METHOD_DTX_START: u16 = 20;
pub const METHOD_DTX_START_OK: u16 = 21;

pub const METHOD_EXCHANGE_DECLARE: u16 = 10;
pub const METHOD_EXCHANGE_DECLARE_OK: u16 = 11;
pub const METHOD_EXCHANGE_DELETE: u16 = 20;
pub const METHOD_EXCHANGE_DELETE_OK: u16 = 21;
pub const METHOD_EXCHANGE_BOUND: u16 = 22;
pub const METHOD_EXCHANGE_BOUND_OK: u16 = 23;
pub const METHOD_EXCHANGE_BIND: u16 = 30;
pub const METHOD_EXCHANGE_BIND_OK: u16 = 31;
pub const METHOD_EXCHANGE_UNBIND: u16 = 40;
pub const METHOD_EXCHANGE_UNBIND_OK: u16 = 51;

pub const METHOD_FILE_QOS: u16 = 10;
pub const METHOD_FILE_QOS_OK: u16 = 11;
pub const METHOD_FILE_CONSUME: u16 = 20;
pub const METHOD_FILE_CONSUME_OK: u16 = 21;
pub const METHOD_FILE_CANCEL: u16 = 30;
pub const METHOD_FILE_CANCEL_OK: u16 = 31;
pub const METHOD_FILE_OPEN: u16 = 40;
pub const METHOD_FILE_OPEN_OK: u16 = 41;
pub const METHOD_FILE_STAGE: u16 = 50;
pub const METHOD_FILE_PUBLISH: u16 = 60;
pub const METHOD_FILE_RETURN: u16 = 70;
pub const METHOD_FILE_DELIVER: u16 = 80;
pub const METHOD_FILE_ACK: u16 = 90;
pub const METHOD_FILE_REJECT: u16 = 100;

pub const METHOD_MESSAGE_TRANSFER: u16 = 10;
pub const METHOD_MESSAGE_CONSUME: u16 = 20;
pub const METHOD_MESSAGE_CANCEL: u16 = 30;
pub const METHOD_MESSAGE_GET: u16 = 40;
pub const METHOD_MESSAGE_RECOVER: u16 = 50;
pub const METHOD_MESSAGE_OPEN: u16 = 60;
pub const METHOD_MESSAGE_CLOSE: u16 = 70;
pub const METHOD_MESSAGE_APPEND: u16 = 80;
pub const METHOD_MESSAGE_CHECKPOINT: u16 = 90;
pub const METHOD_MESSAGE_RESUME: u16 = 100;
pub const METHOD_MESSAGE_QOS: u16 = 110;
pub const METHOD_MESSAGE_OK: u16 = 500;
pub const METHOD_MESSAGE_EMPTY: u16 = 510;
pub const METHOD_MESSAGE_REJECT: u16 = 520;
pub const METHOD_MESSAGE_OFFSET: u16 = 530;

pub const METHOD_QUEUE_DECLARE: u16 = 10;
pub const METHOD_QUEUE_DECLARE_OK: u16 = 11;
pub const METHOD_QUEUE_BIND: u16 = 20;
pub const METHOD_QUEUE_BIND_OK: u16 = 21;
pub const METHOD_QUEUE_PURGE: u16 = 30;
pub const METHOD_QUEUE_PURGE_OK: u16 = 31;
pub const METHOD_QUEUE_DELETE: u16 = 40;
pub const METHOD_QUEUE_DELETE_OK: u16 = 41;
pub const METHOD_QUEUE_UNBIND: u16 = 50;
pub const METHOD_QUEUE_UNBIND_OK: u16 = 51;

pub const METHOD_STREAM_QOS: u16 = 10;
pub const METHOD_STREAM_QOS_OK: u16 = 11;
pub const METHOD_STREAM_CONSUME: u16 = 20;
pub const METHOD_STREAM_CONSUME_OK: u16 = 21;
pub const METHOD_STREAM_CANCEL: u16 = 30;
pub const METHOD_STREAM_CANCEL_OK: u16 = 31;
pub const METHOD_STREAM_PUBLISH: u16 = 40;
pub const METHOD_STREAM_RETURN: u16 = 50;
pub const METHOD_STREAM_DELIVER: u16 = 60;

pub const METHOD_TEST_INTEGER: u16 = 10;
pub const METHOD_TEST_INTEGER_OK: u16 = 11;
pub const METHOD_TEST_STRING: u16 = 20;
pub const METHOD_TEST_STRING_OK: u16 = 21;
pub const METHOD_TEST_TABLE: u16 = 30;
pub const METHOD_TEST_TABLE_OK: u16 = 31;
pub const METHOD_TEST_CONTENT: u16 = 40;
pub const METHOD_TEST_CONTENT_OK: u16 = 41;

pub const METHOD_TUNNEL_REQUEST: u16 = 10;

pub const METHOD_TX_SELECT: u16 = 10;
pub const METHOD_TX_SELECT_OK: u16 = 11;
pub const METHOD_TX_COMMIT: u16 = 20;
pub const METHOD_TX_COMMIT_OK: u16 = 21;
pub const METHOD_TX_ROLLBACK: u16 = 30;
pub const METHOD_TX_ROLLBACK_OK: u16 = 31;

pub trait Protocol<'a> {
    type Frame: 'a;

    fn protocol_header() -> &'static [u8];

fn access_request() -> <Self as ::method::access::RequestMethod<'a>>::Payload
where Self: ::method::access::RequestMethod<'a>
{
        Default::default()
    }
fn access_request_ok() -> <Self as ::method::access::RequestOkMethod>::Payload
where Self: ::method::access::RequestOkMethod
{
        Default::default()
    }
    fn basic_ack() -> <Self as ::method::basic::AckMethod>::Payload
        where Self: ::method::basic::AckMethod
    {
        Default::default()
    }
fn basic_cancel() -> <Self as ::method::basic::CancelMethod<'a>>::Payload
where Self: ::method::basic::CancelMethod<'a>
{
        Default::default()
    }
fn basic_cancel_ok() -> <Self as ::method::basic::CancelOkMethod<'a>>::Payload
where Self: ::method::basic::CancelOkMethod<'a>
{
        Default::default()
    }
fn basic_consume() -> <Self as ::method::basic::ConsumeMethod<'a>>::Payload
where Self: ::method::basic::ConsumeMethod<'a>
{
        Default::default()
    }
fn basic_consume_ok() -> <Self as ::method::basic::ConsumeOkMethod<'a>>::Payload
where Self: ::method::basic::ConsumeOkMethod<'a>
{
        Default::default()
    }
fn basic_deliver() -> <Self as ::method::basic::DeliverMethod<'a>>::Payload
where Self: ::method::basic::DeliverMethod<'a>
{
        Default::default()
    }
    fn basic_get() -> <Self as ::method::basic::GetMethod<'a>>::Payload
        where Self: ::method::basic::GetMethod<'a>
    {
        Default::default()
    }
fn basic_get_empty() -> <Self as ::method::basic::GetEmptyMethod<'a>>::Payload
where Self: ::method::basic::GetEmptyMethod<'a>
{
        Default::default()
    }
fn basic_get_ok() -> <Self as ::method::basic::GetOkMethod<'a>>::Payload
where Self: ::method::basic::GetOkMethod<'a>
{
        Default::default()
    }
    fn basic_nack() -> <Self as ::method::basic::NackMethod>::Payload
        where Self: ::method::basic::NackMethod
    {
        Default::default()
    }
fn basic_publish() -> <Self as ::method::basic::PublishMethod<'a>>::Payload
where Self: ::method::basic::PublishMethod<'a>
{
        Default::default()
    }
    fn basic_qos() -> <Self as ::method::basic::QosMethod>::Payload
        where Self: ::method::basic::QosMethod
    {
        Default::default()
    }
    fn basic_qos_ok() -> <Self as ::method::basic::QosOkMethod>::Payload
        where Self: ::method::basic::QosOkMethod
    {
        Default::default()
    }
    fn basic_recover() -> <Self as ::method::basic::RecoverMethod>::Payload
        where Self: ::method::basic::RecoverMethod
    {
        Default::default()
    }
fn basic_recover_async() -> <Self as ::method::basic::RecoverAsyncMethod>::Payload
where Self: ::method::basic::RecoverAsyncMethod
{
        Default::default()
    }
fn basic_recover_ok() -> <Self as ::method::basic::RecoverOkMethod>::Payload
where Self: ::method::basic::RecoverOkMethod
{
        Default::default()
    }
fn basic_recover_sync() -> <Self as ::method::basic::RecoverSyncMethod>::Payload
where Self: ::method::basic::RecoverSyncMethod
{
        Default::default()
    }
fn basic_recover_sync_ok() -> <Self as ::method::basic::RecoverSyncOkMethod>::Payload
where Self: ::method::basic::RecoverSyncOkMethod
{
        Default::default()
    }
    fn basic_reject() -> <Self as ::method::basic::RejectMethod>::Payload
        where Self: ::method::basic::RejectMethod
    {
        Default::default()
    }
fn basic_return() -> <Self as ::method::basic::ReturnMethod<'a>>::Payload
where Self: ::method::basic::ReturnMethod<'a>
{
        Default::default()
    }
fn channel_alert() -> <Self as ::method::channel::AlertMethod<'a>>::Payload
where Self: ::method::channel::AlertMethod<'a>
{
        Default::default()
    }
fn channel_close() -> <Self as ::method::channel::CloseMethod<'a>>::Payload
where Self: ::method::channel::CloseMethod<'a>
{
        Default::default()
    }
fn channel_close_ok() -> <Self as ::method::channel::CloseOkMethod>::Payload
where Self: ::method::channel::CloseOkMethod
{
        Default::default()
    }
    fn channel_flow() -> <Self as ::method::channel::FlowMethod>::Payload
        where Self: ::method::channel::FlowMethod
    {
        Default::default()
    }
    fn channel_flow_ok() -> <Self as ::method::channel::FlowOkMethod>::Payload
        where Self: ::method::channel::FlowOkMethod
    {
        Default::default()
    }
    fn channel_ok() -> <Self as ::method::channel::OkMethod>::Payload
        where Self: ::method::channel::OkMethod
    {
        Default::default()
    }
fn channel_open() -> <Self as ::method::channel::OpenMethod<'a>>::Payload
where Self: ::method::channel::OpenMethod<'a>
{
        Default::default()
    }
fn channel_open_ok() -> <Self as ::method::channel::OpenOkMethod<'a>>::Payload
where Self: ::method::channel::OpenOkMethod<'a>
{
        Default::default()
    }
    fn channel_ping() -> <Self as ::method::channel::PingMethod>::Payload
        where Self: ::method::channel::PingMethod
    {
        Default::default()
    }
    fn channel_pong() -> <Self as ::method::channel::PongMethod>::Payload
        where Self: ::method::channel::PongMethod
    {
        Default::default()
    }
fn channel_resume() -> <Self as ::method::channel::ResumeMethod<'a>>::Payload
where Self: ::method::channel::ResumeMethod<'a>
{
        Default::default()
    }
    fn confirm_select() -> <Self as ::method::confirm::SelectMethod>::Payload
        where Self: ::method::confirm::SelectMethod
    {
        Default::default()
    }
fn confirm_select_ok() -> <Self as ::method::confirm::SelectOkMethod>::Payload
where Self: ::method::confirm::SelectOkMethod
{
        Default::default()
    }
fn connection_blocked() -> <Self as ::method::connection::BlockedMethod<'a>>::Payload
where Self: ::method::connection::BlockedMethod<'a>
{
        Default::default()
    }
fn connection_close() -> <Self as ::method::connection::CloseMethod<'a>>::Payload
where Self: ::method::connection::CloseMethod<'a>
{
        Default::default()
    }
fn connection_close_ok() -> <Self as ::method::connection::CloseOkMethod>::Payload
where Self: ::method::connection::CloseOkMethod
{
        Default::default()
    }
fn connection_open() -> <Self as ::method::connection::OpenMethod<'a>>::Payload
where Self: ::method::connection::OpenMethod<'a>
{
        Default::default()
    }
fn connection_open_ok() -> <Self as ::method::connection::OpenOkMethod<'a>>::Payload
where Self: ::method::connection::OpenOkMethod<'a>
{
        Default::default()
    }
fn connection_redirect() -> <Self as ::method::connection::RedirectMethod<'a>>::Payload
where Self: ::method::connection::RedirectMethod<'a>
{
        Default::default()
    }
fn connection_secure() -> <Self as ::method::connection::SecureMethod<'a>>::Payload
where Self: ::method::connection::SecureMethod<'a>
{
        Default::default()
    }
fn connection_secure_ok() -> <Self as ::method::connection::SecureOkMethod<'a>>::Payload
where Self: ::method::connection::SecureOkMethod<'a>
{
        Default::default()
    }
fn connection_start() -> <Self as ::method::connection::StartMethod<'a>>::Payload
where Self: ::method::connection::StartMethod<'a>
{
        Default::default()
    }
fn connection_start_ok() -> <Self as ::method::connection::StartOkMethod<'a>>::Payload
where Self: ::method::connection::StartOkMethod<'a>
{
        Default::default()
    }
fn connection_tune() -> <Self as ::method::connection::TuneMethod>::Payload
where Self: ::method::connection::TuneMethod
{
        Default::default()
    }
fn connection_tune_ok() -> <Self as ::method::connection::TuneOkMethod>::Payload
where Self: ::method::connection::TuneOkMethod
{
        Default::default()
    }
fn connection_unblocked() -> <Self as ::method::connection::UnblockedMethod>::Payload
where Self: ::method::connection::UnblockedMethod
{
        Default::default()
    }
    fn dtx_select() -> <Self as ::method::dtx::SelectMethod>::Payload
        where Self: ::method::dtx::SelectMethod
    {
        Default::default()
    }
    fn dtx_select_ok() -> <Self as ::method::dtx::SelectOkMethod>::Payload
        where Self: ::method::dtx::SelectOkMethod
    {
        Default::default()
    }
    fn dtx_start() -> <Self as ::method::dtx::StartMethod<'a>>::Payload
        where Self: ::method::dtx::StartMethod<'a>
    {
        Default::default()
    }
    fn dtx_start_ok() -> <Self as ::method::dtx::StartOkMethod>::Payload
        where Self: ::method::dtx::StartOkMethod
    {
        Default::default()
    }
fn exchange_bind() -> <Self as ::method::exchange::BindMethod<'a>>::Payload
where Self: ::method::exchange::BindMethod<'a>
{
        Default::default()
    }
fn exchange_bind_ok() -> <Self as ::method::exchange::BindOkMethod>::Payload
where Self: ::method::exchange::BindOkMethod
{
        Default::default()
    }
fn exchange_bound() -> <Self as ::method::exchange::BoundMethod<'a>>::Payload
where Self: ::method::exchange::BoundMethod<'a>
{
        Default::default()
    }
fn exchange_bound_ok() -> <Self as ::method::exchange::BoundOkMethod<'a>>::Payload
where Self: ::method::exchange::BoundOkMethod<'a>
{
        Default::default()
    }
fn exchange_declare() -> <Self as ::method::exchange::DeclareMethod<'a>>::Payload
where Self: ::method::exchange::DeclareMethod<'a>
{
        Default::default()
    }
fn exchange_declare_ok() -> <Self as ::method::exchange::DeclareOkMethod>::Payload
where Self: ::method::exchange::DeclareOkMethod
{
        Default::default()
    }
fn exchange_delete() -> <Self as ::method::exchange::DeleteMethod<'a>>::Payload
where Self: ::method::exchange::DeleteMethod<'a>
{
        Default::default()
    }
fn exchange_delete_ok() -> <Self as ::method::exchange::DeleteOkMethod>::Payload
where Self: ::method::exchange::DeleteOkMethod
{
        Default::default()
    }
fn exchange_unbind() -> <Self as ::method::exchange::UnbindMethod<'a>>::Payload
where Self: ::method::exchange::UnbindMethod<'a>
{
        Default::default()
    }
fn exchange_unbind_ok() -> <Self as ::method::exchange::UnbindOkMethod>::Payload
where Self: ::method::exchange::UnbindOkMethod
{
        Default::default()
    }
    fn file_ack() -> <Self as ::method::file::AckMethod>::Payload
        where Self: ::method::file::AckMethod
    {
        Default::default()
    }
fn file_cancel() -> <Self as ::method::file::CancelMethod<'a>>::Payload
where Self: ::method::file::CancelMethod<'a>
{
        Default::default()
    }
fn file_cancel_ok() -> <Self as ::method::file::CancelOkMethod<'a>>::Payload
where Self: ::method::file::CancelOkMethod<'a>
{
        Default::default()
    }
fn file_consume() -> <Self as ::method::file::ConsumeMethod<'a>>::Payload
where Self: ::method::file::ConsumeMethod<'a>
{
        Default::default()
    }
fn file_consume_ok() -> <Self as ::method::file::ConsumeOkMethod<'a>>::Payload
where Self: ::method::file::ConsumeOkMethod<'a>
{
        Default::default()
    }
fn file_deliver() -> <Self as ::method::file::DeliverMethod<'a>>::Payload
where Self: ::method::file::DeliverMethod<'a>
{
        Default::default()
    }
    fn file_open() -> <Self as ::method::file::OpenMethod<'a>>::Payload
        where Self: ::method::file::OpenMethod<'a>
    {
        Default::default()
    }
    fn file_open_ok() -> <Self as ::method::file::OpenOkMethod>::Payload
        where Self: ::method::file::OpenOkMethod
    {
        Default::default()
    }
fn file_publish() -> <Self as ::method::file::PublishMethod<'a>>::Payload
where Self: ::method::file::PublishMethod<'a>
{
        Default::default()
    }
    fn file_qos() -> <Self as ::method::file::QosMethod>::Payload
        where Self: ::method::file::QosMethod
    {
        Default::default()
    }
    fn file_qos_ok() -> <Self as ::method::file::QosOkMethod>::Payload
        where Self: ::method::file::QosOkMethod
    {
        Default::default()
    }
    fn file_reject() -> <Self as ::method::file::RejectMethod>::Payload
        where Self: ::method::file::RejectMethod
    {
        Default::default()
    }
fn file_return() -> <Self as ::method::file::ReturnMethod<'a>>::Payload
where Self: ::method::file::ReturnMethod<'a>
{
        Default::default()
    }
    fn file_stage() -> <Self as ::method::file::StageMethod<'a>>::Payload
        where Self: ::method::file::StageMethod<'a>
    {
        Default::default()
    }
fn message_append() -> <Self as ::method::message::AppendMethod<'a>>::Payload
where Self: ::method::message::AppendMethod<'a>
{
        Default::default()
    }
fn message_cancel() -> <Self as ::method::message::CancelMethod<'a>>::Payload
where Self: ::method::message::CancelMethod<'a>
{
        Default::default()
    }
fn message_checkpoint() -> <Self as ::method::message::CheckpointMethod<'a>>::Payload
where Self: ::method::message::CheckpointMethod<'a>
{
        Default::default()
    }
fn message_close() -> <Self as ::method::message::CloseMethod<'a>>::Payload
where Self: ::method::message::CloseMethod<'a>
{
        Default::default()
    }
fn message_consume() -> <Self as ::method::message::ConsumeMethod<'a>>::Payload
where Self: ::method::message::ConsumeMethod<'a>
{
        Default::default()
    }
    fn message_empty() -> <Self as ::method::message::EmptyMethod>::Payload
        where Self: ::method::message::EmptyMethod
    {
        Default::default()
    }
fn message_get() -> <Self as ::method::message::GetMethod<'a>>::Payload
where Self: ::method::message::GetMethod<'a>
{
        Default::default()
    }
    fn message_offset() -> <Self as ::method::message::OffsetMethod>::Payload
        where Self: ::method::message::OffsetMethod
    {
        Default::default()
    }
    fn message_ok() -> <Self as ::method::message::OkMethod>::Payload
        where Self: ::method::message::OkMethod
    {
        Default::default()
    }
fn message_open() -> <Self as ::method::message::OpenMethod<'a>>::Payload
where Self: ::method::message::OpenMethod<'a>
{
        Default::default()
    }
    fn message_qos() -> <Self as ::method::message::QosMethod>::Payload
        where Self: ::method::message::QosMethod
    {
        Default::default()
    }
fn message_recover() -> <Self as ::method::message::RecoverMethod>::Payload
where Self: ::method::message::RecoverMethod
{
        Default::default()
    }
fn message_reject() -> <Self as ::method::message::RejectMethod<'a>>::Payload
where Self: ::method::message::RejectMethod<'a>
{
        Default::default()
    }
fn message_resume() -> <Self as ::method::message::ResumeMethod<'a>>::Payload
where Self: ::method::message::ResumeMethod<'a>
{
        Default::default()
    }
fn message_transfer() -> <Self as ::method::message::TransferMethod<'a>>::Payload
where Self: ::method::message::TransferMethod<'a>
{
        Default::default()
    }
    fn queue_bind() -> <Self as ::method::queue::BindMethod<'a>>::Payload
        where Self: ::method::queue::BindMethod<'a>
    {
        Default::default()
    }
    fn queue_bind_ok() -> <Self as ::method::queue::BindOkMethod>::Payload
        where Self: ::method::queue::BindOkMethod
    {
        Default::default()
    }
fn queue_declare() -> <Self as ::method::queue::DeclareMethod<'a>>::Payload
where Self: ::method::queue::DeclareMethod<'a>
{
        Default::default()
    }
fn queue_declare_ok() -> <Self as ::method::queue::DeclareOkMethod<'a>>::Payload
where Self: ::method::queue::DeclareOkMethod<'a>
{
        Default::default()
    }
fn queue_delete() -> <Self as ::method::queue::DeleteMethod<'a>>::Payload
where Self: ::method::queue::DeleteMethod<'a>
{
        Default::default()
    }
    fn queue_delete_ok() -> <Self as ::method::queue::DeleteOkMethod>::Payload
        where Self: ::method::queue::DeleteOkMethod
    {
        Default::default()
    }
fn queue_purge() -> <Self as ::method::queue::PurgeMethod<'a>>::Payload
where Self: ::method::queue::PurgeMethod<'a>
{
        Default::default()
    }
    fn queue_purge_ok() -> <Self as ::method::queue::PurgeOkMethod>::Payload
        where Self: ::method::queue::PurgeOkMethod
    {
        Default::default()
    }
fn queue_unbind() -> <Self as ::method::queue::UnbindMethod<'a>>::Payload
where Self: ::method::queue::UnbindMethod<'a>
{
        Default::default()
    }
    fn queue_unbind_ok() -> <Self as ::method::queue::UnbindOkMethod>::Payload
        where Self: ::method::queue::UnbindOkMethod
    {
        Default::default()
    }
fn stream_cancel() -> <Self as ::method::stream::CancelMethod<'a>>::Payload
where Self: ::method::stream::CancelMethod<'a>
{
        Default::default()
    }
fn stream_cancel_ok() -> <Self as ::method::stream::CancelOkMethod<'a>>::Payload
where Self: ::method::stream::CancelOkMethod<'a>
{
        Default::default()
    }
fn stream_consume() -> <Self as ::method::stream::ConsumeMethod<'a>>::Payload
where Self: ::method::stream::ConsumeMethod<'a>
{
        Default::default()
    }
fn stream_consume_ok() -> <Self as ::method::stream::ConsumeOkMethod<'a>>::Payload
where Self: ::method::stream::ConsumeOkMethod<'a>
{
        Default::default()
    }
fn stream_deliver() -> <Self as ::method::stream::DeliverMethod<'a>>::Payload
where Self: ::method::stream::DeliverMethod<'a>
{
        Default::default()
    }
fn stream_publish() -> <Self as ::method::stream::PublishMethod<'a>>::Payload
where Self: ::method::stream::PublishMethod<'a>
{
        Default::default()
    }
    fn stream_qos() -> <Self as ::method::stream::QosMethod>::Payload
        where Self: ::method::stream::QosMethod
    {
        Default::default()
    }
    fn stream_qos_ok() -> <Self as ::method::stream::QosOkMethod>::Payload
        where Self: ::method::stream::QosOkMethod
    {
        Default::default()
    }
    fn stream_return() -> <Self as ::method::stream::ReturnMethod<'a>>::Payload
        where Self: ::method::stream::ReturnMethod<'a>
    {
        Default::default()
    }
    fn test_content() -> <Self as ::method::test::ContentMethod<'a>>::Payload
        where Self: ::method::test::ContentMethod<'a>
    {
        Default::default()
    }
    fn test_content_ok() -> <Self as ::method::test::ContentOkMethod<'a>>::Payload
        where Self: ::method::test::ContentOkMethod<'a>
    {
        Default::default()
    }
    fn test_integer() -> <Self as ::method::test::IntegerMethod>::Payload
        where Self: ::method::test::IntegerMethod
    {
        Default::default()
    }
    fn test_integer_ok() -> <Self as ::method::test::IntegerOkMethod>::Payload
        where Self: ::method::test::IntegerOkMethod
    {
        Default::default()
    }
fn test_string() -> <Self as ::method::test::StringMethod<'a>>::Payload
where Self: ::method::test::StringMethod<'a>
{
        Default::default()
    }
fn test_string_ok() -> <Self as ::method::test::StringOkMethod<'a>>::Payload
where Self: ::method::test::StringOkMethod<'a>
{
        Default::default()
    }
    fn test_table() -> <Self as ::method::test::TableMethod<'a>>::Payload
        where Self: ::method::test::TableMethod<'a>
    {
        Default::default()
    }
fn test_table_ok() -> <Self as ::method::test::TableOkMethod<'a>>::Payload
where Self: ::method::test::TableOkMethod<'a>
{
        Default::default()
    }
fn tunnel_request() -> <Self as ::method::tunnel::RequestMethod<'a>>::Payload
where Self: ::method::tunnel::RequestMethod<'a>
{
        Default::default()
    }
    fn tx_commit() -> <Self as ::method::tx::CommitMethod>::Payload
        where Self: ::method::tx::CommitMethod
    {
        Default::default()
    }
    fn tx_commit_ok() -> <Self as ::method::tx::CommitOkMethod>::Payload
        where Self: ::method::tx::CommitOkMethod
    {
        Default::default()
    }
    fn tx_rollback() -> <Self as ::method::tx::RollbackMethod>::Payload
        where Self: ::method::tx::RollbackMethod
    {
        Default::default()
    }
    fn tx_rollback_ok() -> <Self as ::method::tx::RollbackOkMethod>::Payload
        where Self: ::method::tx::RollbackOkMethod
    {
        Default::default()
    }
    fn tx_select() -> <Self as ::method::tx::SelectMethod>::Payload
        where Self: ::method::tx::SelectMethod
    {
        Default::default()
    }
    fn tx_select_ok() -> <Self as ::method::tx::SelectOkMethod>::Payload
        where Self: ::method::tx::SelectOkMethod
    {
        Default::default()
    }
} // pub trait Protocol<'a>
pub enum Class {
    Access,
    Basic,
    Channel,
    Confirm,
    Connection,
    Dtx,
    Exchange,
    File,
    Message,
    Queue,
    Stream,
    Test,
    Tunnel,
    Tx,
} // pub trait Class

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub struct Amqp9_1;
impl<'a> ::Protocol<'a> for Amqp9_1 {
    type Frame = amqp9_1::Frame<'a>;
    fn protocol_header() -> &'static [u8] {
        b"AMQP\x00\x00\x09\x01"
    } // fn protocol_header()
} // impl ::Protocol<'a> for Amqp9_1

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub struct Amqp9_0;
impl<'a> ::Protocol<'a> for Amqp9_0 {
    type Frame = amqp9_0::Frame<'a>;
    fn protocol_header() -> &'static [u8] {
        b"AMQP\x00\x00\x09\x00"
    } // fn protocol_header()
} // impl ::Protocol<'a> for Amqp9_0

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub struct Amqp8_0;
impl<'a> ::Protocol<'a> for Amqp8_0 {
    type Frame = amqp8_0::Frame<'a>;
    fn protocol_header() -> &'static [u8] {
        b"AMQP\x00\x00\x08\x00"
    } // fn protocol_header()
} // impl ::Protocol<'a> for Amqp8_0

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub struct Rabbitmq9_1;
impl<'a> ::Protocol<'a> for Rabbitmq9_1 {
    type Frame = rabbitmq9_1::Frame<'a>;
    fn protocol_header() -> &'static [u8] {
        b"AMQP\x00\x00\x09\x01"
    } // fn protocol_header()
} // impl ::Protocol<'a> for Rabbitmq9_1

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub struct Qpid9_0;
impl<'a> ::Protocol<'a> for Qpid9_0 {
    type Frame = qpid9_0::Frame<'a>;
    fn protocol_header() -> &'static [u8] {
        b"AMQP\x00\x00\x09\x00"
    } // fn protocol_header()
} // impl ::Protocol<'a> for Qpid9_0

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub struct Qpid8_0;
impl<'a> ::Protocol<'a> for Qpid8_0 {
    type Frame = qpid8_0::Frame<'a>;
    fn protocol_header() -> &'static [u8] {
        b"AMQP\x00\x00\x08\x00"
    } // fn protocol_header()
} // impl ::Protocol<'a> for Qpid8_0
