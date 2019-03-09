use crate::db;
use redis::Commands;
use log::info;


pub fn cache_set(model_name: &str, instance_id: &str, content: &Option<Vec<u8>>) {
    let redis = db::get_redis();
    let key = "cache:".to_string() + model_name + ":" + instance_id;

    let body_content = String::from_utf8_lossy(content.to_owned().unwrap().as_slice()).to_string();
    let _:() = redis.hset(&key, "content", body_content).unwrap();
    let _:() = redis.hset(&key, "valid", 1).unwrap();
    // expired after 1 hour 
    let _:() = redis.expire(&key, 3600).unwrap();

}

pub fn cache_is_valid(model_name: &str, instance_id: &str) -> bool {
    let redis = db::get_redis();
    let key = "cache:".to_string() + model_name + ":" + instance_id;
    let valid = redis.hget(&key, "valid").unwrap_or(0);

    if valid == 1 {true} else {false}
}


pub fn cache_set_invalid(model_name: &str, instance_id: &str) {
    let redis = db::get_redis();
    let key = "cache:".to_string() + model_name + ":" + instance_id;
    let _:() = redis.hset(&key, "valid", 0).unwrap();
}

pub fn cache_get(model_name: &str, instance_id: &str) -> String {
    let redis = db::get_redis();
    let key = "cache:".to_string() + model_name + ":" + instance_id;
    let valid = redis.hget(&key, "valid").unwrap_or(0);

    if valid == 0 {
        return "".to_string()
    }
    else {
        info!("cache hit: {} {}", model_name, instance_id);
        redis.hget(&key, "content").unwrap_or("".to_string())
    }
}
