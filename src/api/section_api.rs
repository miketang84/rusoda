use sapper::{
    Request, 
    Response, 
    Result as SapperResult, 
    Error as SapperError, 
    Module as SapperModule,
    Router as SapperRouter};
use sapper_std::{JsonParams, SessionVal};
use serde_json;

pub struct SectionApi;

impl SectionApi {

    pub fn article_create(req: &mut Request) -> SapperResult<Response> {

    }

    pub fn article_edit(req: &mut Request) -> SapperResult<Response> {

    }

    pub fn article_delete(req: &mut Request) -> SapperResult<Response> {

    }

    pub fn article_get_by_id(req: &mut Request) -> SapperResult<Response> {

    }

    pub fn article_paging(req: &mut Request) -> SapperResult<Response> {

    }

    pub fn article_paging_by_section(req: &mut Request) -> SapperResult<Response> {

    }
}


impl SapperModule for SectionApi {
    fn before(&self, req: &mut Request) -> SapperResult<()> {
        // check permission, logined
        // Post request needs logining first
        

    }

    fn router(&self, router: &mut SapperRouter) -> SapperResult<()> {
        router.post("/section/create", Self::section_create);
        router.post("/section/edit", Self::section_edit);
        router.post("/section/delete", Self::section_delete);

        router.get("/section/get_by_id", Self::section_get_by_id);
        router.get("/section/get_by_ids", Self::section_get_by_ids);

        Ok(())
    }
}


