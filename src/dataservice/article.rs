
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

impl for_write::ArticleEdit {
    pub fn update(&self) -> Result<Article, String>{
        let em = db::get_db();
        let clause = format!("where id={}", self.id);
        // here, will overide the id field, that's for tidy code yet
        match db_update!(em, self, &clause, Article) {
            Some(art) => {
                Ok(art.to_owned())
            },
            None => {
                Err("Update article error.".to_string())
            }
        }
    }
}

impl for_write::ArticleDelete    {
    pub fn delete(&self) -> Result<Article, String>{
        let em = db::get_db();
        let clause = format!("where id={}", self.id);
        match db_delete!(em, &clause, Article) {
            Some(art) => {
                Ok(art.to_owned())
            },
            None => {
                Err("Delete article error.".to_string())
            }
        }
    }
}


// impl retrieving methods on this model, return various views of Section
impl Article {
    // NOTICE: no self here
    pub fn article_get_by_id(id: Uuid) -> Result<Article, String> {
        let em = db::get_db();
        let clause = format!("where id={}", id);
        match db_find!(em, "", "", &clause, Article) {
            Some(art) => {
                Ok(art.to_owned())
            },
            None => {
                Err("get article error.".to_string())
            }
        }
    }

    pub fn article_paging(page: usize, page_size: usize) -> Vec<Article> {
        let em = db::get_db();
        let clause = format!("order by created_time desc limit {} offset {}", page_size, page_size*page);
        let articles = db_select!(em, "", "", &clause, Article);

        articles
    }

    pub fn article_paging_by_section(section_id: Uuid, page: usize, page_size: usize) -> Vec<Article> {
        let em = db::get_db();
        let clause = format!("where section_id={} order by created_time desc limit {} offset {}", section_id, page_size, page_size*page);
        let articles = db_select!(em, "", "", &clause, Article);

        articles
    }

}


