use std::sync::Arc;
extern crate grpcio;
extern crate cryptoxide;
use bytes::{BytesMut, BufMut};
use cryptoxide::ed25519;
use protobuf::RepeatedField;
use grpcio::{ChannelBuilder, EnvBuilder};
use ::protos::replication_grpc::ReplicatorClient;
use ::protos::replication::{Replication,ServerSource, AddrInfo, SignedSubscription, Subscription, SubscriptionUpdate, ReplicationStatus};

fn main() {

    let rep_cli = new_rep_client();

    // Create a Replication request
    let mut req = Replication::default();

    // Collect cids into a vector TODO add iter
    let cids = Vec::new();
    let v = RepeatedField::from_vec(cids);
    req.set_cids_bytes(v);
    
    // Set Replication req params
    let header = "Example header";
    req.set_header(header.to_string());
    req.set_refresh_interval_seconds(10);
    req.set_replication_factor(4);
    req.set_server_down_delay_seconds(30);


    // TODO add iterator 
    let mut source = ServerSource::default();
    let mut addr_info = AddrInfo::default();
    let addr_vec = Vec::new();
    let addrs = RepeatedField::from_vec(addr_vec);
    addr_info.set_addrs_bytes(addrs);
    source.set_addr_info(addr_info);
    

    // Collect servers and sets req field
    let server = Vec::<ServerSource>::new();
    let servers = RepeatedField::from_vec(server);
    req.set_servers(servers);
    
    let mut rep_bytes = {
        let mut buf = BytesMut::default();
    };
     
    let mut sub = Subscription::default();
    sub.set_author_id_bytes("aidan".as_bytes().to_vec());
    sub.set_topic(String::from("bruh"));

    let mut ss = SignedSubscription::default();
    
    let subup = get_signature(sub);
    ss.set_update_part(subup);
    let reply = rep_cli.submit_replication(&ss).expect("RPC Failed");


}

// Sign the SubcriptionUpdate and return a SubcriptionUpdate
pub fn get_signature(sub: Subscription ) -> SubscriptionUpdate {

    let mut subup = SubscriptionUpdate::default();
    let data = {
        let topic = sub.get_topic().as_bytes();
        let author = sub.get_author_id_bytes();
        let version = subup.get_version();
        let mut buf = BytesMut::default();
        buf.put(topic);
        buf.put(author);
        buf.put_i64(version);
        buf
    };
    let seed = [0u8;32]; // seed only for example !
    let (secret, public) = ed25519::keypair(&seed[..]);
    let sig = ed25519::signature(&data, &secret[..]);
    subup.set_signature(sig.to_vec());
    subup

}

// Creates a new Replication Client
pub fn new_rep_client() -> ReplicatorClient {
    // Setup the gRPC Client
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("localhost:50051");
    let client = ReplicatorClient::new(ch);
    client
}