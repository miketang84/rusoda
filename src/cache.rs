use crate::db;
use redis::Commands;

pub fn cache_set(model_name: &str, instance_id: &str, content: &str) {
    let redis = db::get_redis();
    let key = model_name.to_string() + ":" + instance_id;
    let _:() = redis.hset(&key, "content", content).unwrap();
    let _:() = redis.hset(&key, "valid", 1).unwrap();

}

pub fn cache_is_valid(model_name: &str, instance_id: &str) -> bool {
    let redis = db::get_redis();
    let key = model_name.to_string() + ":" + instance_id;
    let valid = redis.hget(&key, "valid").unwrap_or(0);

    if valid == 1 {true} else {false}
}


pub fn cache_set_invalid(model_name: &str, instance_id: &str) {
    let redis = db::get_redis();
    let key = model_name.to_string() + ":" + instance_id;
    let _:() = redis.hset(&key, "valid", 0).unwrap();
}

pub fn cache_get(model_name: &str, instance_id: &str) -> String {
    let redis = db::get_redis();
    let key = model_name.to_string() + ":" + instance_id;
    let valid = redis.hget(&key, "valid").unwrap_or(0);

    if valid == 0 {
        return "".to_string()
    }
    else {
        redis.hget(&key, "content").unwrap_or("".to_string())
    }
}
