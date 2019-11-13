use tide::{Context, EndpointResult};
use juniper::graphiql::graphiql_source;

pub async fn get_graphql_service(_: Context<()>) -> EndpointResult {
  let html = graphiql_source("http://localhost:1234/graphql");
  Ok(http::Response::builder()
                .header("Content-Type", "text/html")
                .body(html.into())
                .expect("failed to build static response?"))
}
