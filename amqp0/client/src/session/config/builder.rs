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
use primitives::Spec;
use primitives::rabbitmq9_1::Rabbitmq9_1;

use session::auth::{Authenticator, PlainAuthenticator};
use channel;
use super::ParseUrlError;

type DefaultAuthenticator = PlainAuthenticator<'static>;
type DefaultSpec = Rabbitmq9_1;

/// Provides an interface to build a session configuration (`SessionConfig`)
#[derive(Debug, Clone, PartialEq)]
pub struct ConfigBuilder<'a, S, A, C>
    where S: Spec,
          A: Authenticator,
          C: channel::IdStrategy
{
    config: Config<'a, S, A, C>
}

impl<'a> ConfigBuilder<'a, Rabbitmq9_1, PlainAuthenticator<'a>, channel::DefaultIdStrategy> {
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

impl<'a, S, A, C> ConfigBuilder<'a, S, A, C>
    where S: Spec,
          A: Authenticator,
          C: channel::IdStrategy
{
    pub fn host<H>(mut self, host: H) -> Self
        where H: Into<Cow<'a, str>>
    {
        self.config.host = host.into();
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.config.port = port.into();
        self
    }

    pub fn virtual_host<V>(mut self, virtual_host: V) -> Self
        where V: Into<Cow<'a, str>>
    {
        self.config.virtual_host = virtual_host.into();
        self
    }

    pub fn use_tls(mut self, use_tls: bool) -> Self {
        self.config.use_tls = use_tls;
        self
    }

    pub fn locale<L>(mut self, locale: L) -> Self
        where L: Into<Cow<'a, str>>
    {
        self.config.locale = locale.into();
        self
    }

    pub fn channel_max(mut self, channel_max: u16) -> Self {
        self.config.channel_max = channel_max;
        self
    }

    pub fn plain_auth<U, P>(self, username: U, password: P) -> ConfigBuilder<'a, S, PlainAuthenticator<'a>, C>
        where U: Into<Cow<'a, str>>,
              P: Into<Cow<'a, str>>
    {
        let authenticator = PlainAuthenticator::new("", username.into(), password.into());
        ConfigBuilder {
            config: self.config.with_authenticator(authenticator)
        }
    }

    pub fn with_spec<SS>(self) -> ConfigBuilder<'a, SS, A, C>
        where SS: Spec
    {
        ConfigBuilder {
            config: self.config.with_spec::<SS>()
        }
    }

    pub fn with_authenticator<T, U>(self, authenticator: U) -> ConfigBuilder<'a, S, T, C>
        where T: Authenticator,
              U: Into<T>
    {
        ConfigBuilder {
            config: self.config.with_authenticator(authenticator)
        }
    }

    pub fn with_channel_id_strategy<T, U>(self, channel_id_strategy: U) -> ConfigBuilder<'a, S, A, T>
        where T: channel::IdStrategy,
              U: Into<T>
    {
        ConfigBuilder {
            config: self.config.with_channel_id_strategy(channel_id_strategy)
        }
    }
}


impl<'a, S, A, C> From<ConfigBuilder<'a, S, A, C>> for Config<'a, S, A, C>
    where S: Spec,
          A: Authenticator,
          C: channel::IdStrategy
{
    fn from(config: ConfigBuilder<'a, S, A, C>) -> Self {
        config.config
    }
}