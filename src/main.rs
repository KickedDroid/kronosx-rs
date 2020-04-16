use std::sync::Arc;
extern crate grpcio;

pub mod protos;
use protobuf::RepeatedField;
use grpcio::{ChannelBuilder, EnvBuilder};
use ::protos::replication_grpc::ReplicatorClient;
use ::protos::replication::Replication;

fn main() {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("localhost:50051");
    let client = ReplicatorClient::new(ch);

    let mut req = Replication::default();
    let bytes = "wbrbrb".as_bytes();
    let v = RepeatedField::default();
    v.to_vec();
    req.set_cids_bytes(v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>);
    let reply = client.say_hello(&req).expect("rpc");
    info!("Greeter received: {}", reply.get_message());
}