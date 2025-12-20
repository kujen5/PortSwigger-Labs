use reqwest::Client;
use scraper::{Html,ElementRef,Selector};
use std::error::Error;

fn check_internal_server_error(html_text: &str) -> bool {
    let document = Html::parse_document(html_text);
    let sel = Selector::parse(r#"p[class="is-warning"]"#).unwrap();
    let text = document
        .select(&sel)
        .next()
        .map(|el| el.text().collect::<Vec<_>>().join(" ").trim().to_string());
    println!("{:?}", &text);

    match text {
        Some(ref t) if t == "Internal Server Error" => false,
        Some(_) => true,
        None => true,
    }
}


fn main() {
    
}
