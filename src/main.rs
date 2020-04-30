use futures::Future;

extern crate grpcio;
extern crate cryptoxide;


pub mod protos;




mod fileapi;
use fileapi::{create_upload_req, new_file_client};
mod rep;
use rep::{get_signature, new_rep_client};

fn main() {
        
    let file_cli = new_file_client();
    let rep_cli = new_rep_client();

}
