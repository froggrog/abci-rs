use super::Application;

use tokio::io;

use tokio_service::Service;

use futures::{future, Future};

use types::*;

pub struct ABCIService {
    pub app: &'static Application
}

impl ABCIService {
    pub fn handle(&self, request: &Request) -> Response {
        let mut response = Response::new();

        // Info/Query connection
        if request.has_info() {
            response.set_info(self.app.info(request.get_info()));
            return response;
        }

        if request.has_set_option() {
            response.set_set_option(self.app.set_option(request.get_set_option()));
            return response;
        }

        if request.has_query() {
            response.set_query(self.app.query(request.get_query()));
            return response;
        }

        // Mempool connection
        if request.has_check_tx() {
            response.set_check_tx(self.app.check_tx(request.get_check_tx()));
            return response;
        }

        // Consensus connection
        if request.has_init_chain() {
            response.set_init_chain(self.app.init_chain(request.get_init_chain()));
            return response;
        }

        if request.has_begin_block() {
            response.set_begin_block(self.app.begin_block(request.get_begin_block()));
            return response;
        }

        if request.has_deliver_tx() {
            response.set_deliver_tx(self.app.deliver_tx(request.get_deliver_tx()));
            return response;
        }

        if request.has_end_block() {
            response.set_end_block(self.app.end_block(request.get_end_block()));
            return response;
        }

        if request.has_commit() {
            response.set_commit(self.app.commit(request.get_commit()));
            return response;
        }

        // Miscellaneous connection
        if request.has_echo() {
            response.set_echo(self.app.echo(request.get_echo()));
            return response;
        }

        if request.has_flush() {
            response.set_flush(self.app.flush(request.get_flush()));
            return response;
        }

        unreachable!();
    }
}

impl Service for ABCIService {
    type Request = Request;
    type Response = Response;
    type Error = io::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        Box::new(future::ok(self.handle(&req)))
    }
}