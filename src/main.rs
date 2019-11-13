mod graphql_service;
mod routes;

use routes::setup_routes;
use std::sync::{atomic, Arc};
use tide::{middleware::RootLogger, App};

// First, we define `State` that holds accumulator state. This is accessible as App data in
// Tide, and as executor context in Juniper.
#[derive(Clone, Default)]
pub struct State(Arc<atomic::AtomicIsize>);

fn main() {
    let mut app = App::with_state(State::default());
    app.middleware(RootLogger::new());

    setup_routes(&mut app);

    app.serve("127.0.0.1:1212").unwrap();
}
