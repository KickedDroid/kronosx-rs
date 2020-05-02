use futures::{Future, Sink};

extern crate cryptoxide;
extern crate grpcio;
use cid::{Cid, Codec, Version};
use multihash::Sha2_256;
pub mod protos;

mod fileapi;
use fileapi::{create_upload_req, new_file_client};
mod rep;
use rep::{get_signature, new_rep_client};
mod statusapi;
use statusapi::{create_empty_req, new_status_client};
mod block;
use block::{new_node_client, create_block_req};

fn main() {
    let _file_cli = new_file_client();
    let _rep_cli = new_rep_client();
    let _stat_cli = new_status_client();

    let block_cli = new_node_client();

    let block_req = create_block_req();
    block_cli.blockstore(&block_req).expect("RPC Failed");

    println!("DOne?");

    let cid = Cid::new_v1(Codec::Raw, Sha2_256::digest(&data));


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
}
