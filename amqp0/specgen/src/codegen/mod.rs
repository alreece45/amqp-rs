// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod primitives;
mod specs;

pub trait FormatRustCode {
    fn format_rust(&self) -> String;
}

pub fn format_to_vec<I, T>(iter: I) -> String
    where I: Iterator<Item = T>,
          T: FormatRustCode
{
    let values = iter.map(|v| v.format_rust().to_owned()).collect::<Vec<_>>();

    if values.is_empty() {
        return "vec![]".to_string();
    }

    let values_len = values.iter().fold(0, |l, v| l + v.len());

    // rustfmt doesn't format the vec![] macros very well, yet; avoid them until formatting improves
    if cfg!(feature = "macros") {
        let init_len = 5;
        let len_per_value = 2;
        let suffix_len = 2;

        let cap_array = [
            5, // init
            values_len,
            values.len() * 2, // length per value
            2 // suffix_len
        ];
        let mut formatted = String::with_capacity(cap_array.iter().sum());

        formatted.push_str("vec![");

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
    } else {
        const SUFFIX_LENGTH: usize = 4;
        const PREFIX_LENGTH: usize = 36;
        const LENGTH_PER_VALUE: usize = 10;

        let capacity_len = format!("{}", values.len()).len();
        let formatted_cap_nums = [
            PREFIX_LENGTH,
            capacity_len,
            values_len,
            values.len() * LENGTH_PER_VALUE,
            SUFFIX_LENGTH,
        ];

        let mut formatted = String::with_capacity(formatted_cap_nums.iter().sum());

        formatted.push_str(&format!("{{\nlet mut v = Vec::with_capacity({});\n", values.len()));

        let mut expected_len = PREFIX_LENGTH + capacity_len;
        debug_assert!(formatted.len() == expected_len);

        for value in values {
            formatted.push_str(&format!("v.push({});\n", value));

            expected_len += LENGTH_PER_VALUE + value.len();
            debug_assert!(formatted.len() == expected_len);
        }

        formatted.push_str("\nv\n}");

        expected_len += SUFFIX_LENGTH;
        debug_assert!(formatted.len() == expected_len);

        formatted
    }
}