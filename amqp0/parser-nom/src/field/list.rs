// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use nom::{IResult, be_u32};
use primitives::field::{List, Value};

use pool::ParserPool;
use NomBytes;

impl<'a> NomBytes<'a> for List<'a> {
    /// TODO: Return error if there is junk at the end
    fn nom_bytes<'b, P>(input: &'a [u8], pool: &'b mut P) -> IResult<&'a [u8], Self>
        where P: ParserPool
    {
        let (input, len) = try_parse!(input, be_u32);
        if len == 0 {
            return IResult::Done(input, List::from_vec(vec![]));
        }

        println!("Length: {:?}", len);
        println!("Input: {:?}", input);
        let (_, bytes) = try_parse!(input, peek!(length_bytes!(value!(len))));
        println!("Payload: {:?}", bytes);

        #[ignore(unused_variables)]
        let (rem, entries): (&'a [u8], Vec<Value<'a>>) = try_parse!(input,
            length_value!(value!(len),
                terminated!(
                    many0!(apply!(Value::nom_bytes, pool)),
                    eof!()
                )
            )
        );

        IResult::Done(rem, List::from_vec(entries))
    }
}