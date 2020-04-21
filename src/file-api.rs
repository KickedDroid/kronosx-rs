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

fn main() {
        // Setup the gRPC Client
        let env = Arc::new(EnvBuilder::new().build());
        let ch = ChannelBuilder::new(env).connect("localhost:50051");
        let client = FileApiClient::new(ch);

        let up_req = UploadRequest::default();
        let up_opts = UploadOptions::default();

        up_opts.set_multihash("sha256");
        up_opts.set_layout("balanced");
        up_opts.set_chunker("default");

        up_req.set_options(up_opts);

        let blob = Blob::default();
        let data = "hello".as_bytes();
        blob.set_content(data);

        up_req.set_blob(blob);

        client.upload_file(up_req);
}