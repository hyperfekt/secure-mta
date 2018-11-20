#![feature(pin, arbitrary_self_types, futures_api, async_await)]
use std::net::{TcpStream, TcpListener};
use futures::executor::LocalPool;
use futures::task::SpawnExt;

type Error = Result<(), Box<std::error::Error>>;

async fn accept_at(address : &str) {
    println!("hello from the future!");
}

fn main() -> Error {
    let mut executor = LocalPool::new();
    let sockets = ["127.0.0.1:8080"];
    for socket in &sockets {
        executor.spawner().spawn(accept_at(&socket));
    }
    executor.run();
    Ok(())
}