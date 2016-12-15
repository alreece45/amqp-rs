// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;
use std::io;

#[derive(Clone, Debug)]
pub enum State {
    Unfinished,
    Finished,
}

pub trait Authenticator {
    fn mechanism_name(&self) -> &str;
    fn recieve(&mut self, &[u8]) {}
    fn step<W>(&self, &mut W) -> io::Result<State>
        where W: io::Write;
}

#[derive(Clone, Debug)]
pub struct PlainAuthenticator<'a> {
    act_as: Cow<'a, str>,
    username: Cow<'a, str>,
    password: Cow<'a, str>,
}

impl<'a> PlainAuthenticator<'a> {
    pub fn new<I, U, P>(act_as: I, username: U, password: P) -> Self
        where I: Into<Cow<'a, str>>,
              U: Into<Cow<'a, str>>,
              P: Into<Cow<'a, str>>
    {
        PlainAuthenticator {
            act_as: act_as.into(),
            username: username.into(),
            password: password.into(),
        }
    }

    pub fn act_as(&self) -> &str {
        &*self.act_as
    }

    pub fn username(&self) -> &str {
        &*self.username
    }

    pub fn password(&self) -> &str {
        &*self.password
    }
}

impl<'a> Authenticator for PlainAuthenticator<'a> {
    fn mechanism_name(&self) -> &'static str {
        "PLAIN"
    }

    fn step<W>(&self, writer: &mut W) -> io::Result<State>
        where W: io::Write
    {
        try!(write!(writer, "{}\0{}\0{}\0", self.act_as, self.username, self.password));
        Ok(State::Finished)
    }
}