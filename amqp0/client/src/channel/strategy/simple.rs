// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use channel::{Config as ChannelConfig};
use super::IdStrategy;

/// Counts up from 1. Does not reuse any channel Ids
///
/// Stops returning Ids when the max channel Id is reached.
#[derive(Debug, Clone, PartialEq)]
pub struct SimpleIdStrategy {
    max_channel_id: u16,
    next_channel_id: u16,
}

impl SimpleIdStrategy {
    pub fn new() -> Self {
        SimpleIdStrategy {
            max_channel_id: ::std::u16::MAX,
            next_channel_id: 1,
        }
    }
}

impl IdStrategy for SimpleIdStrategy {
    fn assign_channel_id(&mut self, _: &ChannelConfig) -> Option<u16> {
        if self.next_channel_id == 0 {
            return None;
        }

        let channel_id = self.next_channel_id;
        self.next_channel_id = match channel_id {
            channel_id if channel_id == self.max_channel_id => 0,
            _ => channel_id + 1,
        };

        Some(channel_id)
    }

    fn tune_max_channel_id(&mut self, max_id_channels: u16) {
        self.max_channel_id = max_id_channels;
    }

    fn return_channel_id(&mut self, _: u16) {}
}