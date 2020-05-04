use futures::{Future, Sink};

extern crate cryptoxide;
extern crate grpcio;
pub mod protos;

mod fileapi;
use fileapi::{create_upload_req, new_file_client};
mod rep;
use rep::{get_signature, new_rep_client};
mod statusapi;
use statusapi::{create_empty_req, new_status_client};
mod block;
use block::{get_block, new_node_client, put_block};
mod p2p;
use p2p::create_p2p_req;
mod dag;
use dag::{create_dag_service, put_dag, get_dag};

fn main() {
    let dag_service = create_dag_service();
    let cid = "bafkreib2wkausqyegptb7m7vhegd3oiqdzhwnhb7xxkipqifi3623u7feq";
    get_dag(dag_service, cid.to_string());
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
        let block_cli = new_node_client();

        let mut data = Vec::new();
        data.push("hey bruh".as_bytes().to_vec());

        //put_block(block_cli, data);

        get_block(block_cli);
    }

    #[test]
    fn dag_put_test() {
        let dag_service = create_dag_service();
        let data = "Hey Bruh".as_bytes();
        put_dag(dag_service, data);
    }

    #[test]
    fn dag_get_test() {
        let dag_service = create_dag_service();
        let cid = "bafkreib2wkausqyegptb7m7vhegd3oiqdzhwnhb7xxkipqifi3623u7feq";
        get_dag(dag_service, cid.to_string());
    }
}
