use sapper::{
    Request, 
    Response, 
    Result as SapperResult, 
    Error as SapperError, 
    Module as SapperModule,
    Router as SapperRouter};
use sapper_std::{JsonParams, SessionVal};
use serde_json;

pub struct SectionPage;

impl SectionPage {

    pub fn section_create_page(req: &mut Request) -> SapperResult<Response> {

    }

    pub fn section_edit_page(req: &mut Request) -> SapperResult<Response> {

    }
    
    pub fn section_detail_page(req: &mut Request) -> SapperResult<Response> {

    }

}


impl SapperModule for SectionPage {
    fn before(&self, req: &mut Request) -> SapperResult<()> {
        

    }

    fn router(&self, router: &mut SapperRouter) -> SapperResult<()> {
        router.get("/p/section/create", Self::section_create_page);
        router.get("/p/section/edit", Self::section_edit_page);
        router.get("/p/section", Self::section_detail_page);

        Ok(())
    }
}


