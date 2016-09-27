// Iron
extern crate iron;

extern crate bodyparser;
extern crate logger;
extern crate persistent;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate router;

use iron::prelude::*;
use logger::Logger;
use persistent::Read;
use router::Router;

mod routes;
mod database;

// Allow a maximum size of 10 MiB as body size
const MAX_BODY_LENGTH: usize = 10 * 1024 * 1024;

fn main() {
    println!("Connecting to database");
    let pool = database::setup();

    println!("Setting up routes");
    let mut router = Router::new();
    routes::setup(&mut router);

    let (logger_before, logger_after) = Logger::new(None);

    let mut chain = Chain::new(router);
    chain.link_before(logger_before);
    chain.link_before(Read::<bodyparser::MaxBodyLength>::one(MAX_BODY_LENGTH));

    chain.link_before(Read::<database::StocksDatabase>::one(pool));

    chain.link_after(logger_after);

    println!("Starting server!");
    Iron::new(chain).http("localhost:3000").unwrap();
    println!("Terminating");
}
