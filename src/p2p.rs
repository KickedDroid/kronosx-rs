use ::protos::node::{P2PREQTYPE,P2PRequest, CONNMGMTREQTYPE, ConnMgmtRequest};
use ::protos::node_grpc::NodeApiClient;
use std::sync::Arc;
use grpcio::{ChannelBuilder, EnvBuilder};
use protobuf::RepeatedField;

fn new_node_client() -> NodeApiClient {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("xapi.temporal.cloud:9090");
    let client = NodeApiClient::new(ch);
    client
}

fn create_p2p_req() -> P2PRequest {
    let mut p2p_req = P2PRequest::default();
    let req_type = P2PREQTYPE::LS;
    p2p_req.set_requestType(req_type);
    p2p_req
}

pub fn create_conmgmt_req() -> ConnMgmtRequest {
    let p2p_req = ConnMgmtRequest::default();
    p2p_req
}

pub fn get_peers() {
    let cli = new_node_client();
    let mut p2p_req = create_conmgmt_req();
    let req_type = CONNMGMTREQTYPE::CM_GET_PEERS;
    p2p_req.set_requestType(req_type);
    let resp = cli.conn_mgmt(&p2p_req).expect("RPC Failed");
    let peers = resp.get_peerIDs();
    let cons = resp.get_connected();
    for peer in peers {
        println!("{:?}", peer);
    }
    println!("{:?}", cons);

}

pub fn get_connection_stats() {
    let cli = new_node_client();
    let mut p2p_req = create_conmgmt_req();
    let req_type = CONNMGMTREQTYPE::CM_STATUS;
    p2p_req.set_requestType(req_type);
    let resp = cli.conn_mgmt(&p2p_req).expect("RPC Failed");
    for status in resp.get_status() {
        println!("{:?}" , status)
    }
}