//! =============================================================
//! All structs defined here must keep the field order as the
//! same as db table schema definitions.
//! =============================================================
use uuid::Uuid;
use chrono::{DateTime, Utc};

use rustorm_dao::{
    ToColumnNames,
    ToTableName,
    FromDao,
    ToDao
};

/// import procedure macros
use rustorm_codegen::{
    ToColumnNames,
    ToTableName,
    FromDao,
    ToDao
};


///
/// Model: Ruser
/// Db table: ruser
///

#[derive(Debug, Clone, FromDao, ToColumnNames, ToTableName)]
pub struct Ruser {
    
    pub id: Uuid,
    
    // use email defaultly
    pub account: String,
    
    pub password: String,
    
    pub salt: String,
    
    pub nickname: String,
    
    pub avatar: Option<String>,
    
    pub wx_openid: Option<String>,
    
    pub say: Option<String>,
   
    // user signup time
    pub signup_time: DateTime<Utc>,
    
    // user role: member => 2, manager => 1, admin => 0
    pub role: i16,
    
    // user status: 0 normal, 1 frozen, 2 deleted
    pub status: i16,
    
    pub github: Option<String>,
}

/// 
/// Model: Section
/// DB table: section
///
#[derive(Debug, FromDao, ToColumnNames, ToTableName)]
pub struct Section {
    
    pub id: Uuid,
    
    pub title: String,
    
    pub description: String,
    
    // use stype to separate forum section and user blog section
    // 0 section, 1 user blog
    pub stype: i32,
   
    // if stype==1, record the binding user to section
    pub suser: Option<Uuid>,
    
    pub created_time: DateTime<Utc>,
    
    // 0 normal, 1 frozen, 2 deleted
    pub status: i16, 
}

/// 
/// Model: Article
/// DB table: article
///

#[derive(Debug, FromDao, ToColumnNames, ToTableName)]
pub struct Article {

    pub id: Uuid,

    pub title: String,

    pub raw_content: String,

    pub content: String,

    pub section_id: Uuid,

    pub author_id: Uuid,
    
    pub tags: String,
    
    // used to planet order ranking: 0 section, 1 user blog
    pub stype: i32,

    pub created_time: DateTime<Utc>,
    
    // 0 normal, 1 frozen, 2 deleted
    pub status: i16,

}


/// 
/// Model: Comment
/// DB table: comment
///

#[derive(Debug, FromDao, ToColumnNames, ToTableName)]
pub struct Comment {
    
    pub id: Uuid,
    
    pub content: String,
    
    pub article_id: Uuid,
    
    pub author_id: Uuid,
    
    pub created_time: DateTime<Utc>,
    
    // 0 normal, 1 frozen, 2 deleted
    pub status: i16,

}


/// 
/// Model: ArticleStats
/// DB table: article_stats
///

#[derive(Debug, FromDao, ToColumnNames, ToTableName)]
pub struct ArticleStats {
    
    pub id: Uuid,
    
    pub article_id: Uuid,
    
    pub created_time: DateTime<Utc>,
    
    pub ruser_id: Option<Uuid>,
    
    pub user_agent: Option<String>,
    
    pub visitor_ip: Option<String>,
}


/// 
/// Model: UserNotify
/// DB: redis
/// a cached user notifications queue
///

#[derive(Debug)]
pub struct UserNotify {
    
    pub user_id: Uuid,
    
    pub send_user_name: String,
    
    pub article_id: Uuid,
    
    pub article_title: String,
    
    pub notify_type: String,
}

// ================================================================


// Submodule for insert data to db
// these structs MUST keep the same names with above models
// every table has only one new inserting case,
// but many retreiving cases.
//
pub mod for_insert {
    use super::*;

    #[derive(Debug, PartialEq, ToDao, ToColumnNames, ToTableName)]
    pub struct Ruser {
        pub account: String,
        pub password: String,
        pub salt: String,
        pub nickname: String,
        pub github: Option<String>,
    }

    #[derive(Debug, PartialEq, ToDao, ToColumnNames, ToTableName)]
    pub struct Section {
        pub title: String,
        pub description: String,
        pub stype: i32,
        pub suser: Option<Uuid>,
    }
}


// Submodule for retrieve data from db
// these structs DON'T need to keep the same names with above models
// every table has only one new inserting case,
// but many retrieving cases.
//
pub mod for_retrieve {


}




