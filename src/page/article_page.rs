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

pub struct ArticlePage;

impl ArticlePage {

    pub fn article_create_page(req: &mut Request) -> SapperResult<Response> {
        let mut web = req.ext().get::<AppWebContext>().unwrap();

        res_html!("article_create_page.html", web)
    }

    pub fn article_edit_page(req: &mut Request) -> SapperResult<Response> {
        let mut web = req.ext().get::<AppWebContext>().unwrap();

        res_html!("article_edit_page.html", web)
    }
    
    pub fn article_detail_page(req: &mut Request) -> SapperResult<Response> {
        let mut web = req.ext().get::<AppWebContext>().unwrap();

        res_html!("article_detail_page.html", web)
    }

}


impl SapperModule for ArticlePage {
    fn before(&self, req: &mut Request) -> SapperResult<()> {

        Ok(())
    }

    fn router(&self, router: &mut SapperRouter) -> SapperResult<()> {
        router.get("/p/article/create", Self::article_create_page);
        router.get("/p/article/edit", Self::article_edit_page);
        router.get("/p/article", Self::article_detail_page);

        Ok(())
    }
}


