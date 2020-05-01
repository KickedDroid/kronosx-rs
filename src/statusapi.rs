use std::sync::Arc;
use grpcio::{ChannelBuilder, EnvBuilder};
use ::protos::status_grpc::StatusApiClient;
use ::protos::status;
use ::protos::util::Empty;

pub fn new_status_client() -> StatusApiClient {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("xapi.temporal.cloud:9090");
    let client = StatusApiClient::new(ch);
    client
}

pub fn create_empty_req() -> Empty {
    let req = Empty::default();
    req
}