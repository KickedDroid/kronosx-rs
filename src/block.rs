#![allow(dead_code)]
use ::protos::node::{BlockstoreRequest, BSREQTYPE};
use ::protos::node_grpc::NodeApiClient;
use std::sync::Arc;
use grpcio::{ChannelBuilder, EnvBuilder};
use protobuf::RepeatedField;
use cid::{Cid, Codec, Version};
use multihash::Sha2_256;

pub fn new_node_client() -> NodeApiClient {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("xapi.temporal.cloud:9090");
    let client = NodeApiClient::new(ch);
    client
}

fn create_block_req() -> BlockstoreRequest {
    let mut block_req = BlockstoreRequest::default();
    let req_type = BSREQTYPE::BS_PUT;
    block_req.set_requestType(req_type);

    block_req.set_cidVersion("v1".to_string());
    block_req.set_hashFunc("sha2-256".to_string());
    block_req
}

pub fn put_block(data: Vec<std::vec::Vec<u8>>) {
    let cli = new_node_client();
    let v = RepeatedField::from_vec(data);
    let mut block_req = create_block_req();
    block_req.set_data(v);
    let h = Sha2_256::digest(b"hey bruh");
    println!("{:?}", h);
    let cid = Cid::new(Version::V1, Codec::DagProtobuf, h).unwrap();
    cli.blockstore(&block_req).expect("RPC Failed");
    print!("{:?}", String::from_utf8(cid.hash().as_bytes().to_vec()));
    
}

pub fn get_block() {
    let cli = new_node_client();
    let mut block_req = BlockstoreRequest::default();
    let req_type = BSREQTYPE::BS_GET;
    block_req.set_requestType(req_type);
    let mut cid = Vec::new();
    cid.push(String::from("bafkreib2wkausqyegptb7m7vhegd3oiqdzhwnhb7xxkipqifi3623u7feq"));
    let v = RepeatedField::from_vec(cid);
    block_req.set_cids(v);

    let resp = cli.blockstore(&block_req).expect("RPC Failed");
    for block in resp.get_blocks() {
        println!("{:?}", block)
    }
}