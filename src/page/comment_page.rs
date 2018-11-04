use sapper::{
    Request, 
    Response, 
    Result as SapperResult, 
    Error as SapperError, 
    Module as SapperModule,
    Router as SapperRouter};
use sapper_std::{JsonParams, SessionVal};
use serde_json;

pub struct CommentPage;

impl CommentPage {

    pub fn comment_edit_page(req: &mut Request) -> SapperResult<Response> {

    }
    

}


impl SapperModule for CommentPage {
    fn before(&self, req: &mut Request) -> SapperResult<()> {

    }

    fn router(&self, router: &mut SapperRouter) -> SapperResult<()> {
        router.get("/p/section/edit", Self::comment_edit_page);

        Ok(())
    }
}


