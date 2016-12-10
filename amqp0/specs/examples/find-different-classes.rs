
extern crate amqp0_specs as specs;

use std::collections::HashMap;
use amqp0::ClassMethod;
use specs::amqp0;

fn main() {
    let specs = amqp0::specs();

    // assert_name_indexes_consistent(&specs);
    println!("{:#?}", specs);

    let (vanillas, e) = amqp0::specs().into_iter()
        .partition::<Vec<_>, _>(|s| s.name() == "amqp");

    let extended = e.iter()
        .map(|s| (s.version(), s))
        .collect::<HashMap<_, _>>();

    let mut defined_methods = HashMap::<&str, &ClassMethod>::new();
}

