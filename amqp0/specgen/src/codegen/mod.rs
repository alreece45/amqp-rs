// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod primitives;
mod specs;

use phf_codegen::OrderedMap;

pub trait FormatRustCode {
    fn format_rust(&self) -> String;
}

pub fn format_to_map<'a, I, K, V>(iter: I) -> String
    where I: Iterator<Item = (K, V)>,
          K: AsRef<str>,
          V: FormatRustCode + 'a
{
    let entries = iter.collect::<Vec<_>>();
    let entries = entries.iter().map(|& (ref k, ref v)| (
        k.as_ref(),
        v.format_rust().to_owned()
    )).collect::<Vec<_>>();

    // estimate the length of the map to reduce allocations
    let init_len = 48;
    let len_per_entry = 10;
    let formatted_len = entries.iter().fold(0, |l, &(k, ref v)| l + k.len() + v.len());
    let suffix_len = 8;

    let estimated_len = init_len + entries.len() * len_per_entry + formatted_len + suffix_len;

    let mut map = OrderedMap::new();
    for (key, value) in entries {
        map.entry(key, &value);
    }
    let mut buffer = Vec::with_capacity(estimated_len);
    buffer.extend_from_slice(b"&");

    map.build(&mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}

pub fn format_to_slice<I, T>(iter: I) -> String
    where I: Iterator<Item = T>,
          T: FormatRustCode
{
    let values = iter.map(|v| v.format_rust().to_owned()).collect::<Vec<_>>();
    if values.is_empty() {
        return "&[]".to_string();
    }

    let values_len = values.iter().fold(0, |l, v| l + v.len());
    let init_len = 2;
    let len_per_value = 2;
    let suffix_len = 2;

    let cap_array = [
        init_len,
        values_len,
        len_per_value * values.len(), // length per value
        suffix_len
    ];
    let mut formatted = String::with_capacity(cap_array.iter().sum());

    formatted.push_str("&[");

    let mut expected_len = init_len;
    debug_assert!(formatted.len() == expected_len);

    for value in values {
        formatted.push_str(&value);
        formatted.push_str(",\n");

        expected_len += len_per_value + value.len();
        debug_assert!(formatted.len() == expected_len);
    }
    formatted.push_str("\n]");

    expected_len += suffix_len;
    debug_assert!(formatted.len() == expected_len);

    formatted
}
