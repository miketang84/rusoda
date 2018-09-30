
use rustorm::{Pool, EntityManager};

pub fn get_db() -> EntityManager {
    let db_url = "postgres://postgres:123@localhost/forustm";
    let mut pool = Pool::new();
    let em = pool.em(db_url).unwrap();

    em
}
