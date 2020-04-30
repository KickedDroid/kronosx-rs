use futures::Future;
use std::sync::Arc;
extern crate grpcio;
extern crate cryptoxide;


use grpcio::{ChannelBuilder, EnvBuilder};
use ::protos::file_grpc::FileApiClient;
use ::protos::file::{ UploadRequest, UploadOptions, Blob};
 

fn main() {

        let client = new_file_client();

        let mut up_req = create_upload_req();

        let mut blob = Blob::default();
        let data = "hello".as_bytes();
        blob.set_content(data.to_vec());
        println!("Set blob");

        up_req.set_blob(blob);

        let (_resp, s) = client.upload_file().expect("RPC Failed");

        match s.wait() {
                Err(e) => panic!("{:?}", e),
                Ok(item) => {
                   let hash = item.get_hash();
                   println!("{:?}", hash)
                } 
        }

}

// Creates a default Upload Request
pub fn create_upload_req() -> UploadRequest {
        let mut up_req = UploadRequest::default();
        let mut up_opts = UploadOptions::default();

        up_opts.set_multiHash(String::from("sha256"));
        up_opts.set_layout(String::from("balanced"));
        up_opts.set_chunker(String::from("default"));

        up_req.set_options(up_opts);
        print!("Created request");
        up_req
}

// Creates a new file api Client
pub fn new_file_client() -> FileApiClient {
        let env = Arc::new(EnvBuilder::new().build());
        let ch = ChannelBuilder::new(env).connect("xapi.temporal.cloud:9090");
        let client = FileApiClient::new(ch);
        client
}