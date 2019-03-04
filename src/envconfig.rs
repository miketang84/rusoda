use std::env;

pub fn get_str_item(item_name: &str) -> String {
    env::var(item_name).expect(&format!("env config item {} must be set", item_name))
}

pub fn get_int_item(item_name: &str) -> i64 {
    let item_str = get_str_item(item_name);
    item_str.parse::<i64>().expect(&format!("parse env config item to i64 err: {}", item_name))
}


