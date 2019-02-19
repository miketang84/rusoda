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

pub struct UserPage;

impl UserPage {

    pub fn user_register_page(req: &mut Request) -> SapperResult<Response> {
        let mut web = req.ext().get::<AppWebContext>().unwrap();

        res_html!("forum/user_register_page.html", web)
    }

    pub fn user_login_page(req: &mut Request) -> SapperResult<Response> {
        let mut web = req.ext().get::<AppWebContext>().unwrap();

        res_html!("forum/user_login_page.html", web)
    }

    pub fn user_modifypwd_page(req: &mut Request) -> SapperResult<Response> {
        let mut web = req.ext().get::<AppWebContext>().unwrap();

        res_html!("forum/user_modifypwd_page.html", web)
    }

    pub fn user_detail_page(req: &mut Request) -> SapperResult<Response> {
        let mut web = req.ext().get::<AppWebContext>().unwrap();

        res_html!("forum/user_detail_page.html", web)
    }
}


impl SapperModule for UserPage {
    fn before(&self, req: &mut Request) -> SapperResult<()> {

        Ok(())
    }

    fn router(&self, router: &mut SapperRouter) -> SapperResult<()> {

        router.get("/p/user/register", Self::user_register_page);
        router.get("/p/user/login", Self::user_login_page);
        router.get("/p/user/modifypwd", Self::user_modifypwd_page);
        router.get("/p/user", Self::user_detail_page);

        router.get("/p/user/__admin_login__", Self::user_admin_login);

        Ok(())
    }
}


