extern crate tower_grpc_build;
use std::env;

fn main() {
    env::set_var("OUT_DIR", "src");
    // Build helloworld.
    tower_grpc_build::Config::new()
        .enable_server(true)
        .enable_client(true)
        .build(&["proto/helloworld.proto"], &["proto"])
        .unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e));
}
