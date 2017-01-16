/// Generated by build script in amqp0-specs
/// Pregenerated files are used by default.
///
/// Build using amqp0-pregen crate using: cargo --features="amqp0-build-specs"
/// Regenerate pregenerated scripts using: cargo --features="amqp0-pregen-specs"
///
/// EDITORS BEWARE: Your modifications may be overridden

pub static AMQP9_1: ::Spec = 
::Spec {
name: "amqp",
classes: &::phf::OrderedMap {
    key: 1897749892740154578,
    disps: ::phf::Slice::Static(&[
        (1, 0),
        (2, 3),
    ]),
    idxs: ::phf::Slice::Static(&[
        2,
        1,
        3,
        4,
        5,
        0,
    ]),
    entries: ::phf::Slice::Static(&[
        ("basic", ::Class {
name: "basic",
fields: &[::ClassField {
name: "content-type",
domain: "shortstr"
},
::ClassField {
name: "content-encoding",
domain: "shortstr"
},
::ClassField {
name: "headers",
domain: "table"
},
::ClassField {
name: "delivery-mode",
domain: "octet"
},
::ClassField {
name: "priority",
domain: "octet"
},
::ClassField {
name: "correlation-id",
domain: "shortstr"
},
::ClassField {
name: "reply-to",
domain: "shortstr"
},
::ClassField {
name: "expiration",
domain: "shortstr"
},
::ClassField {
name: "message-id",
domain: "shortstr"
},
::ClassField {
name: "timestamp",
domain: "timestamp"
},
::ClassField {
name: "type",
domain: "shortstr"
},
::ClassField {
name: "user-id",
domain: "shortstr"
},
::ClassField {
name: "app-id",
domain: "shortstr"
},
::ClassField {
name: "reserved",
domain: "shortstr"
},

],
index: 60,
methods: &[::ClassMethod {
name: "qos",
index: 10,
chassis_client: None,
chassis_server: Some("MUST"),
response: Some("qos-ok"),
is_synchronous: true,
fields: &[::ClassMethodField {
name: "prefetch-size",
domain: "long",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "prefetch-count",
domain: "short",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "global",
domain: "bit",
assertions: &[],
is_reserved: false
},

],
},
::ClassMethod {
name: "qos-ok",
index: 11,
chassis_client: Some("MUST"),
chassis_server: None,
response: None,
is_synchronous: true,
fields: &[],
},
::ClassMethod {
name: "consume",
index: 20,
chassis_client: None,
chassis_server: Some("MUST"),
response: Some("consume-ok"),
is_synchronous: true,
fields: &[::ClassMethodField {
name: "reserved-1",
domain: "short",
assertions: &[],
is_reserved: true
},
::ClassMethodField {
name: "queue",
domain: "queue-name",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "consumer-tag",
domain: "consumer-tag",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "no-local",
domain: "no-local",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "no-ack",
domain: "no-ack",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "exclusive",
domain: "bit",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "no-wait",
domain: "no-wait",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "arguments",
domain: "table",
assertions: &[],
is_reserved: false
},

],
},
::ClassMethod {
name: "consume-ok",
index: 21,
chassis_client: Some("MUST"),
chassis_server: None,
response: None,
is_synchronous: true,
fields: &[::ClassMethodField {
name: "consumer-tag",
domain: "consumer-tag",
assertions: &[],
is_reserved: false
},

],
},
::ClassMethod {
name: "cancel",
index: 30,
chassis_client: None,
chassis_server: Some("MUST"),
response: Some("cancel-ok"),
is_synchronous: true,
fields: &[::ClassMethodField {
name: "consumer-tag",
domain: "consumer-tag",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "no-wait",
domain: "no-wait",
assertions: &[],
is_reserved: false
},

],
},
::ClassMethod {
name: "cancel-ok",
index: 31,
chassis_client: Some("MUST"),
chassis_server: None,
response: None,
is_synchronous: true,
fields: &[::ClassMethodField {
name: "consumer-tag",
domain: "consumer-tag",
assertions: &[],
is_reserved: false
},

],
},
::ClassMethod {
name: "publish",
index: 40,
chassis_client: None,
chassis_server: Some("MUST"),
response: None,
is_synchronous: false,
fields: &[::ClassMethodField {
name: "reserved-1",
domain: "short",
assertions: &[],
is_reserved: true
},
::ClassMethodField {
name: "exchange",
domain: "exchange-name",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "routing-key",
domain: "shortstr",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "mandatory",
domain: "bit",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "immediate",
domain: "bit",
assertions: &[],
is_reserved: false
},

],
},
::ClassMethod {
name: "return",
index: 50,
chassis_client: Some("MUST"),
chassis_server: None,
response: None,
is_synchronous: false,
fields: &[::ClassMethodField {
name: "reply-code",
domain: "reply-code",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "reply-text",
domain: "reply-text",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "exchange",
domain: "exchange-name",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "routing-key",
domain: "shortstr",
assertions: &[],
is_reserved: false
},

],
},
::ClassMethod {
name: "deliver",
index: 60,
chassis_client: Some("MUST"),
chassis_server: None,
response: None,
is_synchronous: false,
fields: &[::ClassMethodField {
name: "consumer-tag",
domain: "consumer-tag",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "delivery-tag",
domain: "delivery-tag",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "redelivered",
domain: "redelivered",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "exchange",
domain: "exchange-name",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "routing-key",
domain: "shortstr",
assertions: &[],
is_reserved: false
},

],
},
::ClassMethod {
name: "get",
index: 70,
chassis_client: None,
chassis_server: Some("MUST"),
response: Some("get-empty"),
is_synchronous: true,
fields: &[::ClassMethodField {
name: "reserved-1",
domain: "short",
assertions: &[],
is_reserved: true
},
::ClassMethodField {
name: "queue",
domain: "queue-name",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "no-ack",
domain: "no-ack",
assertions: &[],
is_reserved: false
},

],
},
::ClassMethod {
name: "get-ok",
index: 71,
chassis_client: Some("MAY"),
chassis_server: None,
response: None,
is_synchronous: true,
fields: &[::ClassMethodField {
name: "delivery-tag",
domain: "delivery-tag",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "redelivered",
domain: "redelivered",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "exchange",
domain: "exchange-name",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "routing-key",
domain: "shortstr",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "message-count",
domain: "message-count",
assertions: &[],
is_reserved: false
},

],
},
::ClassMethod {
name: "get-empty",
index: 72,
chassis_client: Some("MAY"),
chassis_server: None,
response: None,
is_synchronous: true,
fields: &[::ClassMethodField {
name: "reserved-1",
domain: "shortstr",
assertions: &[],
is_reserved: true
},

],
},
::ClassMethod {
name: "ack",
index: 80,
chassis_client: None,
chassis_server: Some("MUST"),
response: None,
is_synchronous: false,
fields: &[::ClassMethodField {
name: "delivery-tag",
domain: "delivery-tag",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "multiple",
domain: "bit",
assertions: &[],
is_reserved: false
},

],
},
::ClassMethod {
name: "reject",
index: 90,
chassis_client: None,
chassis_server: Some("MUST"),
response: None,
is_synchronous: false,
fields: &[::ClassMethodField {
name: "delivery-tag",
domain: "delivery-tag",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "requeue",
domain: "bit",
assertions: &[],
is_reserved: false
},

],
},
::ClassMethod {
name: "recover-async",
index: 100,
chassis_client: None,
chassis_server: Some("MAY"),
response: None,
is_synchronous: false,
fields: &[::ClassMethodField {
name: "requeue",
domain: "bit",
assertions: &[],
is_reserved: false
},

],
},
::ClassMethod {
name: "recover",
index: 110,
chassis_client: None,
chassis_server: Some("MUST"),
response: None,
is_synchronous: false,
fields: &[::ClassMethodField {
name: "requeue",
domain: "bit",
assertions: &[],
is_reserved: false
},

],
},
::ClassMethod {
name: "recover-ok",
index: 111,
chassis_client: Some("MUST"),
chassis_server: None,
response: None,
is_synchronous: true,
fields: &[],
},

]
}),
        ("channel", ::Class {
name: "channel",
fields: &[],
index: 20,
methods: &[::ClassMethod {
name: "open",
index: 10,
chassis_client: None,
chassis_server: Some("MUST"),
response: Some("open-ok"),
is_synchronous: true,
fields: &[::ClassMethodField {
name: "reserved-1",
domain: "shortstr",
assertions: &[],
is_reserved: true
},

],
},
::ClassMethod {
name: "open-ok",
index: 11,
chassis_client: Some("MUST"),
chassis_server: None,
response: None,
is_synchronous: true,
fields: &[::ClassMethodField {
name: "reserved-1",
domain: "longstr",
assertions: &[],
is_reserved: true
},

],
},
::ClassMethod {
name: "flow",
index: 20,
chassis_client: Some("MUST"),
chassis_server: Some("MUST"),
response: Some("flow-ok"),
is_synchronous: true,
fields: &[::ClassMethodField {
name: "active",
domain: "bit",
assertions: &[],
is_reserved: false
},

],
},
::ClassMethod {
name: "flow-ok",
index: 21,
chassis_client: Some("MUST"),
chassis_server: Some("MUST"),
response: None,
is_synchronous: false,
fields: &[::ClassMethodField {
name: "active",
domain: "bit",
assertions: &[],
is_reserved: false
},

],
},
::ClassMethod {
name: "close",
index: 40,
chassis_client: Some("MUST"),
chassis_server: Some("MUST"),
response: Some("close-ok"),
is_synchronous: true,
fields: &[::ClassMethodField {
name: "reply-code",
domain: "reply-code",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "reply-text",
domain: "reply-text",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "class-id",
domain: "class-id",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "method-id",
domain: "method-id",
assertions: &[],
is_reserved: false
},

],
},
::ClassMethod {
name: "close-ok",
index: 41,
chassis_client: Some("MUST"),
chassis_server: Some("MUST"),
response: None,
is_synchronous: true,
fields: &[],
},

]
}),
        ("connection", ::Class {
name: "connection",
fields: &[],
index: 10,
methods: &[::ClassMethod {
name: "start",
index: 10,
chassis_client: Some("MUST"),
chassis_server: None,
response: Some("start-ok"),
is_synchronous: true,
fields: &[::ClassMethodField {
name: "version-major",
domain: "octet",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "version-minor",
domain: "octet",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "server-properties",
domain: "peer-properties",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "mechanisms",
domain: "longstr",
assertions: &[::ClassMethodFieldAssertion::NotNull,

],
is_reserved: false
},
::ClassMethodField {
name: "locales",
domain: "longstr",
assertions: &[::ClassMethodFieldAssertion::NotNull,

],
is_reserved: false
},

],
},
::ClassMethod {
name: "start-ok",
index: 11,
chassis_client: None,
chassis_server: Some("MUST"),
response: None,
is_synchronous: true,
fields: &[::ClassMethodField {
name: "client-properties",
domain: "peer-properties",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "mechanism",
domain: "shortstr",
assertions: &[::ClassMethodFieldAssertion::NotNull,

],
is_reserved: false
},
::ClassMethodField {
name: "response",
domain: "longstr",
assertions: &[::ClassMethodFieldAssertion::NotNull,

],
is_reserved: false
},
::ClassMethodField {
name: "locale",
domain: "shortstr",
assertions: &[::ClassMethodFieldAssertion::NotNull,

],
is_reserved: false
},

],
},
::ClassMethod {
name: "secure",
index: 20,
chassis_client: Some("MUST"),
chassis_server: None,
response: Some("secure-ok"),
is_synchronous: true,
fields: &[::ClassMethodField {
name: "challenge",
domain: "longstr",
assertions: &[],
is_reserved: false
},

],
},
::ClassMethod {
name: "secure-ok",
index: 21,
chassis_client: None,
chassis_server: Some("MUST"),
response: None,
is_synchronous: true,
fields: &[::ClassMethodField {
name: "response",
domain: "longstr",
assertions: &[::ClassMethodFieldAssertion::NotNull,

],
is_reserved: false
},

],
},
::ClassMethod {
name: "tune",
index: 30,
chassis_client: Some("MUST"),
chassis_server: None,
response: Some("tune-ok"),
is_synchronous: true,
fields: &[::ClassMethodField {
name: "channel-max",
domain: "short",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "frame-max",
domain: "long",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "heartbeat",
domain: "short",
assertions: &[],
is_reserved: false
},

],
},
::ClassMethod {
name: "tune-ok",
index: 31,
chassis_client: None,
chassis_server: Some("MUST"),
response: None,
is_synchronous: true,
fields: &[::ClassMethodField {
name: "channel-max",
domain: "short",
assertions: &[::ClassMethodFieldAssertion::NotNull,
::ClassMethodFieldAssertion::ChannelMax,

],
is_reserved: false
},
::ClassMethodField {
name: "frame-max",
domain: "long",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "heartbeat",
domain: "short",
assertions: &[],
is_reserved: false
},

],
},
::ClassMethod {
name: "open",
index: 40,
chassis_client: None,
chassis_server: Some("MUST"),
response: Some("open-ok"),
is_synchronous: true,
fields: &[::ClassMethodField {
name: "virtual-host",
domain: "path",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "reserved-1",
domain: "shortstr",
assertions: &[],
is_reserved: true
},
::ClassMethodField {
name: "reserved-2",
domain: "bit",
assertions: &[],
is_reserved: true
},

],
},
::ClassMethod {
name: "open-ok",
index: 41,
chassis_client: Some("MUST"),
chassis_server: None,
response: None,
is_synchronous: true,
fields: &[::ClassMethodField {
name: "reserved-1",
domain: "shortstr",
assertions: &[],
is_reserved: true
},

],
},
::ClassMethod {
name: "close",
index: 50,
chassis_client: Some("MUST"),
chassis_server: Some("MUST"),
response: Some("close-ok"),
is_synchronous: true,
fields: &[::ClassMethodField {
name: "reply-code",
domain: "reply-code",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "reply-text",
domain: "reply-text",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "class-id",
domain: "class-id",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "method-id",
domain: "method-id",
assertions: &[],
is_reserved: false
},

],
},
::ClassMethod {
name: "close-ok",
index: 51,
chassis_client: Some("MUST"),
chassis_server: Some("MUST"),
response: None,
is_synchronous: true,
fields: &[],
},

]
}),
        ("exchange", ::Class {
name: "exchange",
fields: &[],
index: 40,
methods: &[::ClassMethod {
name: "declare",
index: 10,
chassis_client: None,
chassis_server: Some("MUST"),
response: Some("declare-ok"),
is_synchronous: true,
fields: &[::ClassMethodField {
name: "reserved-1",
domain: "short",
assertions: &[],
is_reserved: true
},
::ClassMethodField {
name: "exchange",
domain: "exchange-name",
assertions: &[::ClassMethodFieldAssertion::NotNull,

],
is_reserved: false
},
::ClassMethodField {
name: "type",
domain: "shortstr",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "passive",
domain: "bit",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "durable",
domain: "bit",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "reserved-2",
domain: "bit",
assertions: &[],
is_reserved: true
},
::ClassMethodField {
name: "reserved-3",
domain: "bit",
assertions: &[],
is_reserved: true
},
::ClassMethodField {
name: "no-wait",
domain: "no-wait",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "arguments",
domain: "table",
assertions: &[],
is_reserved: false
},

],
},
::ClassMethod {
name: "declare-ok",
index: 11,
chassis_client: Some("MUST"),
chassis_server: None,
response: None,
is_synchronous: true,
fields: &[],
},
::ClassMethod {
name: "delete",
index: 20,
chassis_client: None,
chassis_server: Some("MUST"),
response: Some("delete-ok"),
is_synchronous: true,
fields: &[::ClassMethodField {
name: "reserved-1",
domain: "short",
assertions: &[],
is_reserved: true
},
::ClassMethodField {
name: "exchange",
domain: "exchange-name",
assertions: &[::ClassMethodFieldAssertion::NotNull,

],
is_reserved: false
},
::ClassMethodField {
name: "if-unused",
domain: "bit",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "no-wait",
domain: "no-wait",
assertions: &[],
is_reserved: false
},

],
},
::ClassMethod {
name: "delete-ok",
index: 21,
chassis_client: Some("MUST"),
chassis_server: None,
response: None,
is_synchronous: true,
fields: &[],
},

]
}),
        ("queue", ::Class {
name: "queue",
fields: &[],
index: 50,
methods: &[::ClassMethod {
name: "declare",
index: 10,
chassis_client: None,
chassis_server: Some("MUST"),
response: Some("declare-ok"),
is_synchronous: true,
fields: &[::ClassMethodField {
name: "reserved-1",
domain: "short",
assertions: &[],
is_reserved: true
},
::ClassMethodField {
name: "queue",
domain: "queue-name",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "passive",
domain: "bit",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "durable",
domain: "bit",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "exclusive",
domain: "bit",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "auto-delete",
domain: "bit",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "no-wait",
domain: "no-wait",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "arguments",
domain: "table",
assertions: &[],
is_reserved: false
},

],
},
::ClassMethod {
name: "declare-ok",
index: 11,
chassis_client: Some("MUST"),
chassis_server: None,
response: None,
is_synchronous: true,
fields: &[::ClassMethodField {
name: "queue",
domain: "queue-name",
assertions: &[::ClassMethodFieldAssertion::NotNull,

],
is_reserved: false
},
::ClassMethodField {
name: "message-count",
domain: "message-count",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "consumer-count",
domain: "long",
assertions: &[],
is_reserved: false
},

],
},
::ClassMethod {
name: "bind",
index: 20,
chassis_client: None,
chassis_server: Some("MUST"),
response: Some("bind-ok"),
is_synchronous: true,
fields: &[::ClassMethodField {
name: "reserved-1",
domain: "short",
assertions: &[],
is_reserved: true
},
::ClassMethodField {
name: "queue",
domain: "queue-name",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "exchange",
domain: "exchange-name",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "routing-key",
domain: "shortstr",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "no-wait",
domain: "no-wait",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "arguments",
domain: "table",
assertions: &[],
is_reserved: false
},

],
},
::ClassMethod {
name: "bind-ok",
index: 21,
chassis_client: Some("MUST"),
chassis_server: None,
response: None,
is_synchronous: true,
fields: &[],
},
::ClassMethod {
name: "unbind",
index: 50,
chassis_client: None,
chassis_server: Some("MUST"),
response: Some("unbind-ok"),
is_synchronous: true,
fields: &[::ClassMethodField {
name: "reserved-1",
domain: "short",
assertions: &[],
is_reserved: true
},
::ClassMethodField {
name: "queue",
domain: "queue-name",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "exchange",
domain: "exchange-name",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "routing-key",
domain: "shortstr",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "arguments",
domain: "table",
assertions: &[],
is_reserved: false
},

],
},
::ClassMethod {
name: "unbind-ok",
index: 51,
chassis_client: Some("MUST"),
chassis_server: None,
response: None,
is_synchronous: true,
fields: &[],
},
::ClassMethod {
name: "purge",
index: 30,
chassis_client: None,
chassis_server: Some("MUST"),
response: Some("purge-ok"),
is_synchronous: true,
fields: &[::ClassMethodField {
name: "reserved-1",
domain: "short",
assertions: &[],
is_reserved: true
},
::ClassMethodField {
name: "queue",
domain: "queue-name",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "no-wait",
domain: "no-wait",
assertions: &[],
is_reserved: false
},

],
},
::ClassMethod {
name: "purge-ok",
index: 31,
chassis_client: Some("MUST"),
chassis_server: None,
response: None,
is_synchronous: true,
fields: &[::ClassMethodField {
name: "message-count",
domain: "message-count",
assertions: &[],
is_reserved: false
},

],
},
::ClassMethod {
name: "delete",
index: 40,
chassis_client: None,
chassis_server: Some("MUST"),
response: Some("delete-ok"),
is_synchronous: true,
fields: &[::ClassMethodField {
name: "reserved-1",
domain: "short",
assertions: &[],
is_reserved: true
},
::ClassMethodField {
name: "queue",
domain: "queue-name",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "if-unused",
domain: "bit",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "if-empty",
domain: "bit",
assertions: &[],
is_reserved: false
},
::ClassMethodField {
name: "no-wait",
domain: "no-wait",
assertions: &[],
is_reserved: false
},

],
},
::ClassMethod {
name: "delete-ok",
index: 41,
chassis_client: Some("MUST"),
chassis_server: None,
response: None,
is_synchronous: true,
fields: &[::ClassMethodField {
name: "message-count",
domain: "message-count",
assertions: &[],
is_reserved: false
},

],
},

]
}),
        ("tx", ::Class {
name: "tx",
fields: &[],
index: 90,
methods: &[::ClassMethod {
name: "select",
index: 10,
chassis_client: None,
chassis_server: Some("MUST"),
response: Some("select-ok"),
is_synchronous: true,
fields: &[],
},
::ClassMethod {
name: "select-ok",
index: 11,
chassis_client: Some("MUST"),
chassis_server: None,
response: None,
is_synchronous: true,
fields: &[],
},
::ClassMethod {
name: "commit",
index: 20,
chassis_client: None,
chassis_server: Some("MUST"),
response: Some("commit-ok"),
is_synchronous: true,
fields: &[],
},
::ClassMethod {
name: "commit-ok",
index: 21,
chassis_client: Some("MUST"),
chassis_server: None,
response: None,
is_synchronous: true,
fields: &[],
},
::ClassMethod {
name: "rollback",
index: 30,
chassis_client: None,
chassis_server: Some("MUST"),
response: Some("rollback-ok"),
is_synchronous: true,
fields: &[],
},
::ClassMethod {
name: "rollback-ok",
index: 31,
chassis_client: Some("MUST"),
chassis_server: None,
response: None,
is_synchronous: true,
fields: &[],
},

]
}),
    ]),
},
constants: &::phf::OrderedMap {
    key: 8958141709656110593,
    disps: ::phf::Slice::Static(&[
        (0, 0),
    ]),
    idxs: ::phf::Slice::Static(&[
        1,
        0,
    ]),
    entries: ::phf::Slice::Static(&[
        ("frame-end", ::Constant {
name: "frame-end",
value: 206,
class: None
}),
        ("frame-min-size", ::Constant {
name: "frame-min-size",
value: 4096,
class: None
}),
    ]),
},
domains: &::phf::OrderedMap {
    key: 9603444721912725599,
    disps: ::phf::Slice::Static(&[
        (2, 4),
        (2, 19),
        (1, 0),
        (2, 7),
        (4, 10),
    ]),
    idxs: ::phf::Slice::Static(&[
        5,
        20,
        12,
        22,
        17,
        19,
        18,
        6,
        1,
        16,
        0,
        10,
        9,
        21,
        14,
        23,
        15,
        7,
        2,
        4,
        8,
        11,
        3,
        13,
    ]),
    entries: ::phf::Slice::Static(&[
        ("bit", "bit"),
        ("class-id", "short"),
        ("consumer-tag", "shortstr"),
        ("delivery-tag", "longlong"),
        ("exchange-name", "shortstr"),
        ("long", "long"),
        ("longlong", "longlong"),
        ("longstr", "longstr"),
        ("message-count", "long"),
        ("method-id", "short"),
        ("no-ack", "bit"),
        ("no-local", "bit"),
        ("no-wait", "bit"),
        ("octet", "octet"),
        ("path", "shortstr"),
        ("peer-properties", "table"),
        ("queue-name", "shortstr"),
        ("redelivered", "bit"),
        ("reply-code", "short"),
        ("reply-text", "shortstr"),
        ("short", "short"),
        ("shortstr", "shortstr"),
        ("table", "table"),
        ("timestamp", "timestamp"),
    ]),
},
frame_types: &::phf::OrderedMap {
    key: 8958141709656110593,
    disps: ::phf::Slice::Static(&[
        (3, 0),
    ]),
    idxs: ::phf::Slice::Static(&[
        2,
        0,
        1,
        3,
    ]),
    entries: ::phf::Slice::Static(&[
        ("frame-body", ::Constant {
name: "frame-body",
value: 3,
class: None
}),
        ("frame-header", ::Constant {
name: "frame-header",
value: 2,
class: None
}),
        ("frame-heartbeat", ::Constant {
name: "frame-heartbeat",
value: 8,
class: None
}),
        ("frame-method", ::Constant {
name: "frame-method",
value: 1,
class: None
}),
    ]),
},
response_codes: &::phf::OrderedMap {
    key: 5621513170501782519,
    disps: ::phf::Slice::Static(&[
        (8, 5),
        (1, 12),
        (2, 3),
        (1, 0),
    ]),
    idxs: ::phf::Slice::Static(&[
        0,
        6,
        8,
        2,
        12,
        7,
        4,
        14,
        1,
        5,
        16,
        15,
        3,
        9,
        10,
        17,
        13,
        11,
    ]),
    entries: ::phf::Slice::Static(&[
        ("access-refused", ::Constant {
name: "access-refused",
value: 403,
class: Some("soft-error")
}),
        ("channel-error", ::Constant {
name: "channel-error",
value: 504,
class: Some("hard-error")
}),
        ("command-invalid", ::Constant {
name: "command-invalid",
value: 503,
class: Some("hard-error")
}),
        ("connection-forced", ::Constant {
name: "connection-forced",
value: 320,
class: Some("hard-error")
}),
        ("content-too-large", ::Constant {
name: "content-too-large",
value: 311,
class: Some("soft-error")
}),
        ("frame-error", ::Constant {
name: "frame-error",
value: 501,
class: Some("hard-error")
}),
        ("internal-error", ::Constant {
name: "internal-error",
value: 541,
class: Some("hard-error")
}),
        ("invalid-path", ::Constant {
name: "invalid-path",
value: 402,
class: Some("hard-error")
}),
        ("no-consumers", ::Constant {
name: "no-consumers",
value: 313,
class: Some("soft-error")
}),
        ("not-allowed", ::Constant {
name: "not-allowed",
value: 530,
class: Some("hard-error")
}),
        ("not-found", ::Constant {
name: "not-found",
value: 404,
class: Some("soft-error")
}),
        ("not-implemented", ::Constant {
name: "not-implemented",
value: 540,
class: Some("hard-error")
}),
        ("precondition-failed", ::Constant {
name: "precondition-failed",
value: 406,
class: Some("soft-error")
}),
        ("reply-success", ::Constant {
name: "reply-success",
value: 200,
class: None
}),
        ("resource-error", ::Constant {
name: "resource-error",
value: 506,
class: Some("hard-error")
}),
        ("resource-locked", ::Constant {
name: "resource-locked",
value: 405,
class: Some("soft-error")
}),
        ("syntax-error", ::Constant {
name: "syntax-error",
value: 502,
class: Some("hard-error")
}),
        ("unexpected-frame", ::Constant {
name: "unexpected-frame",
value: 505,
class: Some("hard-error")
}),
    ]),
},
version: ::Version { minor: 9, revision: 1 },
}
;
