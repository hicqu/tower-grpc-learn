#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloRequest {
    #[prost(string, tag="1")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloReply {
    #[prost(string, tag="1")]
    pub reply: std::string::String,
}
pub mod client {
    use ::tower_grpc::codegen::client::*;
    use super::{HelloRequest, HelloReply};

    #[derive(Debug, Clone)]
    pub struct Greeter<T> {
        inner: grpc::Grpc<T>,
    }

    impl<T> Greeter<T> {
        pub fn new(inner: T) -> Self {
            let inner = grpc::Grpc::new(inner);
            Self { inner }
        }

        /// Poll whether this client is ready to send another request.
        pub fn poll_ready<R>(&mut self) -> futures::Poll<(), grpc::Status>
        where T: grpc::GrpcService<R>,
        {
            self.inner.poll_ready()
        }

        /// Get a `Future` of when this client is ready to send another request.
        pub fn ready<R>(self) -> impl futures::Future<Item = Self, Error = grpc::Status>
        where T: grpc::GrpcService<R>,
        {
            futures::Future::map(self.inner.ready(), |inner| Self { inner })
        }

        pub fn say_hello<R>(&mut self, request: grpc::Request<HelloRequest>) -> grpc::unary::ResponseFuture<HelloReply, T::Future, T::ResponseBody>
        where T: grpc::GrpcService<R>,
              grpc::unary::Once<HelloRequest>: grpc::Encodable<R>,
        {
            let path = http::PathAndQuery::from_static("/helloworld.Greeter/SayHello");
            self.inner.unary(request, path)
        }

        pub fn c_stream_say_hello<R, B>(&mut self, request: grpc::Request<B>) -> grpc::client_streaming::ResponseFuture<HelloReply, T::Future, T::ResponseBody>
        where T: grpc::GrpcService<R>,
              B: futures::Stream<Item = HelloRequest>,
              B: grpc::Encodable<R>,
        {
            let path = http::PathAndQuery::from_static("/helloworld.Greeter/CStreamSayHello");
            self.inner.client_streaming(request, path)
        }

        pub fn s_stream_say_hello<R>(&mut self, request: grpc::Request<HelloRequest>) -> grpc::server_streaming::ResponseFuture<HelloReply, T::Future>
        where T: grpc::GrpcService<R>,
              grpc::unary::Once<HelloRequest>: grpc::Encodable<R>,
        {
            let path = http::PathAndQuery::from_static("/helloworld.Greeter/SStreamSayHello");
            self.inner.server_streaming(request, path)
        }

        pub fn b_stream_say_hello<R, B>(&mut self, request: grpc::Request<B>) -> grpc::streaming::ResponseFuture<HelloReply, T::Future>
        where T: grpc::GrpcService<R>,
              B: futures::Stream<Item = HelloRequest>,
              B: grpc::Encodable<R>,
        {
            let path = http::PathAndQuery::from_static("/helloworld.Greeter/BStreamSayHello");
            self.inner.streaming(request, path)
        }
    }
}

pub mod server {
    use ::tower_grpc::codegen::server::*;
    use super::{HelloRequest, HelloReply};

    // Redefine the try_ready macro so that it doesn't need to be explicitly
    // imported by the user of this generated code.
    macro_rules! try_ready {
        ($e:expr) => (match $e {
            Ok(futures::Async::Ready(t)) => t,
            Ok(futures::Async::NotReady) => return Ok(futures::Async::NotReady),
            Err(e) => return Err(From::from(e)),
        })
    }

    pub trait Greeter: Clone {
        type SayHelloFuture: futures::Future<Item = grpc::Response<HelloReply>, Error = grpc::Status>;
        type CStreamSayHelloFuture: futures::Future<Item = grpc::Response<HelloReply>, Error = grpc::Status>;
        type SStreamSayHelloStream: futures::Stream<Item = HelloReply, Error = grpc::Status>;
        type SStreamSayHelloFuture: futures::Future<Item = grpc::Response<Self::SStreamSayHelloStream>, Error = grpc::Status>;
        type BStreamSayHelloStream: futures::Stream<Item = HelloReply, Error = grpc::Status>;
        type BStreamSayHelloFuture: futures::Future<Item = grpc::Response<Self::BStreamSayHelloStream>, Error = grpc::Status>;

