use sapper::{
    Request, 
    Response, 
    Result as SapperResult, 
    Error as SapperError, 
    Module as SapperModule,
    Router as SapperRouter};
use sapper_std::{JsonParams, SessionVal};
use serde_json;

pub struct UserPage;

impl UserPage {

    pub fn user_register_page(req: &mut Request) -> SapperResult<Response> {

    }

    pub fn user_login_page(req: &mut Request) -> SapperResult<Response> {

    }

    pub fn user_modifypwd_page(req: &mut Request) -> SapperResult<Response> {

    }

    pub fn user_detail_page(req: &mut Request) -> SapperResult<Response> {

    }
}


impl SapperModule for UserPage {
    fn before(&self, req: &mut Request) -> SapperResult<()> {

    }

    fn router(&self, router: &mut SapperRouter) -> SapperResult<()> {

        router.get("/p/user/register", Self::user_register_page);
        router.get("/p/user/login", Self::user_login_page);
        router.get("/p/user/modifypwd", Self::user_modifypwd_page);
        router.get("/p/user", Self::user_detail_page);

        Ok(())
    }
}


