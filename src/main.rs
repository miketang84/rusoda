
/// 
use rustorm_dao::{
    ToColumnNames,
    ToTableName,
    FromDao
};

/// import procedure macros
use rustorm_codegen::{
    ToColumnNames,
    ToTableName,
    FromDao
};

use rustorm::{Pool, DbError};

#[derive(Debug, FromDao, ToColumnNames, ToTableName)]
struct Actor {
    salt: String,
    account: String,
}

fn main(){
    let db_url = "postgres://postgres:123@localhost/forustm";
    let mut pool = Pool::new();
    let em = pool.em(db_url).unwrap();
    let sql = "SELECT * FROM ruser LIMIT 10";
    let actors: Result<Vec<Actor>, DbError> = em.execute_sql_with_return(sql, &[]);
    println!("Actor: {:#?}", actors);
    let actors = actors.unwrap();
    assert_eq!(actors.len(), 1);
    for actor in actors {
        println!("actor: {:?}", actor);
    }
}
