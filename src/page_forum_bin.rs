
use sapper::{
    App as SapperApp,
    Smock as SapperSmock,
    Result as SapperResult,
    Request,
    Response,
    Key
};
use sapper_std::{
    WebContext,
    SessionVal
};

use serde;
use serde_json;

#[macro_use] mod helper_macros;
mod db;
mod model;
mod dataservice;
mod util;
mod middleware;


// include page modules
mod page_forum;


pub struct AppWebContext;
impl Key for AppWebContext { 
    type Value = WebContext;
}   

pub struct AppUser;
impl Key for AppUser { 
    type Value = WebContext;
}   


// define global smock
struct PageForum;

impl SapperSmock for WebPage {
    fn before(&self, req: &mut Request) -> SapperResult<()> {
        // define cookie prefix
        sapper_std::init(req, Some("rusoda_session"))?;
        // init web instance state
        let mut web = WebContext::new();
        // we can add something to web

        match req.ext().get::<SessionVal>() {
            Some(cookie) => {
                // using this cookie to retreive user instance
                match Ruser::get_user_by_cookie(&cookie) {
                    Ok(user) => {
                        web.add("user", &user);
                        req.ext_mut().insert::<AppUser>(user);
                    },
                    Err(_) => {}
                }
            },
            None => {}
        }

        // insert it to req
        req.ext_mut().insert::<AppWebContext>(web);

        Ok(())
    }

    fn after(&self, req: &Request, res: &mut Response) -> SapperResult<()> {
        sapper_std::finish(req, res)?;
        Ok(())
    }
}

fn main () {
    let addr = "0.0.0.0";
    let port = 8081;
    let mut app = SapperApp::new();
    app.address(addr)
        .port(port)
        .with_smock(Box::new(PageForum))
        .add_module(Box::new(page_forum::index_page::IndexPage))
        .add_module(Box::new(page_forum::user_page::UserPage))
        .add_module(Box::new(page_forum::section_page::SectionPage))
        .add_module(Box::new(page_forum::article_page::ArticlePage))
        .add_module(Box::new(page_forum::comment_page::CommentPage))
        .static_file_service(true);

    println!("Start listen on http://{}:{}", addr, port);
    app.run_http();

}
