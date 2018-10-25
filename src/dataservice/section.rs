
use rustorm::DbError;
use crate::db;
use crate::model::Section;
use crate::model::{for_write, for_read};
use crate::util::{random_string, sha3_256_encode};

use uuid::Uuid;

// here to define struct to accept request params
pub struct SectionNew {
    pub title: String,
    pub description: String,
}

// impl some methods on request params structure
impl SectionNew {
    pub fn create(&self) -> Result<Section, String> {
        let new_section = for_write::SectionCreate {
            title: self.title.to_owned(),
            description: self.description.to_owned(),
            stype: 0,
            suser: None
        };

        new_section.insert()
    }

}


// here, we impl some methods for for_insert::Section
impl for_write::SectionCreate {
    pub fn insert(&self) -> Result<Section, String>{
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

// impl retrieving methods on this model, return various views of Section
impl Section {
    pub fn section_by_id(id: Uuid) -> Result<Section, String> {
        let em = db::get_db();
        let clause = format!("where id='{}'", id);
        match db_find!(em, "", "", &clause, Section) {
            Some(sec) => {
                Ok(sec.to_owned())
            },
            None => {
                Err("Insert section error.".to_string())
            }
        }
    }

    pub fn section_by_ids(ids: Vec<Uuid>) -> Vec<Section> {
        let em = db::get_db();
        let ids_str = ids.iter().map(|u| u.to_string()).collect::<Vec<String>>().join(", ");
        let clause = format!("where id in ({})", ids_str);
        let sections = db_select!(em, "", "", &clause, Section);

        sections
    }


}
