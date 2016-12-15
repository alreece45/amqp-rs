// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod recycling;
mod simple;

use channel::Config as ChannelConfig;

pub use self::simple::SimpleIdStrategy;
pub use self::recycling::RecyclingIdStrategy;

/// The strategy to assign a Id to a channel may vary, and is delegated
/// to this trait.
pub trait IdStrategy {
    /// When the connection is tuned, the server and the client agree on
    /// the maximum channel Id.
    ///
    /// Note, this is a Id, not the number of channels. For example, if the
    /// max_channel_id is 2, only two Ids available are 1 and 2. It would
    /// be illegal, per the spec, to open a channel with an id of 3
    fn tune_max_channel_id(&mut self, u16);

    /// Called when a channel is requested without a explicit channel id
    /// Returns Some(u16) with the next channel id to use, otherwise None if
    /// the strategy can not assign any more channel ids
    fn assign_channel_id(&mut self, &ChannelConfig) -> Option<u16>;

    /// Called wen a channel is released (e.g: dropped)
    fn return_channel_id(&mut self, u16);
}

#[cfg(test)]
mod tests {

}