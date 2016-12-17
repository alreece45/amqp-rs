// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::Config;

use std::borrow::Cow;
use url::Url;
use parser_nom::NomBytes;
use primitives::{Protocol, Rabbitmq9_1};

use channel;
use session::BlockingSession;
use session::auth::{Authenticator, PlainAuthenticator};
use super::ParseUrlError;

type DefaultAuthenticator = PlainAuthenticator<'static>;
type DefaultProtocol = Rabbitmq9_1;

/// Provides an interface to build a session configuration (`SessionConfig`)
#[derive(Debug, Clone, PartialEq)]
pub struct ConfigBuilder<'config, S, A, C>
    where A: Authenticator,
          C: channel::IdStrategy
{
    config: Config<'config, S, A, C>
}

impl<'config> ConfigBuilder<'config, Rabbitmq9_1, PlainAuthenticator<'config>, channel::DefaultIdStrategy> {
    pub fn parse_url<U>(url: U) -> Result<Self, ParseUrlError>
        where U: AsRef<str>
    {
        let parsed = try!(Url::parse(url.as_ref()));

        let mut config = match parsed.scheme() {
            "amqps" => Self::with_tls(),
            "amqp" => Self::without_tls(),
            invalid_scheme => return Err(ParseUrlError::InvalidScheme(invalid_scheme.to_string())),
        };

        if let Some(host) = parsed.host_str() {
            config = config.host(host.to_string());
        }
        if let Some(port) = parsed.port() {
            config = config.port(port);
        }

        match parsed.path() {
            "" => (),
            virtual_host => config = config.virtual_host(virtual_host.to_string())
        }

        let username = parsed.username();
        let password = parsed.password().unwrap_or("");
        config = config.plain_auth(username.to_string(), password.to_string());

        Ok(config)
    }

    pub fn without_tls() -> Self {
        ConfigBuilder {
            config: Config::new("localhost", 5671, "/", false)
        }
    }

    pub fn with_tls() -> Self {
        ConfigBuilder {
            config: Config::new("localhost", 5672, "/", true)
        }
    }
}

impl<'config, P, A, C> ConfigBuilder<'config, P, A, C>
    where A: Authenticator,
          C: channel::IdStrategy
{
    pub fn into_blocking(self) -> BlockingSession<'config, P, A, C> {
        BlockingSession::from_config(self.into())
    }
}

impl<'config, P, A, C> ConfigBuilder<'config, P, A, C>
    where A: Authenticator,
          C: channel::IdStrategy
{
    pub fn host<H>(mut self, host: H) -> Self
        where H: Into<Cow<'config, str>>
    {
        self.config.host = host.into();
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.config.port = port.into();
        self
    }

    pub fn virtual_host<V>(mut self, virtual_host: V) -> Self
        where V: Into<Cow<'config, str>>
    {
        self.config.virtual_host = virtual_host.into();
        self
    }

    pub fn use_tls(mut self, use_tls: bool) -> Self {
        self.config.use_tls = use_tls;
        self
    }

    pub fn locale<L>(mut self, locale: L) -> Self
        where L: Into<Cow<'config, str>>
    {
        self.config.locale = locale.into();
        self
    }

    pub fn channel_max(mut self, channel_max: u16) -> Self {
        self.config.channel_max = channel_max;
        self
    }

    pub fn plain_auth<T, U>(self, username: T, password: U) -> ConfigBuilder<'config, P, PlainAuthenticator<'config>, C>
        where T: Into<Cow<'config, str>>,
              U: Into<Cow<'config, str>>
    {
        let authenticator = PlainAuthenticator::new("", username.into(), password.into());
        ConfigBuilder {
            config: self.config.with_authenticator(authenticator)
        }
    }

    pub fn with_protocol<'a, SS>(self) -> ConfigBuilder<'config, SS, A, C>
        where SS: Protocol<'a>
    {
        ConfigBuilder {
            config: self.config.with_protocol::<SS>()
        }
    }

    pub fn with_authenticator<T, U>(self, authenticator: U) -> ConfigBuilder<'config, P, T, C>
        where T: Authenticator,
              U: Into<T>
    {
        ConfigBuilder {
            config: self.config.with_authenticator(authenticator)
        }
    }

    pub fn with_channel_id_strategy<T, U>(self, channel_id_strategy: U) -> ConfigBuilder<'config, P, A, T>
        where T: channel::IdStrategy,
              U: Into<T>
    {
        ConfigBuilder {
            config: self.config.with_channel_id_strategy(channel_id_strategy)
        }
    }
}


impl<'config, P, A, C> From<ConfigBuilder<'config, P, A, C>> for Config<'config, P, A, C>
    where A: Authenticator,
          C: channel::IdStrategy
{
    fn from(config: ConfigBuilder<'config, P, A, C>) -> Self {
        config.config
    }
}