
mod db;
mod model;

use self::model::User;

use log::info;

use rustorm::DbError;

fn main(){
    env_logger::init();

    let em = db::get_db();
    let sql = "SELECT * FROM ruser LIMIT 10";
    let users: Result<Vec<User>, DbError> = em.execute_sql_with_return(sql, &[]);
    let users = users.unwrap();
    assert_eq!(users.len(), 1);
    for user in users {
        info!("user: {:?}", user);
    }
}
