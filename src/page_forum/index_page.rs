use sapper::{
    Request, 
    Response, 
    Result as SapperResult, 
    Error as SapperError, 
    Module as SapperModule,
    Router as SapperRouter};
use sapper_std::*;
use crate::serde_json;

use crate::db;
// introduce macros
use sapper_std::res_html;
use crate::AppWebContext;

use crate::constants::NUMBER_ARTICLE_PER_PAGE;
use crate::dataservice::article::Article;
use crate::dataservice::section::Section;

pub struct IndexPage;

impl IndexPage {

    pub fn index(req: &mut Request) -> SapperResult<Response> {
        let mut web = ext_type_owned!(req, AppWebContext).unwrap();
        let db_conn = db::get_db();
        let redis_conn = db::get_redis();

        let articles = Article::get_latest_articles(NUMBER_ARTICLE_PER_PAGE);

        let blog_articles = Article::get_latest_blog_articles(NUMBER_ARTICLE_PER_PAGE);

        // get all configured index displaying sections
        // and latest commented three articles 
        let sections = Section::forum_sections();

        web.insert("articles", &articles);
        web.insert("blog_articles", &blog_articles);
        web.insert("sections", &sections);

        res_html!("forum/index.html", web)
    }

}


impl SapperModule for IndexPage {
    fn before(&self, req: &mut Request) -> SapperResult<()> {
        
        Ok(())
    }

    fn router(&self, router: &mut SapperRouter) -> SapperResult<()> {
        router.get("/", Self::index);

        Ok(())
    }
}


