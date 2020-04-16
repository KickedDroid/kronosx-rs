use std::sync::Arc;
extern crate grpcio;

pub mod protos;

use grpcio::{ChannelBuilder, EnvBuilder};
use protos::replication_grpc::ReplicatorClient;

fn main() {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("localhost:50051");
    let client = ReplicatorClient::new(ch);

    let mut req = HelloRequest::default();
    req.set_name("world".to_owned());
    let reply = client.say_hello(&req).expect("rpc");
    info!("Greeter received: {}", reply.get_message());
}