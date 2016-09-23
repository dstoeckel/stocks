// Iron
extern crate iron;

extern crate bodyparser;
extern crate logger;
extern crate persistent;
extern crate router;

extern crate r2d2;
extern crate r2d2_postgres;

use iron::prelude::*;
use logger::Logger;
use persistent::Read;
use router::Router;
use r2d2::{Config, Pool};
use r2d2_postgres::{PostgresConnectionManager, SslMode};

pub struct StocksDatabase;
pub type StocksDbPool = Pool<PostgresConnectionManager>;

impl iron::typemap::Key for StocksDatabase {
    type Value = StocksDbPool;
}

mod routes;

// Allow a maximum size of 10 MiB as body size
const MAX_BODY_LENGTH: usize = 10 * 1024 * 1024;

fn setup_database() -> StocksDbPool {
    let dbconfig = Config::builder()
        .pool_size(10)
        .build();

    // FIXME: Use TLS, do not use unwrap
    let dbmanager = PostgresConnectionManager::new("postgres://daniel@localhost:5432/stocks", SslMode::None)
        .unwrap();

    r2d2::Pool::new(dbconfig, dbmanager).unwrap()
}

fn setup_routes(router: &mut Router) {
    // Items
    router.get("/item/:id", routes::item::get, "item_get");
    router.delete("/item/:id", routes::item::delete, "item_delete");
    router.post("/item", routes::item::create, "item_create");
    router.put("/item/:id", routes::item::update, "item_update");

    // Locations
    router.get("/location/:id", routes::item::get, "location_get");
    router.delete("/location/:id", routes::item::delete, "location_delete");
    router.post("/location", routes::item::create, "location_create");
    router.put("/location/:id", routes::item::update, "location_update");

    // Users
    router.post("/user/login", routes::user::login, "user_login");
    router.post("/user", routes::user::create, "user_create");
    router.get("/user/:id", routes::user::get, "user_get");
    router.delete("/user/:id", routes::user::delete, "user_delete");
}

fn main() {
    println!("Connecting to database");
    let pool = setup_database();

    println!("Setting up routes");
    let mut router = Router::new();
    setup_routes(&mut router);

    let (logger_before, logger_after) = Logger::new(None);

    let mut chain = Chain::new(router);
    chain.link_before(logger_before);
    chain.link_before(Read::<bodyparser::MaxBodyLength>::one(MAX_BODY_LENGTH));

    chain.link(Read::<StocksDatabase>::both(pool));

    chain.link_after(logger_after);

    println!("Starting server!");
    Iron::new(chain).http("localhost:3000").unwrap();
    println!("Terminating");
}

