
extern crate amqp0_specs as amqp0;

use std::collections::BTreeMap;

fn main() {
    let mut classes = BTreeMap::new();
    for spec in amqp0::specs().iter() {
        for class in spec.classes().values() {
            let entry = classes.entry(class.name())
                .or_insert((1, class.index()));
            entry.0 += 1;
        }
    }

    let common = classes.into_iter()
        .filter(|&(_, v)| v.0 > 1)
        .map(|(k, v)| (k, v.1))
        .collect::<BTreeMap<_, _>>();

    println!("Common Classes: {:#?}", common);
}