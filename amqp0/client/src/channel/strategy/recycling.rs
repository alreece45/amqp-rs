// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::VecDeque;
use channel::{Config as ChannelConfig};
use super::{IdStrategy, SimpleIdStrategy};

/// Channel identifier strategy that counts up from 1 upward, but will reuse channel Ids.
///
/// Will reuse channels in the order they're released-- it may not be the lowest
/// channel Id first.
///
#[derive(Debug, Clone, PartialEq)]
pub struct RecyclingIdStrategy {
    inner: SimpleIdStrategy,
    available_channels: VecDeque<u16>,
}

impl RecyclingIdStrategy {
    pub fn new() -> Self {
        RecyclingIdStrategy {
            inner: SimpleIdStrategy::new(),
            available_channels: VecDeque::new(),
        }
    }
}

impl IdStrategy for RecyclingIdStrategy {
    fn assign_channel_id(&mut self, channel: &ChannelConfig) -> Option<u16> {
        self.available_channels.pop_front()
            .or_else(|| self.inner.assign_channel_id(channel))
    }

    fn return_channel_id(&mut self, channel: u16) {
        self.available_channels.push_back(channel)
    }

    fn tune_max_channel_id(&mut self, max_id_channels: u16) {
        self.inner.tune_max_channel_id(max_id_channels)
    }
}
