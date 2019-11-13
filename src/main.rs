mod graphql_service;
mod routes;

use routes::setup_routes;
use tide::{middleware::RootLogger, App};
fn main() {
    let mut app = App::new();
    app.middleware(RootLogger::new());

    setup_routes(&mut app);

    app.serve("127.0.0.1:1212").unwrap();
}
