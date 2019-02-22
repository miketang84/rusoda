
use rustorm::DbError;
use crate::db;
use crate::model::Ruser;
use crate::util::{random_string, sha3_256_encode};
use redis::Commands;
use chrono::{DateTime, Utc};

use log::info;



pub fn set_session(account: &str, ttl: usize) -> Result<String, String> {
    let redis = db::get_redis();
    let cookie = sha3_256_encode(&random_string(8));
    let _: () = redis.hset(&cookie, "login_time", Utc::now().timestamp()).unwrap();
    let _: () = redis.hset(&cookie, "account", account).unwrap();
    let _: () = redis.expire(&cookie, ttl).unwrap();

    Ok(cookie)
}



/// ===== Struct Definition =====
// these structs are defined for request params
pub struct UserSignUp {
    pub account: String,
    pub password: String,
    pub nickname: String,
}

pub struct UserLogin {
    account: String,
    password: String,
}

pub struct UserChangePassword {
    pub old_password: String,
    pub new_password: String,
}

pub use crate::model::for_write {
    UserCreate,
    UserEdit,
    SectionCreate,
};

pub use crate::model::for_read {
    RuserWithoutPwd,
};

pub use self::Ruser;

/// ===== Implementation Area =====
///
impl UserSignUp {
    pub fn sign_up_with_email (&self) -> Result<String, String>{
        let em = db::get_db();
        let salt = random_string(6);

        let new_user = UserCreate {
            account: self.account.to_owned(),
            password: sha3_256_encode(&format!("{}{}", self.password, salt)),
            salt: salt,
            nickname: self.nickname.to_owned(),
            github: None,
        };

        let rest_clause = format!("WHERE account='{}'", new_user.account);
        // check if the same name account exists already
        match db_find!(em, "", "", &rest_clause, Ruser) {
            Some(_) => {
                // exist already, return Error
                Err(format!("user {} exists.", new_user.account))
            },
            None => {
                // it's a new user, insert it
                match db_insert!(em, &new_user, Ruser) {
                    Some(user) => {
                        // generate a corresponding section to this user as his blog section
                        let section = SectionCreate {
                            title: user.nickname.to_owned(),
                            description: format!("{}'s blog", user.nickname),
                            stype: 1,
                            suser: Some(user.id.to_owned()),
                        };
                        section.insert();

                        Ok("register success.")
                        //let ttl = 60*24*3600;
                        // set user cookies to redis to keep login session
                        //set_session(&user.account, ttl)
                    },
                    None => {
                        unreachable!();
                    }
                }
            }
        }
    }
}

impl UserLogin {

    pub fn verify_login(&self) -> Result<String, String> {
        let em = db::get_db();

        let rest_clause = format!("WHERE status=0 and account='{}'", self.account);
        // check if the same name account exists already
        match db_find!(em, "", "", &rest_clause, Ruser) {
            Some(user) => {
                // check calulation equality
                if user.password == sha3_256_encode(&format!("{}{}", self.password, user.salt)) {
                    let ttl = 60*24*3600;

                    // store session
                    set_session(&self.account, ttl)

                } else {
                    Err("Wrong account or password.".into())
                }

            },
            None => {
                Err("User doesn't exist.".into())
            }
        }
    }
}


impl UserEdit {
    pub fn update(&self, cookie: &str) -> Result<Ruser, String> {
        let em = db::get_db();
        let redis = db::get_redis();
        let account: String = redis.hget(cookie, "account").unwrap();

        // update new info by account
        let clause = format!("WHERE account={}", account);
        match db_update!(em, self, &clause, Ruser) {
            Some(user) => {
                Ok(user.to_owned())
            },
            None => {
                Err("User doesn't exist.".into())
            }
        }
    }
}


impl UserChangePassword {


}


impl Ruser {
    pub fn get_user_by_cookie(cookie: &str) -> Result<RuserWithoutPwd, String> {
        let em = db::get_db();
        let redis = db::get_redis();
        let account: String = redis.hget(cookie, "account").unwrap();
        match redis.hget(cookie, "account") {
            Ok(account) => {
                let clause = format!("where account={}", account);
                match db_find!(em, "", "", &clause, RuserWithoutPwd) {
                    Some(user) => {
                        Ok(user)
                    },
                    None => Err("no this user".to_string())
                }
            },
            Err(_) => {
                Err("no cookie cached".to_string())
            }
        }
       
    }

    pub fn sign_out(cookie: &str) -> Result<(), String> {
        let redis = db::get_redis();
        let _: () = redis.del(cookie).unwrap();

        Ok(())
    }

}

