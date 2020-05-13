use ::protos::pubsub::{PubSubRequest, PSREQTYPE, PubSubResponse};
use ::protos::pubsub_grpc::PubSubApiClient;
use futures::prelude::*;
use grpcio::{ChannelBuilder, EnvBuilder, WriteFlags};
use protobuf::RepeatedField;
use std::sync::Arc;
use std::thread;

fn create_pubsub_client() -> PubSubApiClient {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("xapi.temporal.cloud:9090");
    let cli = PubSubApiClient::new(ch);
    cli
}

pub async fn ps_sub(topic: String) {
    let cli = create_pubsub_client();
    let (mut sink, mut receiver) = cli.pub_sub().expect("RPC Failed");
    
    let send = async move {
        let req = {
            let mut req = PubSubRequest::default();
            let req_type = PSREQTYPE::PS_SUBSCRIBE;
            req.set_requestType(req_type);
            let mut topics = Vec::new();
            topics.push(String::from(
                topic,
            ));
            let v = RepeatedField::from_vec(topics);
            req.set_topics(v);
            req
        };
        sink = sink.send((req, WriteFlags::default())).wait().unwrap();
        sink.close().unwrap();

    };

    loop {
        match receiver.into_future().wait() {
            Ok((Some(item), r)) => {
                item.get_message();
                receiver = r;
            },
            Ok((None, _)) => break,
            Err((e, _)) => return eprintln!("{:?}", e),
        }
    }
}
