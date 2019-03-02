
pub fn make_rss_feed(req: &mut Request) -> String {

    let articles = Article::get_latest_articles(10);
    let blog_articles = Article::get_latest_blog_articles(10);


    let mut channel = ChannelBuilder::default()
        .title("Rust.cc")
        .link("https://Rust.cc")
        .description("An RSS feed.")
        .build()
        .unwrap();

    let mut item = ItemBuilder::default()
        .title("xxxxx".to_string())
        .link("yyy".to_string())
        .description("zzz.".to_string())
        .build()
        .unwrap();


    channel.set_items(vec![item]);

    println!("{}", channel.to_string());



}