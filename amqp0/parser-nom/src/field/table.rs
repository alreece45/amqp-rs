// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;

use nom::{IResult, be_u32};
use primitives::field::{TableEntries, TableEntry, Value};

use common::shortstr;
use pool::ParserPool;
use NomBytes;

impl<'a> NomBytes<'a> for TableEntries<'a> {
    type Output = TableEntries<'a>;
    fn nom_bytes<'b, P>(input: &'a [u8], pool: &'b mut P) -> IResult<&'a [u8], Self>
        where P: ParserPool
    {
        let (input, num_bytes) = try_parse!(input, be_u32);
        let (_, bytes) = try_parse!(input, peek!(length_bytes!(value!(num_bytes))));

        #[ignore(unused_variables)]
        map!(input,
            length_value!(
                value!(num_bytes),
                terminated!(
                    fold_many0!(
                        tuple!(
                            map!(shortstr, |s| Cow::Borrowed(s)),
                            apply!(Value::nom_bytes, pool)
                        ),
                        pool.new_table_entries_vec(bytes),
                        |mut entries: Vec<TableEntry<'a>>, entry: TableEntry<'a>| {
                            entries.push(entry);
                            entries
                        }
                    ),
                    eof!()
                )
            ),
            |entries| TableEntries::from_entries(entries)
        )
    }
}