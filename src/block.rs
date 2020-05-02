use ::protos::node::{BlockstoreRequest, DagRequest, BSREQTYPE};
use ::protos::node_grpc::NodeApiClient;
use std::sync::Arc;
use grpcio::{ChannelBuilder, EnvBuilder};
use protobuf::RepeatedField;

pub fn new_node_client() -> NodeApiClient {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("xapi.temporal.cloud:9090");
    let client = NodeApiClient::new(ch);
    client
}

pub fn create_block_req() -> BlockstoreRequest {
    let mut block_req = BlockstoreRequest::default();
    let req_type = BSREQTYPE::BS_PUT;
    block_req.set_requestType(req_type);

    let mut data = Vec::new();
    data.push("hey bruh".as_bytes().to_vec());
    let v = RepeatedField::from_vec(data);
    block_req.set_data(v);

    block_req.set_cidVersion("v1".to_string());
    block_req.set_hashFunc("sha2-256".to_string());
    block_req
}