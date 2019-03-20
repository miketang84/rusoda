use crate::db;
use uuid::Uuid;
use crate::model::{for_write, for_read};
use redis::Commands;

pub use crate::model::Article;
pub use crate::model::ArticleWeight;
pub use crate::model::for_write::{
    ArticleCreate,
    ArticleEdit,
    ArticleDelete,
    ArticleWeightCreate
};

pub use crate::model::for_read::{
    ArticleForList,
    ArticleForList2,
    BlogArticleForList,
    CommentWithAuthorName,
    ArticleCount,
    ArticleWeightView,
};

use crate::dataservice::comment::{
    Comment,
    CommentCount
};
use crate::envconfig;


// here, we impl some methods for for_insert::Section
impl ArticleCreate {
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

impl ArticleEdit {
    pub fn update(&self) -> Result<Article, String>{
        let em = db::get_db();
        let clause = format!("where id='{}'", self.id);
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

impl ArticleDelete    {
    pub fn delete(&self) -> Result<Article, String>{
        let em = db::get_db();
        let clause = format!("where id='{}'", self.id);
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
    pub fn get_by_id(id: Uuid) -> Result<Article, String> {
        let em = db::get_db();
        let clause = format!("where id='{}'", id);
        match db_find!(em, "", "", &clause, Article) {
            Some(art) => {
                Ok(art.to_owned())
            },
            None => {
                Err("get article error.".to_string())
            }
        }
    }

    pub fn delete_by_id(id: Uuid) -> Result<Article, String> {
        let em = db::get_db();
        let clause = format!("where id='{}'", id);
        match db_delete!(em, &clause, Article) {
            Some(art) => {
                Ok(art.to_owned())
            },
            None => {
                Err("delete article error.".to_string())
            }
        }
    }

    pub fn paging(page: usize, page_size: usize) -> Vec<Article> {
        let em = db::get_db();
        let clause = format!("order by created_time desc limit {} offset {}", page_size, page_size*page);
        let articles = db_select!(em, "", "", &clause, Article);

        articles
    }

    pub fn paging_by_section(section_id: Uuid, page: i64, page_size: i64) -> Vec<Article> {
        let em = db::get_db();
        let clause = format!("where section_id='{}' order by created_time desc limit {} offset {}", section_id, page_size, page_size*page);
        let articles = db_select!(em, "", "", &clause, Article);

        articles
    }

    pub fn get_latest_articles(size: i64) -> Vec<ArticleForList> {
        let em = db::get_db();
        // need to alias names
        let head_clause = "article.id, article.title, article.created_time, article.tags, section.title as section_title, ruser.nickname as author_name";
        let from_clause = "FROM article LEFT JOIN section ON article.section_id = section.id LEFT JOIN ruser ON article.author_id = ruser.id";
        let rest_clause = format!("WHERE article.stype = 0 ORDER BY created_time DESC LIMIT {}", size);
        let articles = db_select!(em, head_clause, from_clause, &rest_clause, ArticleForList);

        articles
    }

    pub fn get_latest_full_articles(size: i64) -> Vec<Article> {
        let em = db::get_db();
        // need to alias names
        let head_clause = "";
        let from_clause = "";
        let rest_clause = format!("WHERE article.stype = 0 ORDER BY created_time DESC LIMIT {}", size);
        let articles = db_select!(em, head_clause, from_clause, &rest_clause, Article);

        articles
    }

    pub fn get_all_articles() -> Vec<Article> {
        let em = db::get_db();
        // need to alias names
        let head_clause = "";
        let from_clause = "";
        let rest_clause = "ORDER BY created_time DESC";
        let articles = db_select!(em, head_clause, from_clause, rest_clause, Article);

        articles
    }

    pub fn get_latest_blog_articles(size: i64) -> Vec<BlogArticleForList> {
        let em = db::get_db();
        // need to alias names
        let head_clause = "article.id, article.title, article.created_time, ruser.nickname as author_name";
        let from_clause = "FROM article LEFT JOIN ruser ON article.author_id = ruser.id";
        let rest_clause = format!("WHERE article.stype = 1 ORDER BY created_time DESC LIMIT {}", size);
        let blog_articles = db_select!(em, head_clause, from_clause, &rest_clause, BlogArticleForList);

        blog_articles
    }

    pub fn get_latest_full_blog_articles(size: i64) -> Vec<Article> {
        let em = db::get_db();
        // need to alias names
        let head_clause = "";
        let from_clause = "";
        let rest_clause = format!("WHERE article.stype = 1 ORDER BY created_time DESC LIMIT {}", size);
        let articles = db_select!(em, head_clause, from_clause, &rest_clause, Article);

        articles
    }

    pub fn get_comments_paging_belong_to_this(article_id: Uuid, current_page: i64) -> Vec<CommentWithAuthorName> {
        let em = db::get_db();

        let ncpp = envconfig::get_int_item("NUMBER_COMMENT_PER_PAGE");
        let offset = ncpp * (current_page - 1);
        let head_clause = "comment.id, comment.content, comment.author_id, comment.created_time, ruser.nickname";
        let from_clause = "FROM comment LEFT JOIN ruser ON comment.author_id = ruser.id";
        let clause = format!("where article_id='{}' order by created_time desc limit {} offset {}", article_id, ncpp, offset);
        let comments = db_select!(em, head_clause, from_clause, &clause, CommentWithAuthorName);

        comments
    }

    pub fn get_comments_count_belong_to_this(article_id: Uuid) -> i64 {
        let em = db::get_db();
        let clause = format!("where article_id='{}'", article_id);
        let count_r = db_find!(em, "count(*)", "from comment", &clause, CommentCount);

        count_r.unwrap().count
    }

    pub fn increase_viewtimes(article_id: Uuid) {
        let redis = db::get_redis();
        let _: () = redis.hincr("article_stats", article_id.to_string(), 1).unwrap();

    }

    pub fn get_viewtimes(article_id: Uuid) -> i64 {
        let redis = db::get_redis();
        redis.hget("article_stats", article_id.to_string()).unwrap_or(0)
    }

}


impl ArticleWeight {
    pub fn delete_by_id(id: Uuid) -> Result<ArticleWeight, String>{
        let em = db::get_db();
        let clause = format!("where id='{}'", id);
        match db_delete!(em, &clause, ArticleWeight) {
            Some(art) => {
                Ok(art.to_owned())
            },
            None => {
                Err("delete articleweight error.".to_string())
            }
        }
    }
}

impl ArticleWeightCreate {
    pub fn insert(&self) -> Result<ArticleWeight, String>{
        let em = db::get_db();
        match db_insert!(em, self, ArticleWeight) {
            Some(art) => {
                Ok(art.to_owned())
            },
            None => {
                Err("Insert articleweight error.".to_string())
            }
        }
    }
}

impl ArticleWeightView {
    pub fn get_all_weight_articles() -> Vec<ArticleWeightView> {
        let em = db::get_db();
        let head_clause = "articleweight.id, article_id, articleweight.section_id, article.title, weight, article.created_time";
        let from_clause = "FROM articleweight LEFT JOIN article ON article.id = articleweight.article_id";
        let rest_clause = "order by weight desc";
        let articles = db_select!(em, head_clause, from_clause, &rest_clause, ArticleWeightView);

        articles
    }
}
