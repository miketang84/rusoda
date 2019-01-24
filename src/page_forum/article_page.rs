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

use crate::dataservice::article::ArticleCreate;
use crate::util::markdown_render;

pub struct ArticlePage;

impl ArticlePage {

    pub fn article_create_page(req: &mut Request) -> SapperResult<Response> {
        let mut web = req.ext().get::<AppWebContext>().unwrap();
        let params = get_query_params!(req);
        let section_id = t_param!(params, "section_id");
        let prev_uri = t_param!(params, "prev_uri");
        //let from = t_param!(params, "from");

        let sections = Section::normal_sections();

        web.add("section_id", section_id);
        web.add("sections", &sections);
        web.add("prev_uri", prev_uri);
        //web.add("from", from);


        res_html!("new_article.html", web)
    }

    pub fn article_edit_page(req: &mut Request) -> SapperResult<Response> {
        let mut web = req.ext().get::<AppWebContext>().unwrap();
        let params = get_query_params!(req);
        let id = t_param_parse!(params, "id", Uuid);

        let sections = Section::normal_sections();

        // get article object


        res_html!("edit_article.html", web)
    }
    
    pub fn article_detail_page(req: &mut Request) -> SapperResult<Response> {
        let mut web = req.ext().get::<AppWebContext>().unwrap();

        res_html!("article.html", web)
    }




    pub fn article_create(req: &mut Request) -> SapperResult<Response> {
        let mut user = req.ext().get::<AppUser>().unwrap();

        let params = get_form_params!(req);
        let section_id = t_param_parse!(params, "section_id", Uuid);
        let title = t_param!(params, "title");
        let tags = t_param!(params, "tags");
        let raw_content = t_param!(params, "raw_content");
        let stype = t_param_parse_default!(params, "stype", i32, 0);
        let prev_uri = t_param_default!(params, "prev_uri", "/");

        let content = markdown_render(raw_content);

        let article_create = ArticleCreate {
            title,
            tags,
            section_id,
            author_id: user.id,
            raw_content,
            content,
            stype,
            status: 0,
        }

        article_create.insert();

        res_redirect!(prev_uri)
    }

    pub fn article_edit(req: &mut Request) -> SapperResult<Response> {
        let mut user = req.ext().get::<AppUser>().unwrap();

        let params = get_form_params!(req);
        let id = t_param_parse!(params, "id", Uuid);
        let section_id = t_param_parse!(params, "section_id", Uuid);
        let title = t_param!(params, "title");
        let tags = t_param!(params, "tags");
        let raw_content = t_param!(params, "raw_content");

        let content = markdown_render(raw_content);

        let article_edit = ArticleEdit {
            id,
            section_id,
            title,
            tags,
            raw_content,
            content,
        }

        article_edit.update();

        res_redirect!("/p/article?id=".to_string() + id)
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

        router.post("/s/article/create", Self::article_create);
        router.post("/s/article/edit", Self::article_edit);

        Ok(())
    }
}


