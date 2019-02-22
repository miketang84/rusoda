
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
    SectionDelete
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

    pub fn get_by_ids(ids: Vec<Uuid>) -> Vec<Section> {
        let em = db::get_db();
        let ids_str = ids.iter().map(|u| u.to_string()).collect::<Vec<String>>().join(", ");
        let clause = format!("where id in ({})", ids_str);
        let sections = db_select!(em, "", "", &clause, Section);

        sections
    }

    pub fn normal_sections() -> Vec<Section> {
        let em = db::get_db();
        let clause = "where stype=0";
        let sections = db_select!(em, "", "", "where stype=0", Section);

        sections
    }

}
