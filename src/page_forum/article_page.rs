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

static NUMBER_PER_PAGE: i32 = 50;
struct CommentPaginator {
    total_comments: i32,
    total_page: i32,
    current_page: i32
}

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


        res_html!("forum/new_article.html", web)
    }

    pub fn article_edit_page(req: &mut Request) -> SapperResult<Response> {
        let mut web = req.ext().get::<AppWebContext>().unwrap();
        let params = get_query_params!(req);
        let id = t_param_parse!(params, "id", Uuid);

        // get article object
        let article = Article::get_by_id(id);
        if article.is_none() {
            return res_400!(format!("no this artile: {}", id);
        }

        let sections = Section::normal_sections();

        web.add("sections", &sections);
        web.add("article", &article);

        res_html!("forum/edit_article.html", web)
    }
    
    pub fn article_detail_page(req: &mut Request) -> SapperResult<Response> {
        let mut web = req.ext().get::<AppWebContext>().unwrap();

        let params = get_query_params!(req);
        let id = t_param_parse!(params, "id", Uuid);
        let comment_page = t_param_parse_default!(params, "comment_page", i32, 1);

        let article = Article::get_by_id(id);
        if article.is_none() {
            return res_400!(format!("no this artile: {}", id));
        }

        // retrieve comments belongs to this article, and calculate its paginator
        let total_comments = Comment::get_comments_count_belong_to_article(id);
        let total_page = math.floor(total_comments / NUMBER_PER_PAGE) + 1;

        let comment_paginator = CommentPaginator {
            total_comments,
            total_page,
            current_page: comment_page
        }

        let comments = Comment::get_comments_paging_belong_to_article(id, comment_page);

        // search a method to do count and do 


        res_html!("forum/article.html", web)
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

        let result = article_create.insert();
        info!("article insert result {:?}", result);

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

        let result = article_edit.update();
        info!("article update result {:?}", result);

        res_redirect!("/p/article?id=".to_string() + id)
    }

}


impl SapperModule for ArticlePage {
    fn before(&self, req: &mut Request) -> SapperResult<()> {
        match permission_need_login(req) {
            Ok(_) => {
                // pass, nothing need to do here
            },
            Err(info) => {
                return res_400!(info)
            }
        }

        Ok(())
    }

    fn router(&self, router: &mut SapperRouter) -> SapperResult<()> {
        router.get("/article", Self::article_detail_page);

        router.get("/p/article/create", Self::article_create_page);
        router.get("/p/article/edit", Self::article_edit_page);
        router.post("/s/article/create", Self::article_create);
        router.post("/s/article/edit", Self::article_edit);

        Ok(())
    }
}