        fn say_hello(&mut self, request: grpc::Request<HelloRequest>) -> Self::SayHelloFuture;

        fn c_stream_say_hello(&mut self, request: grpc::Request<grpc::Streaming<HelloRequest>>) -> Self::CStreamSayHelloFuture;

        fn s_stream_say_hello(&mut self, request: grpc::Request<HelloRequest>) -> Self::SStreamSayHelloFuture;

        fn b_stream_say_hello(&mut self, request: grpc::Request<grpc::Streaming<HelloRequest>>) -> Self::BStreamSayHelloFuture;
    }

    #[derive(Debug, Clone)]
    pub struct GreeterServer<T> {
        greeter: T,
    }

    impl<T> GreeterServer<T>
    where T: Greeter,
    {
        pub fn new(greeter: T) -> Self {
            Self { greeter }
        }
    }

    impl<T> tower::Service<http::Request<grpc::BoxBody>> for GreeterServer<T>
    where T: Greeter,
    {
        type Response = http::Response<greeter::ResponseBody<T>>;
        type Error = grpc::Never;
        type Future = greeter::ResponseFuture<T>;

        fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
            Ok(().into())
        }

        fn call(&mut self, request: http::Request<grpc::BoxBody>) -> Self::Future {
            use self::greeter::Kind::*;

            match request.uri().path() {
                "/helloworld.Greeter/SayHello" => {
                    let service = greeter::methods::SayHello(self.greeter.clone());
                    let response = grpc::unary(service, request);
                    greeter::ResponseFuture { kind: SayHello(response) }
                }
                "/helloworld.Greeter/CStreamSayHello" => {
                    let mut service = greeter::methods::CStreamSayHello(self.greeter.clone());
                    let response = grpc::client_streaming(&mut service, request);
                    greeter::ResponseFuture { kind: CStreamSayHello(response) }
                }
                "/helloworld.Greeter/SStreamSayHello" => {
                    let service = greeter::methods::SStreamSayHello(self.greeter.clone());
                    let response = grpc::server_streaming(service, request);
                    greeter::ResponseFuture { kind: SStreamSayHello(response) }
                }
                "/helloworld.Greeter/BStreamSayHello" => {
                    let mut service = greeter::methods::BStreamSayHello(self.greeter.clone());
                    let response = grpc::streaming(&mut service, request);
                    greeter::ResponseFuture { kind: BStreamSayHello(response) }
                }
                _ => {
                    greeter::ResponseFuture { kind: __Generated__Unimplemented(grpc::unimplemented(format!("unknown service: {:?}", request.uri().path()))) }
                }
            }
        }
    }

    impl<T> tower::Service<()> for GreeterServer<T>
    where T: Greeter,
    {
        type Response = Self;
        type Error = grpc::Never;
        type Future = futures::FutureResult<Self::Response, Self::Error>;

        fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
            Ok(futures::Async::Ready(()))
        }

        fn call(&mut self, _target: ()) -> Self::Future {
            futures::ok(self.clone())
        }
    }

    impl<T> tower::Service<http::Request<tower_hyper::Body>> for GreeterServer<T>
    where T: Greeter,
    {
        type Response = <Self as tower::Service<http::Request<grpc::BoxBody>>>::Response;
        type Error = <Self as tower::Service<http::Request<grpc::BoxBody>>>::Error;
        type Future = <Self as tower::Service<http::Request<grpc::BoxBody>>>::Future;

        fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
            tower::Service::<http::Request<grpc::BoxBody>>::poll_ready(self)
        }

        fn call(&mut self, request: http::Request<tower_hyper::Body>) -> Self::Future {
            let request = request.map(|b| grpc::BoxBody::map_from(b));
            tower::Service::<http::Request<grpc::BoxBody>>::call(self, request)
        }
    }

    pub mod greeter {
        use ::tower_grpc::codegen::server::*;
        use super::Greeter;
        use super::super::HelloRequest;

        pub struct ResponseFuture<T>
        where T: Greeter,
        {
            pub(super) kind: Kind<
                // SayHello
                grpc::unary::ResponseFuture<methods::SayHello<T>, grpc::BoxBody, HelloRequest>,
                // CStreamSayHello
                grpc::client_streaming::ResponseFuture<methods::CStreamSayHello<T>, grpc::Streaming<HelloRequest>>,
                // SStreamSayHello
                grpc::server_streaming::ResponseFuture<methods::SStreamSayHello<T>, grpc::BoxBody, HelloRequest>,
                // BStreamSayHello
                grpc::streaming::ResponseFuture<methods::BStreamSayHello<T>, grpc::Streaming<HelloRequest>>,
                // A generated catch-all for unimplemented service calls
                grpc::unimplemented::ResponseFuture,
            >,
        }

        impl<T> futures::Future for ResponseFuture<T>
        where T: Greeter,
        {
            type Item = http::Response<ResponseBody<T>>;
            type Error = grpc::Never;

            fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
                use self::Kind::*;

                match self.kind {
                    SayHello(ref mut fut) => {
                        let response = try_ready!(fut.poll());
                        let response = response.map(|body| {
                            ResponseBody { kind: SayHello(body) }
                        });
                        Ok(response.into())
                    }
                    CStreamSayHello(ref mut fut) => {
                        let response = try_ready!(fut.poll());
                        let response = response.map(|body| {
                            ResponseBody { kind: CStreamSayHello(body) }
                        });
                        Ok(response.into())
                    }
                    SStreamSayHello(ref mut fut) => {
                        let response = try_ready!(fut.poll());
                        let response = response.map(|body| {
                            ResponseBody { kind: SStreamSayHello(body) }
                        });
                        Ok(response.into())
                    }
                    BStreamSayHello(ref mut fut) => {
                        let response = try_ready!(fut.poll());
                        let response = response.map(|body| {
                            ResponseBody { kind: BStreamSayHello(body) }
                        });
                        Ok(response.into())
                    }
                    __Generated__Unimplemented(ref mut fut) => {
                        let response = try_ready!(fut.poll());
                        let response = response.map(|body| {
                            ResponseBody { kind: __Generated__Unimplemented(body) }
                        });
                        Ok(response.into())
                    }
                }
            }
        }

        pub struct ResponseBody<T>
        where T: Greeter,
        {
            pub(super) kind: Kind<
                // SayHello
                grpc::Encode<grpc::unary::Once<<methods::SayHello<T> as grpc::UnaryService<HelloRequest>>::Response>>,
                // CStreamSayHello
                grpc::Encode<grpc::unary::Once<<methods::CStreamSayHello<T> as grpc::ClientStreamingService<grpc::Streaming<HelloRequest>>>::Response>>,
                // SStreamSayHello
                grpc::Encode<<methods::SStreamSayHello<T> as grpc::ServerStreamingService<HelloRequest>>::ResponseStream>,
                // BStreamSayHello
                grpc::Encode<<methods::BStreamSayHello<T> as grpc::StreamingService<grpc::Streaming<HelloRequest>>>::ResponseStream>,
                // A generated catch-all for unimplemented service calls
                (),
            >,
        }

        impl<T> tower::HttpBody for ResponseBody<T>
        where T: Greeter,
        {
            type Data = <grpc::BoxBody as grpc::Body>::Data;
            type Error = grpc::Status;

            fn is_end_stream(&self) -> bool {
                use self::Kind::*;

                match self.kind {
                    SayHello(ref v) => v.is_end_stream(),
                    CStreamSayHello(ref v) => v.is_end_stream(),
                    SStreamSayHello(ref v) => v.is_end_stream(),
                    BStreamSayHello(ref v) => v.is_end_stream(),
                    __Generated__Unimplemented(_) => true,
                }
            }

            fn poll_data(&mut self) -> futures::Poll<Option<Self::Data>, Self::Error> {
                use self::Kind::*;

                match self.kind {
                    SayHello(ref mut v) => v.poll_data(),
                    CStreamSayHello(ref mut v) => v.poll_data(),
                    SStreamSayHello(ref mut v) => v.poll_data(),
                    BStreamSayHello(ref mut v) => v.poll_data(),
                    __Generated__Unimplemented(_) => Ok(None.into()),
                }
            }

            fn poll_trailers(&mut self) -> futures::Poll<Option<http::HeaderMap>, Self::Error> {
                use self::Kind::*;

                match self.kind {
                    SayHello(ref mut v) => v.poll_trailers(),
                    CStreamSayHello(ref mut v) => v.poll_trailers(),
                    SStreamSayHello(ref mut v) => v.poll_trailers(),
                    BStreamSayHello(ref mut v) => v.poll_trailers(),
                    __Generated__Unimplemented(_) => Ok(None.into()),
                }
            }
        }

        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone)]
        pub(super) enum Kind<SayHello, CStreamSayHello, SStreamSayHello, BStreamSayHello, __Generated__Unimplemented> {
            SayHello(SayHello),
            CStreamSayHello(CStreamSayHello),
            SStreamSayHello(SStreamSayHello),
            BStreamSayHello(BStreamSayHello),
            __Generated__Unimplemented(__Generated__Unimplemented),
        }

        pub mod methods {
            use ::tower_grpc::codegen::server::*;
            use super::super::{Greeter, HelloRequest, HelloReply};

            pub struct SayHello<T>(pub T);

            impl<T> tower::Service<grpc::Request<HelloRequest>> for SayHello<T>
            where T: Greeter,
            {
                type Response = grpc::Response<HelloReply>;
                type Error = grpc::Status;
                type Future = T::SayHelloFuture;

                fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
                    Ok(futures::Async::Ready(()))
                }

                fn call(&mut self, request: grpc::Request<HelloRequest>) -> Self::Future {
                    self.0.say_hello(request)
                }
            }

            pub struct CStreamSayHello<T>(pub T);

            impl<T> tower::Service<grpc::Request<grpc::Streaming<HelloRequest>>> for CStreamSayHello<T>
            where T: Greeter,
            {
                type Response = grpc::Response<HelloReply>;
                type Error = grpc::Status;
                type Future = T::CStreamSayHelloFuture;

                fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
                    Ok(futures::Async::Ready(()))
                }

                fn call(&mut self, request: grpc::Request<grpc::Streaming<HelloRequest>>) -> Self::Future {
                    self.0.c_stream_say_hello(request)
                }
            }

            pub struct SStreamSayHello<T>(pub T);

            impl<T> tower::Service<grpc::Request<HelloRequest>> for SStreamSayHello<T>
            where T: Greeter,
            {
                type Response = grpc::Response<T::SStreamSayHelloStream>;
                type Error = grpc::Status;
                type Future = T::SStreamSayHelloFuture;

                fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
                    Ok(futures::Async::Ready(()))
                }

                fn call(&mut self, request: grpc::Request<HelloRequest>) -> Self::Future {
                    self.0.s_stream_say_hello(request)
                }
            }

            pub struct BStreamSayHello<T>(pub T);

            impl<T> tower::Service<grpc::Request<grpc::Streaming<HelloRequest>>> for BStreamSayHello<T>
            where T: Greeter,
            {
                type Response = grpc::Response<T::BStreamSayHelloStream>;
                type Error = grpc::Status;
                type Future = T::BStreamSayHelloFuture;

                fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
                    Ok(futures::Async::Ready(()))
                }

                fn call(&mut self, request: grpc::Request<grpc::Streaming<HelloRequest>>) -> Self::Future {
                    self.0.b_stream_say_hello(request)
                }
            }
        }
    }
}
