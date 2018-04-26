use super::Application;

use std::net::SocketAddr;

use tokio::io;

use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::{Framed};

use tokio_proto::TcpServer;
use tokio_proto::pipeline::ServerProto;

use types::*;
use abci_service::ABCIService;
use abci_codec::ABCICodec;

pub fn start<A: Application + Send + Sync + 'static> (addr: SocketAddr, app: &'static A) -> (){
    let server = TcpServer::new(ABCIProto, addr);
    server.serve(move|| Ok(ABCIService{app: app}));
}

struct ABCIProto;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for ABCIProto {
    type Request = Request;
    type Response = Response;
    type Transport = Framed<T, ABCICodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(ABCICodec))
    }
}