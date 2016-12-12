// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::mem;
use nom::{IResult, be_u32};
use primitives::field::{Table, Value};

use common::shortstr;
use pool::ParserPool;
use NomBytes;

type TableEntry<'a> = (&'a str, Value<'a>);

impl<'a> NomBytes<'a> for Table<'a> {
    fn nom_bytes<'b, P>(input: &'a [u8], pool: &'b mut P) -> IResult<&'a [u8], Self>
        where P: ParserPool
    {
        let (input, num_bytes) = try_parse!(input, be_u32);
        let (_, bytes) = try_parse!(input, peek!(length_bytes!(value!(num_bytes))));

        #[ignore(unused_variables)]
        let (rem, mut entries) = try_parse!(input,
            length_value!(value!(num_bytes),
                terminated!(
                    fold_many0!(
                        tuple!(shortstr, apply!(Value::nom_bytes, pool)),
                        pool.new_table_entries_vec(bytes),
                        |mut entries: Vec<TableEntry<'a>>, entry: TableEntry<'a>| {
                            entries.push(entry);
                            entries
                        }
                    ),
                    eof!()
                )
            )
        );

        let hashmap = pool.new_table_hashmap(entries.len());
        let mut table = Table::from_hashmap(hashmap);
        for (k, v) in entries.drain(..) {
            table.insert(k, v);
        }

        // Vec is now empty, discard the lifetimes and return it to the pool
        let entries = unsafe { mem::transmute(entries) };
        pool.return_table_entries_vec(entries);

        IResult::Done(rem, table)
    }
}