use tide::{Context, EndpointResult};
use juniper::graphiql::graphiql_source;
use super::super::State;

pub async fn get_graphql_service(_: Context<State>) -> EndpointResult {
  let html = graphiql_source("/graphql");
  Ok(http::Response::builder()
    .header("Content-Type", "text/html")
    .body(html.into())
    .expect("failed to build static response?"))
}
