#![allow(unused_imports)]
use futures::{Future, Sink};
extern crate cryptoxide;
extern crate grpcio;
pub mod protos;
use std::collections::HashMap;
mod fileapi;
use fileapi::{upload_file};
mod rep;
use rep::{get_signature, new_rep_client};
mod statusapi;
use statusapi::{create_empty_req, new_status_client};
mod block;
use block::{get_block, new_node_client, put_block};
mod p2p;
use p2p::{get_peers, get_connection_stats};
mod pubsub;
use pubsub::ps_sub;
mod dag;
use dag::{add_dag_links, get_dag, get_dag_links, get_dag_many, put_dag};

fn main() {
    let cid = "bafkreib2wkausqyegptb7m7vhegd3oiqdzhwnhb7xxkipqifi3623u7feq";
    let cid2 = "bafkreiadrrjhot4osxtgb6voecvgfwabmjesgfdblh6iqgqt3l6l6soz4y";
    let data = "Wassaup dude".as_bytes();

    //let mut cids = Vec::default();
    //cids.push(cid.to_string());
    //cids.push(cid2.to_string());
    //get_dag_many(&cids);

    //get_block();
    ps_sub(String::from("Aye"));
    get_peers();
    get_connection_stats();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_status_api() {
        // Create a StatusAPiClient
        let stat_cli = new_status_client();

        let req = create_empty_req();
        let stat_resp = stat_cli.version(&req).expect("RPC Failed");
        let stat_resp2 = stat_cli.status(&req).expect("RPC Failed");
        let version = stat_resp.get_version();
        let stats = stat_resp2.get_status();

        // Ex. ONLINE
        // "v3.4.0-1-ge94271e"
        println!("{:?}", stats);
        println!("{:?}", version);
    }

    #[test]
    fn block_test() {
        let mut data = Vec::new();
        data.push("hey bruh".as_bytes().to_vec());

        put_block(data);

        get_block();
    }

    #[test]
    fn dag_put_test() {
        let data = "Hey Bruh".as_bytes();
        put_dag(data);
    }

    #[test]
    fn dag_get_test() {
        let cid = "bafkreib2wkausqyegptb7m7vhegd3oiqdzhwnhb7xxkipqifi3623u7feq";
        get_dag(cid.to_string());
    }

    #[test]
    fn dag_get_many_test() {
        let cid = "bafkreib2wkausqyegptb7m7vhegd3oiqdzhwnhb7xxkipqifi3623u7feq";
        let cid2 = "bafkreiadrrjhot4osxtgb6voecvgfwabmjesgfdblh6iqgqt3l6l6soz4y";

        let mut cids = Vec::default();
        cids.push(cid.to_string());
        cids.push(cid2.to_string());
        get_dag_many(&cids);
    }

    #[test]
    fn get_peers_test() {

        get_peers();
    }
}
