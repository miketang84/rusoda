use wumn::DbError;
use crate::db;
use crate::model::{for_write, for_read};
use uuid::Uuid;

pub use crate::model::Comment;
pub use crate::model::for_write::{
    CommentCreate,
    CommentEdit,
    CommentDelete
};

pub use crate::model::for_read::{
    CommentCount,
    CommentWithAuthorName,
};


// here, we impl some methods for for_insert::Section
impl CommentCreate {
    pub fn insert(&self) -> Result<Comment, String>{
        let em = db::get_db();
        match db_insert!(em, self, Comment) {
            Some(com) => {
                Ok(com.to_owned())
            },
            None => {
                Err("Insert comment error.".to_string())
            }
        }
    }
}

impl CommentEdit {
    pub fn update(&self) -> Result<Comment, String>{
        let em = db::get_db();
        let clause = format!("where id='{}'", self.id);
        // here, will overide the id field, that's for tidy code yet
        match db_update!(em, self, &clause, Comment) {
            Some(com) => {
                Ok(com.to_owned())
            },
            None => {
                Err("Update comment error.".to_string())
            }
        }
    }
}

impl CommentDelete    {
    pub fn delete(&self) -> Result<Comment, String>{
        let em = db::get_db();
        let clause = format!("where id='{}'", self.id);
        match db_delete!(em, &clause, Comment) {
            Some(com) => {
                Ok(com.to_owned())
            },
            None => {
                Err("Delete comment error.".to_string())
            }
        }
    }
}


// impl retrieving methods on this model, return various views of Section
impl Comment {
    pub fn get_by_id(id: Uuid) -> Result<Comment, String> {
        let em = db::get_db();
        let clause = format!("where id='{}'", id);
        match db_find!(em, "", "", &clause, Comment) {
            Some(comment) => {
                Ok(comment.to_owned())
            },
            None => {
                Err("get comment error.".to_string())
            }
        }
    }

    pub fn get_comment_with_author_name(id: Uuid) -> Result<CommentWithAuthorName, String> {
        let em = db::get_db();

        let head_clause = "comment.id, comment.raw_content as content, comment.author_id, comment.created_time, ruser.nickname";
        let from_clause = "FROM comment LEFT JOIN ruser ON comment.author_id = ruser.id";
        let clause = format!("where id='{}'", id);
        match db_find!(em, head_clause, from_clause, &clause, CommentWithAuthorName) {
            Some(comment) => {
                Ok(comment.to_owned())
            },
            None => {
                Err("get comment error.".to_string())
            }
        }
    }

    pub fn delete_by_id(id: Uuid) -> Result<Comment, String> {
        let em = db::get_db();
        let clause = format!("where id='{}'", id);
        match db_delete!(em, &clause, Comment) {
            Some(comment) => {
                Ok(comment.to_owned())
            },
            None => {
                Err("delete comment error.".to_string())
            }
        }
    }
}
