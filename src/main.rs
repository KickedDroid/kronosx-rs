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

fn main() {
    let file_cli = new_file_client();
    let rep_cli = new_rep_client();
    let stat_cli = new_status_client();

    let req = create_empty_req();
    let stat_resp = stat_cli.version(&req).expect("RPC Failed");

    let version = stat_resp.get_version();
    println!("{:?}", version);

    let (mut req, resp) = file_cli.upload_file().expect("RPC Failed");

    let stream = req.wait();

    match resp.wait() {
        Err(e) => panic!("{:?}", e),
        Ok(item) => {
            let hash = item.get_hash();
            println!("{:?}", hash)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_status_api() {
        let stat_cli = new_status_client();

        let req = create_empty_req();
        let stat_resp = stat_cli.version(&req).expect("RPC Failed");

        let version = stat_resp.get_version();
        println!("{:?}", version);
    }
}
