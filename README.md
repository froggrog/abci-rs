# Rust ABCI
A rust implementation of the ABCI protocol for Tendermint.

## About abci-rs
This library implements the ABCI protocol and can be used to write ABCI applications for Tendermint in rust.
Here you can find more information about [Tendermint](https://github.com/tendermint/tendermint) and [ABCI application](https://github.com/tendermint/abci).

This library based on [rust-tsp](https://github.com/tendermint/rust-tsp)

## Installation
To use this library to build your own ABCI apps in rust you have to include the following in your `Cargo.toml` file.

```TOML

[dependencies]
abci-rs = { git = "https://github.com/neocortexlab/abci-rs" }

```

## First ABCI app
This app will work as an echo server, for testing use [abci-cli](https://github.com/tendermint/abci).

```rust

extern crate abci_rs;

struct EchoApp;

impl Application for EchoApp {
    fn begin_block(&self, p: &RequestBeginBlock) -> ResponseBeginBlock {
        println!("begin_block");
        ResponseBeginBlock::new()
    }

    fn check_tx(&self, p: &RequestCheckTx) -> ResponseCheckTx {
        println!("check_tx");
        ResponseCheckTx::new()
    }

    fn commit(&self, p: &RequestCommit) -> ResponseCommit {
        println!("commit");
        ResponseCommit::new()
    }

    fn deliver_tx(&self, p: &RequestDeliverTx) -> ResponseDeliverTx {
        println!("deliver_tx");
        ResponseDeliverTx::new()
    }

    fn echo(&self, p: &RequestEcho) -> ResponseEcho {
        let mut response = ResponseEcho::new();
        response.set_message(p.get_message().to_owned());
        return response;
    }

    fn end_block(&self, p: &RequestEndBlock) -> ResponseEndBlock {
        println!("end_block");
        ResponseEndBlock::new()
    }

    fn flush(&self, p: &RequestFlush) -> ResponseFlush {
        println!("flush");
        ResponseFlush::new()
    }

    fn init_chain(&self, p: &RequestInitChain) -> ResponseInitChain {
        println!("init_chain");
        ResponseInitChain::new()
    }

    fn info(&self, p: &RequestInfo) -> ResponseInfo {
        println!("indo {:?}", p);
        ResponseInfo::new()
    }

    fn query(&self, p: &RequestQuery) -> ResponseQuery {
        println!("query");
        ResponseQuery::new()
    }

    fn set_option(&self, p: &RequestSetOption) -> ResponseSetOption {
        println!("set_option {:?}", p);
        ResponseSetOption::new()
    }
}

fn main() {
    static APP: EchoApp = EchoApp;
    let addr = "127.0.0.1:46658".parse().unwrap();
    println!("Starting ABCIServer on {:?}", addr);
    server::start(addr, &APP);
}
```

