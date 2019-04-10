use crate::db;
use crate::util::{random_string, sha3_256_encode};
use serde_derive::{Serialize, Deserialize};
use uuid::Uuid;

pub use crate::model::Section;

pub struct SectionNew {
    pub title: String,
    pub description: String,
}

use crate::dataservice::article::{
    Article,
    ArticleForList,
    ArticleForList2,
    ArticleCount
};

#[derive(Serialize, Deserialize)]
pub struct ArticleWithStats {
    pub article: ArticleForList2,
    pub viewtimes: i64,
    // pub comment_count: i64
}

pub use crate::model::for_write::{
    SectionCreate,
    SectionEdit,
    SectionDelete,
    UpdateSectionWeight,
};

pub use crate::model::for_read::{
    ArticleWeightView
};

use crate::envconfig;


// impl some methods on request params structure
impl SectionNew {
    pub fn create(&self) -> Result<Section, String> {
        let new_section = SectionCreate {
            title: self.title.to_owned(),
            description: self.description.to_owned(),
            stype: 0,
            suser: None
        };

        new_section.insert()
    }

}


// here, we impl some methods for for_insert::Section
impl SectionCreate {
    pub fn insert(&self) -> Result<Section, String> {
        let em = db::get_db();
        match db_insert!(em, self, Section) {
            Some(sec) => {
                Ok(sec.to_owned())
            },
            None => {
                Err("Insert section error.".to_string())
            }
        }
    }
}

impl SectionEdit {
    pub fn update(&self) -> Result<Section, String>{
        let em = db::get_db();
        let clause = format!("where id='{}'", self.id);
        // here, will overide the id field, that's for tidy code yet
        match db_update!(em, self, &clause, Section) {
            Some(sec) => {
                Ok(sec.to_owned())
            },
            None => {
                Err("Update section error.".to_string())
            }
        }
    }
}

impl SectionDelete    {
    pub fn delete(&self) -> Result<Section, String>{
        let em = db::get_db();
        let clause = format!("where id='{}'", self.id);
        match db_delete!(em, &clause, Section) {
            Some(sec) => {
                Ok(sec.to_owned())
            },
            None => {
                Err("Delete section error.".to_string())
            }
        }
    }
}


impl UpdateSectionWeight {
    pub fn update(&self) -> Result<Section, String>{
        let em = db::get_db();
        let clause = format!("where id='{}'", self.id);
        match db_update!(em, self, &clause, Section) {
            Some(sec) => {
                Ok(sec.to_owned())
            },
            None => {
                Err("Delete section error.".to_string())
            }
        }
    }
}

// impl retrieving methods on this model, return various views of Section
impl Section {
    pub fn get_by_id(id: Uuid) -> Result<Section, String> {
        let em = db::get_db();
        let clause = format!("where id='{}'", id);
        match db_find!(em, "", "", &clause, Section) {
            Some(sec) => {
                Ok(sec.to_owned())
            },
            None => {
                Err("get section error.".to_string())
            }
        }
    }

    pub fn get_by_suser(suser_id: Uuid) -> Result<Section, String> {
        let em = db::get_db();
        let clause = format!("where suser='{}'", suser_id);
        match db_find!(em, "", "", &clause, Section) {
            Some(sec) => {
                Ok(sec.to_owned())
            },
            None => {
                Err("get section error.".to_string())
            }
        }
    }

    pub fn get_by_ids(ids: Vec<Uuid>) -> Vec<Section> {
        let em = db::get_db();
        let ids_str = ids.iter().map(|u| u.to_string()).collect::<Vec<String>>().join(", ");
        let clause = format!("where id in ({})", ids_str);
        let sections = db_select!(em, "", "", &clause, Section);

        sections
    }

    pub fn forum_sections() -> Vec<Section> {
        let em = db::get_db();
        let clause = "where (stype=0 or stype=2) and weight >= 0 order by weight desc";
        let sections = db_select!(em, "", "",&clause, Section);

        sections
    }

    pub fn forum_sections_orderby_createdtime() -> Vec<Section> {
        let em = db::get_db();
        let clause = "where (stype=0 or stype=2) and weight >= 0 order by created_time desc";
        let sections = db_select!(em, "", "",&clause, Section);

        sections
    }

    pub fn all_forum_sections() -> Vec<Section> {
        let em = db::get_db();
        let clause = "where (stype=0 or stype=2) order by weight desc";
        let sections = db_select!(em, "", "",&clause, Section);

        sections
    }


    pub fn get_articles_paging_belong_to_this(section_id: Uuid, current_page: i64) -> Vec<ArticleWithStats> {
        let em = db::get_db();
        
        let napp = envconfig::get_int_item("NUMBER_ARTICLE_PER_PAGE");
        let offset = napp * (current_page - 1);
        let head_clause = "article.id, article.title, article.created_time, article.tags, section.title as section_title, ruser.nickname as author_name, (select count(*) from comment where article_id=article.id) as comment_count";
        let from_clause = "FROM article LEFT JOIN section ON article.section_id = section.id LEFT JOIN ruser ON article.author_id = ruser.id";
        let rest_clause = format!("where section_id='{}' order by created_time desc limit {} offset {}", section_id, napp, offset);
        let articles = db_select!(em, head_clause, from_clause, &rest_clause, ArticleForList2);

        // add view times for each article
        let mut article_vec: Vec<ArticleWithStats> = vec![];
        for article in articles {
            let viewtimes = Article::get_viewtimes(article.id);
            let article_with_viewtimes = ArticleWithStats {
                article,
                viewtimes,
            };

            article_vec.push(article_with_viewtimes);
        }

        article_vec
    }

    pub fn get_articles_count_belong_to_this(section_id: Uuid) -> i64 {
        let em = db::get_db();
        let clause = format!("where section_id='{}'", section_id);
        // count here is Option
        let count = db_find!(em, "count(*)", "from article", &clause, ArticleCount);
        count.unwrap().count
    }

    pub fn get_latest_articles_paging_belong_to_this(section_id: Uuid, current_page: i64) -> Vec<ArticleWithStats> {
        let em = db::get_db();
        
        let napp = envconfig::get_int_item("NUMBER_ARTICLE_PER_PAGE");
        let offset = napp * (current_page - 1);
        let head_clause = "article.id, article.title, article.created_time, article.tags, section.title as section_title, ruser.nickname as author_name, (select count(*) from comment where article_id=article.id) as comment_count";
        let from_clause = "FROM article LEFT JOIN section ON article.section_id = section.id LEFT JOIN ruser ON article.author_id = ruser.id";
        let rest_clause = format!("where section_id='{}' order by created_time desc limit {} offset {}", section_id, napp, offset);
        let articles = db_select!(em, head_clause, from_clause, &rest_clause, ArticleForList2);

        // add view times for each article
        let mut article_vec: Vec<ArticleWithStats> = vec![];
        for article in articles {
            let viewtimes = Article::get_viewtimes(article.id);
            let article_with_viewtimes = ArticleWithStats {
                article,
                viewtimes,
            };

            article_vec.push(article_with_viewtimes);
        }

        article_vec
    }

    pub fn get_specified_articles(section_id: Uuid) -> Vec<ArticleWeightView> {
        let em = db::get_db();
        let head_clause = "articleweight.id, article_id, articleweight.section_id, article.title, weight, article.created_time";
        let from_clause = "FROM articleweight LEFT JOIN article ON article.id = articleweight.article_id";
        let rest_clause = format!("where articleweight.section_id='{}' order by weight desc", section_id);
        let articles = db_select!(em, head_clause, from_clause, &rest_clause, ArticleWeightView);

        articles
    }
}
