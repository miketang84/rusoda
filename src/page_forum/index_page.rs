use sapper::{
    Request, 
    Response, 
    Result as SapperResult, 
    Error as SapperError, 
    Module as SapperModule,
    Router as SapperRouter};
use sapper_std::{
    JsonParams,
    SessionVal,
    render
};
use crate::serde_json;

use crate::db;
// introduce macros
use sapper_std::res_html;
use crate::AppWebContext;

pub struct IndexPage;

impl IndexPage {

    pub fn index(req: &mut Request) -> SapperResult<Response> {
        let mut web = req.ext().get::<AppWebContext>().unwrap();
        let db_conn = db::get_db();
        let redis_conn = db::get_redis();

        // get latest 10 articles and digests
        // let latest_articles = ....
        


        // get all configured index displaying sections
        // and latest commented three articles 
        // let 


        res_html!("index.html", web)
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


