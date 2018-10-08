use sapper::{
    Result as SapperResult, 
    Error as SapperError, 
    Request, 
    Response, 
    SapperModule,
    SapperRouter};
use sapper_std::{JsonParams, SessionVal};
use serde_json;

pub struct ArticleApi;

impl User {

}

impl SapperModule for ArticleApi {
    fn before(&self, req: &mut Request) -> SapperResult<()> {
        // check permission, logined
        // Post request needs logining first
        

    }

    fn router(&self, router: &mut SapperRouter) -> SapperResult<()> {
        router.post("/article/new", Self::article_new);
        router.post("/article/edit", Self::article_edit);
        router.post("/article/delete", Self::article_delete);

        router.get("/article/get_by_id", Self::article_get_by_id);
        router.get("/article/paging", Self::article_paging);
        router.get("/article/paging_by_section", Self::article_paging_by_section);

        Ok(())
    }
}


