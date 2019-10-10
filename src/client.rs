extern crate env_logger;
extern crate futures;
extern crate http;
extern crate hyper;
extern crate tokio;
extern crate tokio_threadpool;
extern crate tower_hyper;
extern crate tower_request_modifier;
extern crate tower_util;

use std::iter;

use futures::{future, stream, Future, Stream};
use hyper::client::connect::{Destination, HttpConnector};
use tokio::runtime::{Builder as RuntimeBuilder, Runtime};
use tower_grpc::generic::client::GrpcService;
use tower_grpc::{Body, BoxBody, Request, Status};
use tower_hyper::client::{Builder as ClientBuilder, Connect};
use tower_hyper::util::Connector;
use tower_util::MakeService;

#[allow(dead_code)]
mod helloworld;
use helloworld::client::Greeter;
use helloworld::HelloRequest;

pub fn main() {
    env_logger::init();

    let uri: http::Uri = format!("http://[::1]:50051").parse().unwrap();
    let dst = Destination::try_from_uri(uri.clone()).unwrap();
    let connector = Connector::new(HttpConnector::new(1)); // 1 thread for DNS.
    let settings = ClientBuilder::new().http2_only(true).clone();

    let mut runtime = RuntimeBuilder::new()
        .name_prefix("client-grpc-runtime-")
        .core_threads(4)
        .build()
        .unwrap();

    let mut client = Connect::with_executor(connector, settings, runtime.executor())
        .make_service(dst)
        .map_err(|e| println!("establish connection fail: {:?}", e))
        .map(move |conn| {
            let req_modifier = tower_request_modifier::Builder::new()
                .set_origin(uri)
                .build(conn)
                .unwrap();
            Greeter::new(req_modifier)
        })
        .wait()
        .unwrap();

    // for _ in 0..10 {
    //     let client_f = unary_example(client, &runtime);
    //     client = runtime.block_on(client_f).unwrap();
    // }

    for _ in 0..1 {
        let client_f = bstream_example(client, &runtime);
        client = runtime.block_on(client_f).unwrap();
    }

    drop(client);
    runtime.shutdown_on_idle().wait().unwrap();
}

#[allow(dead_code)]
fn unary_example<T>(
    client: Greeter<T>,
    runtime: &Runtime,
) -> impl Future<Item = Greeter<T>, Error = ()>
where
    T: GrpcService<BoxBody>,
    <T as GrpcService<BoxBody>>::Future: Send + 'static,
    <T as GrpcService<BoxBody>>::ResponseBody: Send + 'static,
    <<T as GrpcService<BoxBody>>::ResponseBody as Body>::Data: Send,
{
    let executor = runtime.executor();
    client
        .ready()
        .map_err(|e| println!("poll the client fail: {:?}", e))
        .map(move |mut client| {
            let name = "Client".to_owned();
            executor.spawn(
                client
                    .say_hello(Request::new(new_request(name)))
                    .map(|resp| println!("RESPONSE = {:?}", resp))
                    .map_err(|e| panic!("gRPC failed; err={:?}", e)),
            );
            client
        })
}

#[allow(dead_code)]
fn bstream_example<T>(
    client: Greeter<T>,
    runtime: &Runtime,
) -> impl Future<Item = Greeter<T>, Error = ()>
where
    T: GrpcService<BoxBody>,
    <T as GrpcService<BoxBody>>::Future: Send + 'static,
    <T as GrpcService<BoxBody>>::ResponseBody: Send + 'static,
    <<T as GrpcService<BoxBody>>::ResponseBody as Body>::Data: Send,
{
    let executor = runtime.executor();
    client
        .ready()
        .map_err(|e| println!("poll the client fail: {:?}", e))
        .map(move |mut client| {
            let name = "Client".to_owned();
            let s = stream::iter_ok::<_, Status>(iter::repeat(new_request(name)));
            // let s = stream::iter_ok::<_, Status>(iter::repeat(new_request(name)).take(100));
            executor.spawn(
                client
                    .b_stream_say_hello(Request::new(s))
                    .and_then(|s| {
                        s.into_inner()
                            // .map(|resp| println!("RESPONSE = {:?}", resp))
                            .map(|_resp| ())
                            .fold(0, |count, _| future::ok::<_, Status>(count + 1))
                            .map(|count| println!("All {} responses are received", count))
                    })
                    .map_err(|e| panic!("gRPC failed; err={:?}", e)),
            );
            client
        })
}

fn new_request(name: String) -> HelloRequest {
    HelloRequest { name }
}
