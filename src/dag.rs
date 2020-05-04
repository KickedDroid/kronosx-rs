use ::protos::node::{DagRequest, DAGREQTYPE};
use ::protos::node_grpc::NodeApiClient;
use std::sync::Arc;
use grpcio::{ChannelBuilder, EnvBuilder};
use protobuf::RepeatedField;

pub fn create_dag_service() -> NodeApiClient {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("xapi.temporal.cloud:9090");
    let client = NodeApiClient::new(ch);
    client
}

pub fn put_dag(cli: NodeApiClient, data: &[u8]) {
    let mut dag_req = DagRequest::default();
    let req_type = DAGREQTYPE::DAG_PUT;
    dag_req.set_requestType(req_type);

    dag_req.set_data(data.to_vec());
    let resp = cli.dag(&dag_req).expect("RPC Failed");
    let hashes = resp.get_hashes();
    for hash in hashes {
        println!("{:?}", hash);
    }
}

pub fn get_dag(cli: NodeApiClient, cid: String) {
    let mut dag_req = DagRequest::default();
    let req_type = DAGREQTYPE::DAG_GET;
    dag_req.set_requestType(req_type);
    dag_req.set_hash(String::from(cid));

    let resp = cli.dag(&dag_req).expect("RPC Failed");
    let r_data = resp.get_rawData();
    println!("{:?}", String::from_utf8(r_data.to_vec()));
}