use rss::{
    Item,
    ItemBuilder,
    ChannelBuilder,
    Channel
};

use crate::dataservice::article::Article;
use std::env;

pub fn make_rss_feed() -> String {
    let host_domain = env::var("HOST_DOMAIN").expect("HOST_DOMAIN must be set");
    let mut channel = ChannelBuilder::default()
        .title("Rust.cc")
        .link("https://rust.cc")
        .description("This Is Rust Crustacean Community RSS feed.")
        .build()
        .unwrap();

    let articles = Article::get_latest_full_articles(10);
    let blog_articles = Article::get_latest_full_blog_articles(10);

    let mut items: Vec<Item> = vec![];
    for article in articles {
        let item = ItemBuilder::default()
        .title(article.title)
            .link(host_domain.clone() + "/article?id=" + &article.id.to_string())
            .description(article.content)
            .pub_date(article.created_time.format("%Y-%m-%d %H:%M:%S").to_string())
            .build()
            .unwrap();

        items.push(item);
    }

    for article in blog_articles {
        let item = ItemBuilder::default()
        .title(article.title)
            .link(host_domain.clone() + "/article?id=" + &article.id.to_string())
            .description(article.content)
            .pub_date(article.created_time.format("%Y-%m-%d %H:%M:%S").to_string())
            .build()
            .unwrap();

        items.push(item);
    }

    channel.set_items(items);

    channel.to_string()
}