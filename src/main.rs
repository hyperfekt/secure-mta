#![feature(pin, arbitrary_self_types, futures_api, async_await, await_macro)]

extern crate futures;
extern crate futures_util;
extern crate fahrenheit;

use futures::io::AsyncReadExt;
use futures::io::AsyncWriteExt;
use futures::future::FutureExt;
use futures::stream::StreamExt;
use fahrenheit::AsyncTcpListener;
use fahrenheit::AsyncTcpStream;
use futures_util::future::join_all;

async fn accept_at(address : &str) {
    let listener = AsyncTcpListener::bind(address).unwrap();
    let mut incoming = listener.incoming();
    while let Some(stream) = await!(incoming.next()) {
        fahrenheit::spawn(handle(stream));
    }
}

async fn handle(mut stream: AsyncTcpStream) {
    let mut buf = vec![0;10];
    await!(stream.read_exact(&mut buf)).unwrap();
    println!("{}", String::from_utf8_lossy(&buf));
}

fn main() {
    let sockets = [ "127.0.0.1:8080", "127.0.0.1:8070" ];
    /* for each socket we create a future that will be responsible for accepting incoming connections, ensuring to box it in order to gain the Unpin trait, which the futures created by async functions have not. afterwards we join all these futures into a single one and then throw away the result with map. to use map our new future needs to have the Future trait, which we only get if it implements Unpin - since we made sure earlier that all the constituent futures do, this is the case (see https://github.com/rust-lang-nursery/wg-net/blob/master/async-book/src/pinning/chapter.md).
     */
    let listeners = sockets.into_iter().map(|&x| Box::pinned(accept_at(x)));
    fahrenheit::run(join_all(listeners).map(|_| ())); 
}