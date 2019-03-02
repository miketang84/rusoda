use std::fs;
use std::env;
use toml::Value;
use lazy_static::lazy_static;
use toml_query::read::TomlValueReadExt;

lazy_static! {
    pub static ref I18NDATA: Value = parse_toml_str(read_i18n_data());
}


pub fn read_i18n_data() -> String {
    String::from_utf8_lossy(&fs::read("tomls/i18n.toml").unwrap()).to_string()
}


pub fn parse_toml_str(i18n_data: String) -> Value {
    match i18n_data.parse::<Value>() {
        Ok(value) => {
            value
        },
        Err(err) => {
            panic!("{:?}", err)
        }
    }
}


pub fn read_i18n_item(item_str: &str, lang: &str) -> String {
    let path = item_str.to_string() + "." + lang;
    let ret = I18NDATA.read(&path);

    if ret.is_err() {
        panic!("Reading i18n path err: {}", path);
    }
    let ret = ret.unwrap();
    if ret.is_none() {
        println!("Reading i18n path, none result: {}", path);
        return "".to_string();
    }
    let ret = ret.unwrap();

    match ret {
        Value::String(s) => {
            s.to_string()
        }
        _ => "".to_string()
    }
}
