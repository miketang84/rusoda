
use rustorm::DbError;
use crate::db;
use crate::model::Comment;
use crate::model::{for_write, for_read};


// here, we impl some methods for for_insert::Section
impl for_write::CommentCreate {
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

impl for_write::CommentEdit {
    pub fn update(&self) -> Result<Comment, String>{
        let em = db::get_db();
        let clause = format!("where id={}", self.id);
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

impl for_write::CommentDelete    {
    pub fn delete(&self) -> Result<Comment, String>{
        let em = db::get_db();
        let clause = format!("where id={}", self.id);
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
    // NOTICE: no self here
    pub fn get_comments_by_article_id(article_id: Uuid) -> Result<Vec<Comment>, String> {
        let em = db::get_db();
        let clause = format!("where article_id={} order by created_time desc", article_id);
        let comments = db_select!(em, "", "", &clause, Comment);

        Ok(comments)
    }
}


