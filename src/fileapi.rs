use futures::prelude::*;
use std::sync::Arc;
extern crate cryptoxide;
extern crate grpcio;
use ::protos::file::{Blob, UploadOptions, UploadRequest};
use ::protos::file_grpc::FileApiClient;
use grpcio::{ChannelBuilder, EnvBuilder, WriteFlags};
use std::thread;


// Creates a default Upload Request
fn create_upload_req() -> UploadRequest {
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
fn new_file_client() -> FileApiClient {
        let env = Arc::new(EnvBuilder::new().build());
        let ch = ChannelBuilder::new(env).connect("xapi.temporal.cloud:9090");
        let client = FileApiClient::new(ch);
        client
}

pub async fn upload_file() {
        let cli = new_file_client();
        let mut req = create_upload_req();

        let (mut sink, receiver) = cli.upload_file().expect("RPC Failed");

        let send = async move {
                let mut blob = Blob::default();
                let data = "hello".as_bytes();
                blob.set_content(data.to_vec());
                req.set_blob(blob);

                sink = sink.send((req, WriteFlags::default())).wait().unwrap();
                sink.close().unwrap();
        };
        send.await;
        
        match receiver.wait() {
                Err(e) => panic!("{:?}", e),
                Ok(item) => {
                        let hash = item.get_hash();
                        println!("{:?}", hash)
                }
        }
}
