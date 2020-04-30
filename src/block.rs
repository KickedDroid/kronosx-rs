use ::protos::node{BlockstoreRequest};
use ::protos::node_grpc::NodeApiClient;

pub fn new_node_client() -> NodeApiClient {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("localhost:50051");
    let client = NodeApiClient::new(ch);
    client
}