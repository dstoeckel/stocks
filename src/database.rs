extern crate iron;
extern crate r2d2;

use r2d2::{Config, Pool};
use r2d2_postgres::{PostgresConnectionManager, SslMode};

pub struct StocksDatabase;
pub type StocksDbPool = Pool<PostgresConnectionManager>;

impl iron::typemap::Key for StocksDatabase {
    type Value = StocksDbPool;
}

pub fn setup() -> StocksDbPool {
    let dbconfig = Config::builder()
        .pool_size(10)
        .build();

    // FIXME: Use TLS, do not use unwrap
    let dbmanager = PostgresConnectionManager::new("postgres://daniel@%2Fvar%2Frun%2Fpostgresql/stocks", SslMode::None)
        .unwrap();

    r2d2::Pool::new(dbconfig, dbmanager).unwrap()
}

