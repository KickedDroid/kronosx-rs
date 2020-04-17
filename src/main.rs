use std::sync::Arc;
extern crate grpcio;

pub mod protos;
use protobuf::RepeatedField;
use grpcio::{ChannelBuilder, EnvBuilder};
use ::protos::replication_grpc::ReplicatorClient;
use ::protos::replication::{Replication,ServerSource, AddrInfo, SignedSubscription, Subscription};

fn main() {

    // Setup the gRPC Client
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("localhost:50051");
    let client = ReplicatorClient::new(ch);

    // Create a Replication request
    let mut req = Replication::default();

    // Collect cids into a vector TODO add iter
    let mut cids = Vec::new();
    let v = RepeatedField::from_vec(cids);
    req.set_cids_bytes(v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>);
    
    // Set Replication req params
    let header = "Example header";
    req.set_header(header.to_string());
    req.set_refresh_interval_seconds(10);
    req.set_replication_factor(4);
    req.set_server_down_delay_seconds(30);

    // TODO add iterator 
    let source = ServerSource::default();
    let addrInfo = AddrInfo::default();
    let mut addrVec = Vec::new();
    let addrs = RepeatedField::from_vec(addrVec);
    source.set_addr_info(addrInfo);
    addrInfo.set_addrs_bytes(addrs: ::protobuf::RepeatedField<::std::vec::Vec<u8>>);

    // Collect servers and sets req field
    let mut server = Vec::<ServerSource>::new();
    let servers = RepeatedField::from_vec(server);
    req.set_servers(servers: ::protobuf::RepeatedField<ServerSource>);

    let sub = Subscription::default();
    sub.set_author_id_bytes("aidan".as_bytes().to_vec());
    let reply = client.add(&sub);
    //info!("Greeter received: {}", reply.get_message());
}
