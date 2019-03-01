use std::time::Duration;
use std::collections::HashMap;
use chrono::{DateTime, Utc, NaiveDateTime};
use std::env;

use sapper_std::TERA;

use sapper_std::{
    TeraValue,
    to_value,
    TeraResult
};

use crate::i18n::read_i18n_item;



pub fn zone8_view(date: DateTime<Utc>) -> NaiveDateTime {
    NaiveDateTime::from_timestamp(date.timestamp() + 8*3600, 0)
}

pub fn zone8(value: TeraValue, mut args: HashMap<String, TeraValue>) -> TeraResult<TeraValue> {
    let output = match value {
        TeraValue::String(s) => {
            match s.parse::<DateTime<Utc>>() {
                Ok(val) => {
                    zone8_view(val).format("%Y-%m-%d %H:%M").to_string()
                }
                Err(_) => {
                    "".to_string()
                }
            }
        },
        _ => { "".to_string() }
    };

    Ok(to_value(&output)?)
}


pub fn i18n(value: TeraValue, mut args: HashMap<String, TeraValue>) -> TeraResult<TeraValue> {
    let lang = env::var("RUSODA_LANG").expect("RUSODA_LANG must be set");
    let output = match value {
        TeraValue::String(ref s) => {
            read_i18n_item(s, &lang)
        },
        _ => { "".to_string() }
    };

    Ok(to_value(&output)?)
}


pub fn register_web_filters() {
    TERA.write()
        .and_then(|mut _tera| {
            _tera.register_filter("zone8", zone8);
            _tera.register_filter("i18n", i18n);
            Ok(())
        }).unwrap();
}
