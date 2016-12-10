
use std::borrow::Cow;

pub struct Frame<'a> {
    track: u8,
    channel: u8,
    payload: Cow<'a, [u8]>
}

impl<'a> Frame<'a> {
    pub fn new<P>(track: u8, channel: u8, payload: P) -> Self
        where P: Into<Cow<'a, [u8]>>
    {
        Frame {
            track: track,
            channel: channel,
            payload: payload.into(),
        }
    }

    pub fn track(&self) -> u8 {
        self.track
    }

    pub fn channel(&self) -> u8 {
        self.channel
    }

    pub fn payload(&self) -> &[u8] {
        &*self.payload
    }
}