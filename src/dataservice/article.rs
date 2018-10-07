
use rustorm::DbError;
use crate::db;
use crate::model::Article;
use crate::model::{for_write, for_read};


// here, we impl some methods for for_insert::Section
impl for_write::ArticleCreate {
    pub fn insert(&self) -> Result<Article, String>{
        let em = db::get_db();
        match db_insert!(em, self, Article) {
            Some(art) => {
                Ok(art.to_owned())
            },
            None => {
                Err("Insert article error.".to_string())
            }
        }
    }
}

// impl retrieving methods on this model, return various views of Section
impl Article {


}
