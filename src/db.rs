
use rustorm::{Pool, EntityManager};

use redis::Client as redis_client;
use redis::Connection as RedisConnection;


pub fn get_db() -> EntityManager {
    let db_url = "postgres://postgres:123@localhost/rusoda";
    let mut pool = Pool::new();
    let em = pool.em(db_url).unwrap();

    em
}

pub fn get_redis() -> RedisConnection {
    let client = redis_client::open("redis://127.0.0.1/1").unwrap();
    let conn = client.get_connection().unwrap();

    conn
}

