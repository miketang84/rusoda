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

pub struct CommentPage;

impl CommentPage {

    pub fn comment_edit_page(req: &mut Request) -> SapperResult<Response> {
        let mut web = req.ext().get::<AppWebContext>().unwrap();

        res_html!("index.html", web)
    }

}


impl SapperModule for CommentPage {
    fn before(&self, req: &mut Request) -> SapperResult<()> {

        Ok(())
    }

    fn router(&self, router: &mut SapperRouter) -> SapperResult<()> {
        router.get("/p/section/edit", Self::comment_edit_page);

        Ok(())
    }
}


