use tide::{App, Context};
use super::graphql_service;

async fn echo_path(_: Context<()>) -> String {
  "others".into()
}

pub fn setup_routes(app: &mut App<()>) {
    &app.at("/graphql").get(graphql_service::get_graphql_service);
    &app.at("*").get(echo_path);
}
