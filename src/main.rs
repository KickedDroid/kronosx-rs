use futures::future::result;
use futures::{Future, Stream};
use std::sync::Arc;
extern crate grpcio;
extern crate cryptoxide;
use bytes::{BytesMut, BufMut};
use cryptoxide::ed25519;
pub mod protos;
use protobuf::RepeatedField;
use grpcio::{ChannelBuilder, EnvBuilder};
use ::protos::file_grpc::FileApiClient;
use ::protos::file::{ UploadRequest, UploadOptions, DownloadRequest, DownloadResponse, Blob};
use ::protos::util::PutResponse; 

fn main() {
        // Setup the gRPC Client
        let env = Arc::new(EnvBuilder::new().build());
        let ch = ChannelBuilder::new(env).connect("xapi.temporal.cloud:9090");
        let client = FileApiClient::new(ch);

        let mut up_req = UploadRequest::default();
        let mut up_opts = UploadOptions::default();

        up_opts.set_multiHash(String::from("sha256"));
        up_opts.set_layout(String::from("balanced"));
        up_opts.set_chunker(String::from("default"));

        up_req.set_options(up_opts);

        let mut blob = Blob::default();
        let data = "hello".as_bytes();
        blob.set_content(data.to_vec());

        up_req.set_blob(blob);

        let (_resp, s) = client.upload_file().expect("RPC Failed");
        match s.wait() {
                Err(e) => panic!("{:?}", e),
                Ok(item) => {
                   let hash = item.get_hash();
                   println!("{:?}", hash)
                } 
        };


        

        println!("Done?");

        let mut down_req = DownloadRequest::default();
        down_req.set_hash(String::from("bafybeifx7yeb55armcsxwwitkymga5xf53dxiarykms3ygqic223w5sk3m"));
        let down_resp = client.download_file(&down_req).expect("RPC Failed");
        match down_resp.wait() {
                Err(e) => panic!("{:?}", e),
                Ok(r) => {
                        r.get_blob();
                }
        }
        down_resp.get_blob();
        
        let r = DownloadResponse::default();
        let msg = r.get_blob();
        println!("{:?}", msg);
}