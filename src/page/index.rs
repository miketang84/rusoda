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
    res_html
};
use serde_json;

pub struct IndexPage;

impl IndexPage {

    pub fn index(req: &mut Request) -> SapperResult<Response> {
        let mut web = req.ext().get::<WebContent>().unwrap();
        let redis_conn = get_redis!();
        let db_conn = get_db!();

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

    }

    fn router(&self, router: &mut SapperRouter) -> SapperResult<()> {
        router.get("/", Self::index);

        Ok(())
    }
}


