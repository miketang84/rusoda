
use rustorm::DbError;
use crate::db;
use crate::model::Section;
use crate::model::{for_write, for_read};
use crate::util::{random_string, sha3_256_encode};

use uuid::Uuid;


pub struct SectionNew {
    pub title: String,
    pub description: String,
}

pub use crate::model::for_write::{
    SectionCreate,
    SectionEdit,
    SectionDelete,
    UpdateSectionWeight,
};

pub use self::Section;

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
        let clause = format!("where id={}", self.id);
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
        let clause = format!("where id={}", self.id);
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
        let clause = format!("where id={}", self.id);
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
        let clause = "where stype=0 order by weight desc";
        let sections = db_select!(em, "", "",&clause, Section);

        sections
    }

    pub fn forum_sections_orderby_createdtime() -> Vec<Section> {
        let em = db::get_db();
        let clause = "where stype=0 order by created_time desc";
        let sections = db_select!(em, "", "",&clause, Section);

        sections
    }

    pub fn get_articles_paging_belong_to_this(section_id: Uuid, current_page: usize) -> Vec<ArticleForList> {
        let em = db::get_db();

        let offset = NUMBER_PER_PAGE * (current_page - 1);

        let clause = format!("where section_id={} order by created_time desc limit {} offset {}", section_id, NUMBER_PER_PAGE, offset);
        let articles = db_select!(em, "", "", &clause, ArticleForList);

        articles
    }

    pub fn get_articles_count_belong_to_this(section_id: Uuid) -> i32 {
        let em = db::get_db();
        let clause = format!("where section_id={}", section_id);
        let count = db_find!(em, "count(*)", "", &clause, Article);

        count
    }

}
