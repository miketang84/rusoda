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

    pub fn comment_new_page(req: &mut Request) -> SapperResult<Response> {
        let mut web = reqext!(req, AppWebContext).unwrap();

        res_html!("forum/index.html", web)
    }

    pub fn comment_edit_page(req: &mut Request) -> SapperResult<Response> {
        let mut web = reqext!(req, AppWebContext).unwrap();

        res_html!("forum/index.html", web)
    }


    pub fn comment_new(req: &mut Request) -> SapperResult<Response> {

    }

    pub fn comment_edit(req: &mut Request) -> SapperResult<Response> {

    }


}


impl SapperModule for CommentPage {
    fn before(&self, req: &mut Request) -> SapperResult<()> {

        Ok(())
    }

    fn router(&self, router: &mut SapperRouter) -> SapperResult<()> {
        router.get("/p/comment/new", Self::comment_new_page);
        router.get("/p/comment/edit", Self::comment_edit_page);
        
        router.get("/s/comment/new", Self::comment_new);
        router.get("/s/comment/edit", Self::comment_new);
        

        Ok(())
    }
}


