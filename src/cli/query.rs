use super::util::convert_json_value_to_nu_value;
use crate::state::State;
use couchbase::QueryOptions;
use futures::executor::block_on;
use futures::stream::StreamExt;
use log::debug;
use nu_cli::{CommandArgs, CommandRegistry, OutputStream};
use nu_errors::ShellError;
use nu_protocol::{Signature, SyntaxShape};
use nu_source::Tag;
use std::sync::Arc;

pub struct Query {
    state: Arc<State>,
}

impl Query {
    pub fn new(state: Arc<State>) -> Self {
        Self { state }
    }
}

impl nu_cli::WholeStreamCommand for Query {
    fn name(&self) -> &str {
        "query"
    }

    fn signature(&self) -> Signature {
        Signature::build("query").required("statement", SyntaxShape::String, "the query statement")
    }

    fn usage(&self) -> &str {
        "Performs a n1ql query"
    }

    fn run(
        &self,
        args: CommandArgs,
        registry: &CommandRegistry,
    ) -> Result<OutputStream, ShellError> {
        let args = args.evaluate_once(registry)?;
        let statement = args.nth(0).expect("need statement").as_string()?;

        debug!("Running n1ql query {}", &statement);
        let result = block_on(
            self.state
                .active_cluster()
                .cluster()
                .query(statement, QueryOptions::default()),
        );

        match result {
            Ok(mut r) => {
                let stream = r
                    .rows::<serde_json::Value>()
                    .map(|v| convert_json_value_to_nu_value(&v.unwrap(), Tag::default()));
                Ok(OutputStream::from_input(stream))
            }
            Err(e) => Err(ShellError::untagged_runtime_error(format!("{}", e))),
        }
    }
}
