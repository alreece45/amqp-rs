// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[cfg(all(feature = "amqp", feature = "0.9.1"))]
pub mod amqp0_9_1 {
    include!(concat!(env!("OUT_DIR"), "/amqp-0.9.1.rs"));
}

#[cfg(all(feature = "amqp", feature = "0.9"))]
pub mod amqp0_9_0 {
    include!(concat!(env!("OUT_DIR"), "/amqp-0.9.0.rs"));
}

#[cfg(all(feature = "amqp", feature = "0.8"))]
pub mod amqp0_8_0 {
    include!(concat!(env!("OUT_DIR"), "/amqp-0.8.0.rs"));
}

#[cfg(all(feature = "rabbitmq", feature = "0.9.1"))]
pub mod rabbitmq0_9_1 {
    include!(concat!(env!("OUT_DIR"), "/rabbitmq-0.9.1.rs"));
}

#[cfg(all(feature = "qpid", feature = "0.9"))]
pub mod qpid0_10_0 {
    include!(concat!(env!("OUT_DIR"), "/qpid-0.9.rs"));
}

#[cfg(all(feature = "qpid", feature = "0.8"))]
pub mod qpid0_8_0 {
    include!(concat!(env!("OUT_DIR"), "/qpid-0.8.rs"));
}