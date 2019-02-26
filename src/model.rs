//! =============================================================
//! All structs defined here must keep the field order as the
//! same as db table schema definitions.
//! =============================================================
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};

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

#[derive(Debug, Clone, FromDao, ToColumnNames, ToTableName, Serialize, Deserialize)]
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

    // user role: member => 0, admin => 9
    pub role: i16,

    // user status: 0 normal, 1 frozen, 2 deleted
    pub status: i16,

    pub github: Option<String>,
}

///
/// Model: Section
/// DB table: section
///
#[derive(Debug, Clone, FromDao, ToColumnNames, ToTableName, Serialize, Deserialize)]
pub struct Section {

    pub id: Uuid,

    pub title: String,

    pub description: String,

    // use stype to separate forum section and user blog section
    // 0 section, 1 user blog section
    pub stype: i32,

    // if stype==1, record the binding user to section
    pub suser: Option<Uuid>,

    pub created_time: DateTime<Utc>,

    // 0 normal, 1 frozen, 2 deleted
    pub status: i16,
    // for order, if smaller than zero, doesn't display it
    // for blog section, it defaults -1.0
    pub weight: f32
}

///
/// Model: Article
/// DB table: article
///

#[derive(Debug, Clone, FromDao, ToColumnNames, ToTableName, Serialize, Deserialize)]
pub struct Article {

    pub id: Uuid,

    pub title: String,

    pub raw_content: String,

    pub content: String,

    pub section_id: Uuid,

    pub author_id: Uuid,

    pub tags: String,

    // used to planet order ranking: 0 forum article, 1 user blog article
    pub stype: i32,

    pub created_time: DateTime<Utc>,

    // 0 normal, 1 frozen, 2 deleted
    pub status: i16,

}


///
/// Model: Comment
/// DB table: comment
///

#[derive(Debug, Clone, FromDao, ToColumnNames, ToTableName, Serialize, Deserialize)]
pub struct Comment {

    pub id: Uuid,

    pub raw_content: String,

    pub content: String,

    pub article_id: Uuid,

    pub author_id: Uuid,

    pub created_time: DateTime<Utc>,

    // 0 normal, 1 frozen, 2 deleted
    pub status: i16,

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


// Submodule for write data to db
// these structs DON'T need to keep the same names with above models
// every table has only one new inserting case,
// but many retreiving cases.
//
pub mod for_write {
    use super::*;

    /// User DMO - Data modification object
    #[derive(Debug, PartialEq, ToDao, ToColumnNames, ToTableName)]
    pub struct UserCreate {
        pub account: String,
        pub password: String,
        pub salt: String,
        pub nickname: String,
        pub github: Option<String>,
    }

    #[derive(Debug, PartialEq, ToDao, ToColumnNames, ToTableName)]
    pub struct UserEdit {
        pub id: Uuid,
        pub nickname: String,
        pub avatar: String,
        pub say: String,
    }

    #[derive(Debug, PartialEq, ToDao, ToColumnNames, ToTableName)]
    pub struct UserDelete {
        pub id: Uuid
    }


    /// Section DMO
    #[derive(Debug, PartialEq, ToDao, ToColumnNames, ToTableName)]
    pub struct SectionCreate {
        pub title: String,
        pub description: String,
        pub stype: i32,
        pub suser: Option<Uuid>,
    }

    #[derive(Debug, PartialEq, ToDao, ToColumnNames, ToTableName)]
    pub struct SectionEdit {
        pub id: Uuid,
        pub title: String,
        pub description: String,
    }

    #[derive(Debug, PartialEq, ToDao, ToColumnNames, ToTableName)]
    pub struct SectionDelete {
        pub id: Uuid,
    }

    #[derive(Debug, PartialEq, ToDao, ToColumnNames, ToTableName)]
    pub struct UpdateSectionWeight {
        pub id: Uuid,
        pub weight: f32
    }


    /// Article DMO
    #[derive(Debug, PartialEq, ToDao, ToColumnNames, ToTableName)]
    pub struct ArticleCreate {
        pub title: String,
        pub raw_content: String,
        pub content: String,
        pub section_id: Uuid,
        pub author_id: Uuid,
        pub tags: String,
        pub stype: i32,
        // created_time, auto created by db
        // pub created_time: DateTime<Utc>,
        pub status: i16,
    }

    #[derive(Debug, PartialEq, ToDao, ToColumnNames, ToTableName)]
    pub struct ArticleEdit {
        pub id: Uuid,
        pub section_id: Uuid,
        pub title: String,
        pub raw_content: String,
        pub content: String,
        pub tags: String,
    }

    #[derive(Debug, PartialEq, ToDao, ToColumnNames, ToTableName)]
    pub struct ArticleDelete {
        pub id: Uuid,
    }

    /// Comment DMO
    #[derive(Debug, PartialEq, ToDao, ToColumnNames, ToTableName)]
    pub struct CommentCreate {
        pub raw_content: String,
        pub content: String,
        pub article_id: Uuid,
        pub author_id: Uuid,
        pub status: i16,
    }

    #[derive(Debug, PartialEq, ToDao, ToColumnNames, ToTableName)]
    pub struct CommentEdit {
        pub id: Uuid,
        pub raw_content: String,
        pub content: String,
    }

    #[derive(Debug, PartialEq, ToDao, ToColumnNames, ToTableName)]
    pub struct CommentDelete {
        pub id: Uuid,
    }

    #[derive(Debug, PartialEq, ToDao, ToColumnNames, ToTableName)]
    pub struct CommentFreeze {
        pub id: Uuid,
    }


}


// Submodule for retrieve data from db
// these structs DON'T need to keep the same names with above models
// every table has only one new inserting case,
// but many retrieving cases.
//
pub mod for_read {
    use super::*;

    #[derive(Debug, Clone, FromDao, ToColumnNames, ToTableName, Serialize, Deserialize)]
    pub struct RuserWithoutPwd {
        pub id: Uuid,
        pub account: String,
        pub nickname: String,
        pub avatar: Option<String>,
        pub wx_openid: Option<String>,
        pub say: Option<String>,
        pub signup_time: DateTime<Utc>,
        pub role: i16,
        pub status: i16,
        pub github: Option<String>,
    }

    #[derive(Debug, Clone, FromDao, ToColumnNames, ToTableName, Serialize, Deserialize)]
    pub struct ArticleForList {
        pub id: Uuid,
        pub title: String,
        pub created_time: DateTime<Utc>,
        pub section_title: String,      // left join from section
        pub author_name: String,        // left join from ruser
    }

    #[derive(Debug, Clone, FromDao, ToColumnNames, ToTableName, Serialize, Deserialize)]
    pub struct BlogArticleForList {
        pub id: Uuid,
        pub title: String,
        pub created_time: DateTime<Utc>,
        pub author_name: String,        // left join from ruser
    }

    #[derive(Debug, Clone, FromDao, ToColumnNames, ToTableName, Serialize, Deserialize)]
    pub struct ArticleCount {
        pub count: i64
    }

    #[derive(Debug, Clone, FromDao, ToColumnNames, ToTableName, Serialize, Deserialize)]
    pub struct CommentWithAuthorName {
        pub id: Uuid,
        pub content: String,
        pub author_id: Uuid,
        pub created_time: DateTime<Utc>,
        pub author_name: String,        // left join from ruser
    }

    #[derive(Debug, Clone, FromDao, ToColumnNames, ToTableName, Serialize, Deserialize)]
    pub struct CommentCount {
        pub count: i64
    }

}




