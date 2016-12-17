// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod builder;

use std::borrow::Cow;
use std::marker::PhantomData;
use url;

use primitives::Protocol;
use primitives::field::TableEntries;
use primitives::Rabbitmq9_1;
use super::auth::{Authenticator, PlainAuthenticator};

use channel::{
    IdStrategy as ChannelIdStrategy,
    DefaultIdStrategy as DefaultChannelIdStrategy
};

pub use self::builder::ConfigBuilder;

pub enum ParseUrlError {
    InvalidScheme(String),
    ParseError(url::ParseError)
}

impl From<url::ParseError> for ParseUrlError {
    fn from(e: url::ParseError) -> Self {
        ParseUrlError::ParseError(e)
    }
}

/// The sealed/completed configuration (the other being the config-builder object)
#[derive(Debug, Clone, PartialEq)]
pub struct Config<'config, P, A, C>
    where A: Authenticator,
          C: ChannelIdStrategy
{
    host: Cow<'config, str>,
    port: u16,
    virtual_host: Cow<'config, str>,
    use_tls: bool,

    locale: Cow<'config, str>,
    authenticator: A,
    properties: TableEntries<'config>,
    channel_id_strategy: C,

    // tunable properties
    channel_max: u16,
    frame_max: u32,
    heartbeat: u16,

    protocol: PhantomData<P>,
}

impl<'config, 'bytes> Config<'config, Rabbitmq9_1, PlainAuthenticator<'config>, DefaultChannelIdStrategy> {
    pub fn new<H, V>(host: H, port: u16, virtual_host: V, use_tls: bool) -> Self
        where H: Into<Cow<'config, str>>,
              V: Into<Cow<'config, str>>,
    {
        Config {
            host: host.into(),
            port: port,
            virtual_host: virtual_host.into(),
            use_tls: use_tls,

            locale: "en_US".into(),
            authenticator: PlainAuthenticator::new("", "", ""),
            channel_id_strategy: DefaultChannelIdStrategy::new(),
            properties: TableEntries::new(),

            protocol: PhantomData,

            // tune
            channel_max: 0,
            frame_max: 0,
            heartbeat: 0,
        }
    }
}

impl<'config, S, A, C> Config<'config, S, A, C>
    where A: Authenticator,
          C: ChannelIdStrategy
{
    fn with_authenticator<T, U>(self, authenticator: U) -> Config<'config, S, T, C>
        where T: Authenticator,
              U: Into<T>
    {
        Config {
            protocol: self.protocol,

            host: self.host,
            port: self.port,
            virtual_host: self.virtual_host,
            use_tls: self.use_tls,

            locale: self.locale,
            authenticator: authenticator.into(),
            channel_id_strategy: self.channel_id_strategy,
            properties: self.properties,

            // tune
            channel_max: self.channel_max,
            frame_max: self.frame_max,
            heartbeat: self.heartbeat,
        }
    }

    fn with_channel_id_strategy<T, U>(self, channel_id_strategy: U) -> Config<'config, S, A, T>
        where T: ChannelIdStrategy,
              U: Into<T>
    {
        Config {
            protocol: self.protocol,

            host: self.host,
            port: self.port,
            virtual_host: self.virtual_host,
            use_tls: self.use_tls,

            locale: self.locale,
            authenticator: self.authenticator,
            channel_id_strategy: channel_id_strategy.into(),
            properties: self.properties,

            // tune
            channel_max: self.channel_max,
            frame_max: self.frame_max,
            heartbeat: self.heartbeat,
        }
    }

    fn with_protocol<'a, SS: Protocol<'a>>(self) -> Config<'config, SS, A, C> {
        Config {
            protocol: PhantomData,

            host: self.host,
            port: self.port,
            virtual_host: self.virtual_host,
            use_tls: self.use_tls,

            locale: self.locale,
            authenticator: self.authenticator,
            channel_id_strategy: self.channel_id_strategy,
            properties: self.properties,

            // tune
            channel_max: self.channel_max,
            frame_max: self.frame_max,
            heartbeat: self.heartbeat,
        }
    }

    pub fn host(&self) -> &str {
        &*self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn virtual_host(&self) -> &str {
        &*self.virtual_host
    }

    pub fn use_tls(&self) -> bool {
        self.use_tls
    }

    pub fn locale(&self) -> &str {
        &*self.locale
    }

    pub fn authenticator(&self) -> &A {
        &self.authenticator
    }

    pub fn properties(&self) -> &TableEntries<'config> {
        &self.properties
    }

    pub fn channel_id_strategy(&self) -> &C {
        &self.channel_id_strategy
    }
}