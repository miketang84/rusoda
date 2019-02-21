use sapper::{
    Request, 
    Response, 
    Result as SapperResult, 
    Error as SapperError, 
    Module as SapperModule,
    Router as SapperRouter};
use sapper_std::{JsonParams, SessionVal, render};
use crate::serde_json;

use crate::db;
// introduce macros
use sapper_std::res_html;
use crate::AppWebContext;

pub struct SectionPage;

impl SectionPage {

    pub fn section_create_page(req: &mut Request) -> SapperResult<Response> {
        let mut web = req_ext_entity!(AppWebContext).unwrap();

        res_html!("forum/new_section.html", web)
    }

    pub fn section_edit_page(req: &mut Request) -> SapperResult<Response> {
        let mut web = req_ext_entity!(AppWebContext).unwrap();

        res_html!("forum/edit_section.html", web)
    }
    
    pub fn section_detail_page(req: &mut Request) -> SapperResult<Response> {
        let mut web = req_ext_entity!(AppWebContext).unwrap();

        let params = get_form_params!(req);
        let section_id = t_param!(params, "id", Uuid);

        let section_result = Section::section_by_id(section_id);
        if section_result.is_err() {
            return res_404!("Not Found");
        }
        
        let section = section_result.unwrap();
        let is_a_blog = false;
        if section.stype == 1 {
            is_a_blog = true;
        }
        let is_myown_blog = false;
        let is_admin = false;
        match req_ext_entity!(req, AppUser) {
            Some(user) => {
                if section.suser == Some(user.id) {
                    is_myown_blog = true;
                }
                if user.role >= 9 {
                    is_admin = true;
                }
            },
            None => {}
        }
        
        let current_page
        let total_page
        let total_item


        let articles



        res_html!("forum/section.html", web)
    }





    pub fn section_create(req: &mut Request) -> SapperResult<Response> {
        let params = get_form_params!(req);
        let title = t_param!(params, "title");
        let description = t_param!(params, "description");

        let section_new = SectionNew {
            title,
            description
        };

        match section_new.create() {
            Ok(section) => {
                res_redirect!(format!("/p/section?id={}", section.id))
            },
            Err(_) => {
                res_500!("section create error.")
            }
        }  
    }

    pub fn section_edit(req: &mut Request) -> SapperResult<Response> {
        let params = get_form_params!(req);
        let id = t_param_parse!(params, "id", Uuid);
        let title = t_param!(params, "title");
        let description = t_param!(params, "description");

        let section_edit = SectionEdit {
            id,
            title,
            description
        };

        match section_edit.update() {
            Ok(section) => {
                res_redirect!(format!("/p/section?id={}", section.id))
            },
            Err(_) => {
                res_500!("section edit error.")
            }
        }  
    }

}


impl SapperModule for SectionPage {
    fn before(&self, req: &mut Request) -> SapperResult<()> {
        let (path, _) = req.uri();
        if path.starts_with("/s/") {
            match req_ext_entity!(req, AppUser) {
                Some(ref user) => {
                    if user.role >= 9 {
                        // pass, nothing need to do here
                    }
                    else {
                        return res_400!("No permissions.")
                    }
                },
                None => {
                    return res_400!("No permissions.")
                }
            }
        }

        Ok(())
    }

    fn router(&self, router: &mut SapperRouter) -> SapperResult<()> {
        router.get("/p/section/create", Self::section_create_page);
        router.get("/p/section/edit", Self::section_edit_page);
        router.get("/p/section", Self::section_detail_page);

        router.post("/s/section/create", Self::section_create);
        router.post("/s/section/edit", Self::section_edit);


        Ok(())
    }
}


