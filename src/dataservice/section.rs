
use rustorm::DbError;
use crate::db;
use crate::model::Section;
use crate::model::{for_insert, for_retrieve};
use crate::util::{random_string, sha3_256_encode};


// here to define struct to accept request params
pub struct SectionNew {
    


}

// impl some methods on request params structure
impl SectionNew {
    pub fn new() {
    
    }

}


// here, we impl some methods for for_insert::Section
impl for_insert::Section {
    pub fn insert(&self) {


    }

}

// impl retrieving methods on this model, return various views of Section
impl Section {


}
