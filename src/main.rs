mod graphql_service;
mod routes;

use mongodb::{
    db::{Database, ThreadedDatabase},
    Client, ThreadedClient,
};
use routes::setup_routes;
use std::sync::Mutex;
use tide::{middleware::RootLogger, App};

pub struct State {
    db: Mutex<Database>,
}

impl State {
    pub fn new(db: Mutex<Database>) -> State {
        State { db }
    }
}

fn main() {
    let mongo =
        Client::connect("localhost", 27017).expect("Failed to initialize standalone client.");
    mongo.db("tide").collection("hello");
    let mut app = App::with_state(State::new(Mutex::new(mongo.db("tide"))));
    app.middleware(RootLogger::new());

    setup_routes(&mut app);

    app.serve("127.0.0.1:1212").unwrap();
}
