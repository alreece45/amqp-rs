// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod auth;
mod config;

use std::fmt::Debug;
use std::io::{self, Write};
use std::net::TcpStream;

use netbuf::Buf;
use primitives::Protocol;
use parser_nom::NomBytes;
use parser_nom::pool::NoParserPool;

use channel;

pub use self::auth::{Authenticator, PlainAuthenticator};
pub use self::config::{Config, ConfigBuilder};

#[derive(Debug)]
enum State {
    Unconnected,
    Started(Buf, TcpStream), // Before sending StartOK
    Secured(Buf, TcpStream), // Before sending SecureOK
    Tuned(Buf, TcpStream),   // Before sending TuneOK
    Opened(Buf, TcpStream),  // State we need for most things to be executed
    Closed,
}

#[derive(Debug)]
pub struct BlockingSession<'config, S, A, C>
    where A: Authenticator,
          C: channel::IdStrategy
{
    config: Config<'config, S, A, C>,
    state: State,
    closing: Vec<(Buf, TcpStream)>,
}

impl<'config, 'b, P, A, C> BlockingSession<'config,  P, A, C>
    where A: Authenticator,
          C: channel::IdStrategy
{
    pub fn from_config(config: Config<'config, P, A, C>) -> Self {
        BlockingSession {
            config: config,
            state: State::Unconnected,
            closing: vec![],
        }
    }
}

impl<'config, 'p, P, A, C> BlockingSession<'config, P, A, C>
    where P: Protocol<'p>,
          P::Frame: NomBytes<'p> + Debug,
          A: Authenticator,
          C: channel::IdStrategy
{
    pub fn connect(&'p mut self) -> io::Result<P::Frame> {
        if let State::Unconnected = self.state {
            let server_addr = (self.config.host(), self.config.port());
            let mut stream = try!(TcpStream::connect(server_addr));
            try!(stream.write_all(P::protocol_header()));
            self.state = State::Started(Buf::new(), stream);
        }

        if let State::Started(ref mut buf, ref mut stream) = self.state {
            try!(buf.read_from(stream));
            let frame = P::Frame::nom_bytes(&buf[..4], &mut NoParserPool);

            let (_, frame) = frame.unwrap();
            println!("{:#?}", frame);
            return Ok(frame);
        }
        Err(io::Error::new(io::ErrorKind::Other, "No Start Frame"))
    }
}
