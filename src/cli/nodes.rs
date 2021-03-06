use crate::state::State;
use futures::executor::block_on;
use nu_cli::{CommandArgs, CommandRegistry, OutputStream};
use nu_errors::ShellError;
use nu_protocol::{Signature, TaggedDictBuilder, UntaggedValue};
use nu_source::Tag;
use serde::Deserialize;
use std::fmt;
use std::sync::Arc;

pub struct Nodes {
    state: Arc<State>,
}

impl Nodes {
    pub fn new(state: Arc<State>) -> Self {
        Self { state }
    }
}

impl nu_cli::WholeStreamCommand for Nodes {
    fn name(&self) -> &str {
        "nodes"
    }

    fn signature(&self) -> Signature {
        Signature::build("nodes")
    }

    fn usage(&self) -> &str {
        "Lists all nodes of the connected cluster"
    }

    fn run(
        &self,
        _args: CommandArgs,
        _registry: &CommandRegistry,
    ) -> Result<OutputStream, ShellError> {
        block_on(nodes(self.state.clone()))
    }
}

async fn nodes(state: Arc<State>) -> Result<OutputStream, ShellError> {
    let client = reqwest::Client::new();

    // todo: hack! need to actually use proper hostname from a parsed connstr...
    let host = state.active_cluster().connstr().replace("couchbase://", "");
    let uri = format!("http://{}:8091/pools/default", host);

    let resp = client
        .get(&uri)
        .basic_auth(
            state.active_cluster().username(),
            Some(state.active_cluster().password()),
        )
        .send()
        .await
        .unwrap()
        .json::<PoolInfo>()
        .await
        .unwrap();

    let nodes = resp
        .nodes
        .into_iter()
        .map(|n| {
            let mut collected = TaggedDictBuilder::new(Tag::default());
            let services = n
                .services
                .iter()
                .map(|n| format!("{}", n))
                .collect::<Vec<_>>()
                .join(",");

            collected.insert_value("hostname", n.hostname);
            collected.insert_value("status", n.status);
            collected.insert_value("services", services);
            collected.insert_value("version", n.version);
            collected.insert_value("os", n.os);
            collected.insert_value(
                "memory_total",
                UntaggedValue::bytes(n.memory_total).into_untagged_value(),
            );
            collected.insert_value(
                "memory_free",
                UntaggedValue::bytes(n.memory_free).into_untagged_value(),
            );

            collected.into_value()
        })
        .collect::<Vec<_>>();

    Ok(nodes.into())
}

#[derive(Debug, Deserialize)]
struct PoolInfo {
    name: String,
    nodes: Vec<NodeInfo>,
}

#[derive(Debug, Deserialize)]
struct NodeInfo {
    hostname: String,
    status: String,
    #[serde(rename = "memoryTotal")]
    memory_total: u64,
    #[serde(rename = "memoryFree")]
    memory_free: u64,
    services: Vec<NodeService>,
    version: String,
    os: String,
}

#[derive(Debug, Deserialize)]
enum NodeService {
    #[serde(rename = "cbas")]
    Analytics,
    #[serde(rename = "eventing")]
    Eventing,
    #[serde(rename = "fts")]
    Search,
    #[serde(rename = "n1ql")]
    Query,
    #[serde(rename = "index")]
    Indexing,
    #[serde(rename = "kv")]
    KeyValue,
}

impl fmt::Display for NodeService {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NodeService::Analytics => write!(f, "analytics"),
            NodeService::Eventing => write!(f, "eventing"),
            NodeService::Search => write!(f, "search"),
            NodeService::Query => write!(f, "query"),
            NodeService::Indexing => write!(f, "indexing"),
            NodeService::KeyValue => write!(f, "kv"),
        }
    }
}
