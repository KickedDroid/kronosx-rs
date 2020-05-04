use ::protos::node::{P2PREQTYPE,P2PRequest};
use ::protos::node_grpc::NodeApiClient;
use std::sync::Arc;
use grpcio::{ChannelBuilder, EnvBuilder};
use protobuf::RepeatedField;

pub fn create_p2p_req() -> P2PRequest {
    let mut p2p_req = P2PRequest::default();
    let req_type = P2PREQTYPE::LS;
    p2p_req.set_requestType(req_type);
    p2p_req
}