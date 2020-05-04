use ::protos::node::{DagRequest, DAGREQTYPE, DagResponse};
use ::protos::node_grpc::NodeApiClient;
use std::sync::Arc;
use grpcio::{ChannelBuilder, EnvBuilder};


pub fn create_dag_service() -> NodeApiClient {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("xapi.temporal.cloud:9090");
    let client = NodeApiClient::new(ch);
    client
}

pub fn put_dag(data: &[u8]) {
    let cli = create_dag_service();
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

pub fn get_dag(cid: String) {
    let cli = create_dag_service();
    let mut dag_req = DagRequest::default();
    let req_type = DAGREQTYPE::DAG_GET;
    dag_req.set_requestType(req_type);
    dag_req.set_hash(String::from(cid));

    let resp = cli.dag(&dag_req).expect("RPC Failed");
    let r_data = resp.get_rawData();
    let links = resp.get_links();
    for link in links {
        let hash = link.get_hash();
        let name = link.get_name();
        println!("{:?}", name);
        println!("{:?}", String::from_utf8(hash.to_vec()))
    }
    println!("{:?}", String::from_utf8(r_data.to_vec()));
    
}

pub fn get_dag_links(cid: String) -> DagResponse {

    let cli = create_dag_service();
    let mut dag_req = DagRequest::default();
    let req_type = DAGREQTYPE::DAG_GET_LINKS;
    dag_req.set_requestType(req_type);
    dag_req.set_hash(String::from(cid));

    let resp = cli.dag(&dag_req).expect("RPC Failed");
    resp
}

pub fn add_dag_links(links: ::std::collections::HashMap<::std::string::String, ::std::string::String>, cid: String) -> protos::node::DagResponse {
    let cli = create_dag_service();
    let mut dag_req = DagRequest::default();
    let req_type = DAGREQTYPE::DAG_ADD_LINKS;
    dag_req.set_requestType(req_type);
    dag_req.set_hash(String::from(cid));
    dag_req.set_links(links);

    let resp = cli.dag(&dag_req).expect("RPC Failed");
    resp
}

pub fn get_dag_many(cids: &Vec<String>) {
    
    for cid in cids {
        get_dag((&cid).to_string());
    }
}