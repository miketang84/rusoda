
use rustorm::DbError;
use crate::db;
use crate::model::Ruser;
use crate::model::{for_insert, for_retrieve};
use crate::util::{random_string, sha3_256_encode};
use redis::Commands;
use chrono::{DateTime, Utc};

use log::info;

// this struct is defined for request params
pub struct UserSignUp {
    pub account: String,
    pub password: String,
    pub nickname: String,
}

impl UserSignUp {
    pub fn sign_up_with_email (&self) -> Result<Ruser, String>{
        let em = db::get_db();
        let salt = random_string(6);

        let new_user = for_insert::Ruser {
            account: self.account.to_owned(),
            password: sha3_256_encode(&format!("{}{}", self.password, salt)),
            salt: salt,
            nickname: self.nickname.to_owned(),
            github: None,
        };

        let sql = format!(
            "SELECT {a} FROM {b} 
            where account='{c}' 
            LIMIT 1",
            a = "account",
            b = "ruser",
            c = new_user.account);

        // check if the same name account exists already 
        let user_ret = em.execute_sql_with_return(&sql, &[]).unwrap_or(Vec::<Ruser>::new()); 
        match user_ret.first() {
            Some(_) => {
                // exist already, return Error
                Err(format!("user {} exists.", new_user.account))
            },
            None => {
                // it's a new user, insert it
                let user_ret = em.insert(&[&new_user]).unwrap_or(Vec::<Ruser>::new()); 
                match user_ret.first() {
                    Some(user) => {
                        // generate a corresponding section to this user as his blog section
                        //let section = for_insert::Section {
                        //    title: user.nickname,
                        //    description: format!("{}'s blog", user.nickname),
                        //    suser: Some(user.id),
                        //    stype: 1,
                        //};
                        //section.insert();

                        // set user cookies to redis to keep login session
                        set_session(&user.account).unwrap();

                        Ok(user.to_owned())
                    },
                    None => {
                        unreachable!();
                    }
                }
            }
        }
    }
}


pub fn set_session(account: &str) -> Result<String, String> {
    let redis = db::get_redis();
    let cookie = sha3_256_encode(&random_string(8));
    let _: () = redis.hset(&cookie, "login_time", Utc::now().timestamp()).unwrap();
    let _: () = redis.hset(&cookie, "account", account).unwrap();
    let _: () = redis.expire(&cookie, 24 * 3600).unwrap();

    Ok(cookie)
}







pub fn test () {
    let em = db::get_db();
    let sql = "SELECT * FROM ruser LIMIT 10";
    let users: Result<Vec<Ruser>, DbError> = em.execute_sql_with_return(sql, &[]);
    let users = users.unwrap();
    assert_eq!(users.len(), 1);
    for user in users {
        info!("user: {:?}", user);
    }
}


