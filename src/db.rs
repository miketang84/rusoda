use std::env;
use wumn::{DbManager, EntityManager};

use redis::Client as redis_client;
use redis::Connection as RedisConnection;



pub fn get_db() -> EntityManager {
    let db_url = env::var("DBURL").expect("DBURL must be set");
    let mut pool = DbManager::new();
    let em = pool.em(&db_url).unwrap();

    em
}

pub fn get_redis() -> RedisConnection {
    let redis_url = env::var("REDISURL").expect("REDISURL must be set");
    let client = redis_client::open(&redis_url[..]).unwrap();
    let conn = client.get_connection().unwrap();

    conn
}

