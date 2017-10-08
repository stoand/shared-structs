#![feature(conservative_impl_trait)]

#[macro_use]
extern crate error_chain;

extern crate futures;

extern crate bytes;
extern crate tokio_core;
extern crate tokio_io;

use futures::sync::mpsc;
use std::str;
use tokio_io::{io, AsyncRead};
use futures::{stream, Future, Stream};
use std::net::SocketAddr;


enum RelayAction {
    AddPeer(SocketAddr),
}

enum Action<A> {
    App(A),
    Internal(RelayAction),
}

mod errors {
    error_chain!{
        foreign_links {
            Io(::std::io::Error);
        }
    }
}

use errors::*;

pub fn run_action_relay<A>(
    user: &str,
    pass: &str,
    bind_to: SocketAddr,
    initial_peers: Vec<SocketAddr>,
    on_action: &FnMut(A),
) -> Result<()> {
    // let (mut tx, rx) = mpsc::channel(1);

    let mut core = tokio_core::reactor::Core::new()?;
    let handle = core.handle();

    let server = tokio_core::net::TcpListener::bind(&bind_to, &handle)?
        .incoming()
        .map(|(tcp, _)| {
            let (mut reader, _) = tcp.split();

            let buf = "asdf".to_owned().into_boxed_str().into_boxed_bytes();

            stream::poll_fn(move || {
                let mut buf = Vec::new();
                reader
                    .read_buf(&mut buf)
                    .map(|async| async.map(|num| Some(num)))
            })
        })
        .flatten();

    let server = server.for_each(|_| Ok(()));

    core.run(server)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::*;
    }
}
