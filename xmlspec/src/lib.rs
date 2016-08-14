
pub mod amqp0_10_0 {
    // include!(concat!(env!("OUT_DIR"), "/amqp0_10_0/mod.rs"));
}
pub mod amqp0_9_1 {
    include!(concat!(env!("OUT_DIR"), "/amqp-0.9.1.rs"));
}
pub mod amqp0_9_0 {
    include!(concat!(env!("OUT_DIR"), "/amqp-0.9.0.rs"));
}
pub mod amqp0_8_0 {
    include!(concat!(env!("OUT_DIR"), "/amqp-0.8.0.rs"));
}

pub mod rabbitmq0_9_1 {
    include!(concat!(env!("OUT_DIR"), "/rabbitmq-0.9.1.rs"));
}
pub mod qpid0_10_0 {
    include!(concat!(env!("OUT_DIR"), "/qpid-0.9.rs"));
}
pub mod qpid0_8_0 {
    include!(concat!(env!("OUT_DIR"), "/qpid-0.8.rs"));
}