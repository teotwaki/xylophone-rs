extern crate mio;

use self::mio::net::SockAddr;
use self::mio::net::udp::UdpSocket;
use self::mio::net::tcp::TcpAcceptor;
use self::mio::{IoWriter, IoAcceptor, PollOpt, Interest, ReadHint};

const RESPONSE: &'static str = "foobar";
