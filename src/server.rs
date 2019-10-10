#[macro_use]
extern crate log;
extern crate bytes;
extern crate env_logger;
extern crate futures;
extern crate prost;
extern crate tokio;
extern crate tower_grpc;
extern crate tower_hyper;

use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;
use std::{thread, time};

use futures::sync::{mpsc, oneshot};
use futures::{future, Future, Sink, Stream};
use tokio::net::TcpListener;
use tokio::runtime::{Builder as RuntimeBuilder, TaskExecutor};
use tower_grpc::{Code, Request, Response, Status, Streaming};
use tower_hyper::server::{Http, Server};

#[allow(dead_code)]
mod helloworld;
use helloworld::server::{Greeter, GreeterServer};
use helloworld::{HelloReply, HelloRequest};

#[derive(Clone, Debug)]
struct Greet {
    exec: TaskExecutor,
    counter: Arc<AtomicUsize>,
    ts: Arc<AtomicU64>,
}

const MSG_PER_INSPECT: usize = 1000000;

impl Greet {
    #[inline]
    fn req_to_resp(req: &HelloRequest) -> HelloReply {
        let reply = format!("hello, {}", req.name);
        HelloReply { reply }
    }
}

impl Greeter for Greet {
    type SayHelloFuture =
        Box<dyn Future<Item = Response<HelloReply>, Error = Status> + Send + 'static>;
    type CStreamSayHelloFuture =
        Box<dyn Future<Item = Response<HelloReply>, Error = Status> + Send + 'static>;
    type SStreamSayHelloStream =
        Box<dyn Stream<Item = HelloReply, Error = Status> + Send + 'static>;
    type SStreamSayHelloFuture = Box<
        dyn Future<Item = Response<Self::SStreamSayHelloStream>, Error = Status> + Send + 'static,
    >;
    type BStreamSayHelloStream =
        Box<dyn Stream<Item = HelloReply, Error = Status> + Send + 'static>;
    type BStreamSayHelloFuture = Box<
        dyn Future<Item = Response<Self::BStreamSayHelloStream>, Error = Status> + Send + 'static,
    >;

    fn say_hello(&mut self, request: Request<HelloRequest>) -> Self::SayHelloFuture {
        let (tx, rx) = oneshot::channel();
        self.exec.spawn(future::lazy(move || {
            thread::sleep(time::Duration::from_secs(1));
            tx.send(Greet::req_to_resp(request.get_ref()))
                .map_err(|_| println!("sink error"))
        }));

        let rx = rx.map_err(|_| Status::new(Code::Internal, "sink error"));
        Box::new(rx.map(Response::new))
    }

    fn c_stream_say_hello(
        &mut self,
        _request: Request<Streaming<HelloRequest>>,
    ) -> Self::CStreamSayHelloFuture {
        unimplemented!();
    }

    fn s_stream_say_hello(
        &mut self,
        _request: Request<HelloRequest>,
    ) -> Self::SStreamSayHelloFuture {
        unimplemented!();
    }

    fn b_stream_say_hello(
        &mut self,
        request: Request<Streaming<HelloRequest>>,
    ) -> Self::BStreamSayHelloFuture {
        let (tx, rx) = mpsc::channel(1024);
        self.exec.spawn(
            request
                .into_inner()
                .map(|req| Greet::req_to_resp(&req))
                .forward(tx.sink_map_err(|_| Status::new(Code::Internal, "sink error")))
                .map_err(|e| println!("grpc error: {:?}", e))
                .map(|_| println!("b_stream_say_hello successes")),
        );
        let counter = self.counter.clone();
        let ts = self.ts.clone();
        let start = time::SystemTime::now();
        let s = Box::new(
            rx.map_err(|_| Status::new(Code::Internal, "sink error"))
                .inspect(move |_| {
                    let c = counter.fetch_add(1, Ordering::SeqCst);
                    if c % MSG_PER_INSPECT == 0 {
                        let now = time::SystemTime::now().duration_since(start).unwrap();
                        let now_ts = now.as_millis() as u64;
                        let old_ts = ts.swap(now_ts, Ordering::SeqCst);
                        if old_ts != 0 {
                            let elapsed = (now_ts - old_ts) as f64 / 1000f64;
                            println!("messages per sec: {:2}", MSG_PER_INSPECT as f64 / elapsed);
                        }
                    }
                }),
        ) as Self::BStreamSayHelloStream;
        Box::new(future::ok(Response::new(s)))
    }
}

pub fn main() {
    ::env_logger::init();

    let mut runtime = RuntimeBuilder::new()
        .name_prefix("server-grpc-runtime-")
        .core_threads(1)
        .build()
        .unwrap();

    let mut http = Http::new();
    let mut http2 = http.http2_only(true).clone();
    #[allow(deprecated)]
    http2.executor(runtime.executor());

    let background_runtime = RuntimeBuilder::new()
        .name_prefix("server-background-runtime-")
        .core_threads(4)
        .build()
        .unwrap();

    let addr = "[::1]:50051".parse().unwrap();
    let bind = TcpListener::bind(&addr).expect("bind");
    let mut server = Server::new(GreeterServer::new(Greet {
        exec: background_runtime.executor(),
        counter: Arc::new(AtomicUsize::new(0)),
        ts: Arc::new(AtomicU64::new(0)),
    }));
    runtime
        .block_on(
            bind.incoming()
                .for_each(move |sock| {
                    sock.set_nodelay(true).unwrap();
                    let serve = server.serve_with(sock, http2.clone());
                    tokio::spawn(serve.map_err(|e| error!("h2 error: {:?}", e)));
                    Ok(())
                })
                .map_err(|e| eprintln!("accept error: {}", e)),
        )
        .unwrap();
}
