use deadpool_postgres::{Manager, Pool};
use std::str::FromStr;

pub use queries::user;

pub fn create_pool(database_url: &str) -> Pool {
    let config = tokio_postgres::Config::from_str(database_url).unwrap();
    let manager = Manager::new(config, tokio_postgres::NoTls);
    
    Pool::builder(manager).build().unwrap()
}

include!(concat!(env!("OUT_DIR"), "/cornucopia.rs"));