
extern crate amqp_client0 as amqp;

use amqp::ConfigBuilder;

fn main() {
    let mut session = ConfigBuilder::without_tls()
        .port(5672)
        .plain_auth("", "")
        .channel_max(10)
        .virtual_host("/")
        .into_blocking();

    print!("{:#?}", session);
    {
        let start1 = session.connect().unwrap();
        print!("Start1: {:?}", start1);
    }

    let start2 = session.connect().unwrap();
    print!("Start2: {:?}", start2);
}