extern crate mio;

use std::str;
use std::old_io::net::ip::{Ipv4Addr};

use self::mio::*;
use self::mio::buf::{ByteBuf, MutByteBuf, SliceBuf};
use self::mio::net::*;
use self::mio::net::udp::*;
use self::mio::util::Slab;

type ServerEventLoop = EventLoop<usize, ()>;

const LISTENER: Token = Token(0);
const SERVER: Token = Token(1);

struct EchoConnection {
    recv_socket: UdpSocket,
    buffer: Option<ByteBuf>,
    mut_buffer: Option<MutByteBuf>,
    token: Token,
    interest: Interest
}

impl EchoConnection {
    fn new(recv_socket: UdpSocket) -> EchoConnection {
        EchoConnection {
            recv_socket: recv_socket,
            buffer: None,
            mut_buffer: Some(ByteBuf::mut_with_capacity(2048)),
            token: Token(-1),
            interest: Interest::hup()
        }
    }

    fn readable(&mut self, event_loop: &mut ServerEventLoop) -> MioResult<()> {
        let mut buffer = self.mut_buffer.take().unwrap();

        match self.recv_socket.read(&mut buffer) {
            Ok(NonBlock::WouldBlock) => {
                error!("Received a readable, but unable to read from socket");
            },

            Ok(NonBlock::Ready(r)) => {
                debug!("CONN: We read {} bytes!", r);
            },

            Err(e) => {
                debug!("Not implemented; client err={:?}", e);
            }
        };

        self.buffer = Some(buffer.flip());
        event_loop.reregister(&self.recv_socket, self.token, self.interest, PollOpt::edge());

        Ok(())
    }
}

struct EchoServer {
    socket: UdpSocket,
    connections: Slab<EchoConnection>
}

impl EchoServer {
    fn accept(&mut self, event_loop: &mut ServerEventLoop, token: Token){
        debug!("Accepting new connection");

        let connection = EchoConnection::new(self.socket.bound().unwrap());
        let token = self.connections.insert(connection).ok()
            .expect("Could not add connection to slab");

        self.connections[token].token = token;
        event_loop.register_opt(&self.connections[token].recv_socket, token,
                                Interest::readable(), PollOpt::edge() | PollOpt::oneshot())
            .ok().expect("Could not register socket with event loop");
    }

    fn conn_readable(&mut self, event_loop: &mut ServerEventLoop, token: Token) {
        debug!("Server connection readable; token={:?}", token);
        self.get(token).readable(event_loop);
    }

    fn get<'a>(&'a mut self, token: Token) -> &'a mut EchoConnection {
        &mut self.connections[token]
    }
}

struct EchoHandler {
    server: EchoServer
}

impl EchoHandler {
    fn new(server_socket: UdpSocket) -> EchoHandler {
        EchoHandler {
            server: EchoServer {
                socket: server_socket,
                connections: Slab::new_starting_at(Token(2), 128)
            }
        }
    }
}

impl Handler<usize, ()> for EchoHandler {
    fn readable(&mut self, event_loop: &mut ServerEventLoop, token: Token, hint: ReadHint) {
        assert!(hint.is_data());

        match token {
            i => self.server.conn_readable(event_loop, i)
        };
    }
}

pub fn build() {
    debug!("Starting test echo server");
    let mut event_loop = EventLoop::new().unwrap();

    let addr = SockAddr::parse("127.0.0.1:3000").unwrap();
    let server = UdpSocket::v4().unwrap();

    server.bind(&addr).unwrap();
    server.set_reuseaddr(true).unwrap();

    info!("Listen for connections");
    event_loop.register_opt(&server, SERVER, Interest::readable(), PollOpt::edge() | PollOpt::oneshot()).unwrap();

    event_loop.run(&mut EchoHandler::new(server)).unwrap();
}
