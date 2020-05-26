use ::protos::pubsub::{PubSubRequest, PSREQTYPE, PubSubResponse};
use ::protos::pubsub_grpc::PubSubApiClient;
use futures::prelude::*;
use futures::future::poll_fn;
use grpcio::{ChannelBuilder, EnvBuilder, WriteFlags};
use protobuf::RepeatedField;
use std::sync::Arc;
use std::thread;
use async_std::task;

fn create_pubsub_client() -> PubSubApiClient {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("xapi.temporal.cloud:9090");
    let cli = PubSubApiClient::new(ch);
    cli
}

pub async fn ps_sub(topics: Vec<String>) {
    let cli = create_pubsub_client();
    let (mut sink, mut receiver) = cli.pub_sub().expect("RPC Failed");

    let send = task::spawn(async move {
        let req = {
            let mut req = PubSubRequest::default();
            let req_type = PSREQTYPE::PS_SUBSCRIBE;
            req.set_requestType(req_type);
            let v = RepeatedField::from_vec(topics);
            req.set_topics(v);
            req
        };


        sink = sink.send((req, WriteFlags::default())).wait().unwrap();
        println!("Start Send");
        sink.close().unwrap();
    });


    loop {
        match receiver.into_future().wait() {
            Ok((Some(item), r)) => {
                let msg = item.get_message();
                receiver = r;
                println!("{:?}", msg);
            },
            Ok((None, _)) => break,
            Err((e, _)) => return eprintln!("{:?}", e),
        }
    }

    task::block_on(send);
}


pub async fn ps_pub(topics: Vec<String>, data: Vec<u8>) {
    let cli = create_pubsub_client();
    let (mut sink, mut receiver) = cli.pub_sub().expect("RPC Failed");

    let send = task::spawn(async move {
        let req = {
            let mut req = PubSubRequest::default();
            let req_type = PSREQTYPE::PS_PUBLISH;
            req.set_requestType(req_type);
            let v = RepeatedField::from_vec(topics);
            req.set_topics(v);
            req.set_data(data);
            req
        };
        sink = sink.send((req, WriteFlags::default())).wait().unwrap();
        println!("Start Send");
        sink.close().unwrap();
    });

    loop {
        match receiver.into_future().wait() {
            Ok((Some(item), r)) => {
                let r_type = item.get_requestType();
                receiver = r;
                println!("{:?}", r_type);
                assert_eq!(r_type, PSREQTYPE::PS_PUBLISH);
            },
            Ok((None, _)) => break,
            Err((e, _)) => return eprintln!("{:?}", e),
        }
    }
    task::block_on(send);
}

pub async fn ps_peers(topics: Vec<String>) {
    let cli = create_pubsub_client();
    let (mut sink, mut receiver) = cli.pub_sub().expect("RPC Failed");

    let send = task::spawn(async move {
        let req = {
            let mut req = PubSubRequest::default();
            let req_type = PSREQTYPE::PS_LIST_PEERS;
            req.set_requestType(req_type);
            let v = RepeatedField::from_vec(topics);
            req.set_topics(v);
            req
        };
        sink = sink.send((req, WriteFlags::default())).wait().unwrap();
        println!("Start Send");
        sink.close().unwrap();
    });

    loop {
        match receiver.into_future().wait() {
            Ok((Some(item), r)) => {
                let peers = item.get_peers();
                receiver = r;
                for peer in peers {
                    println!("{:?}", peer)
                }
            },
            Ok((None, _)) => break,
            Err((e, _)) => return eprintln!("{:?}", e),
        }
    }
    task::block_on(send);
}