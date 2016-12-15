
extern crate amqp_client0 as amqp;

use amqp::ConfigBuilder;

fn main() {
    let config = ConfigBuilder::without_tls()
        .plain_auth("", "")
        .channel_max(10)
        .virtual_host("/");

    print!("{:#?}", config);
}