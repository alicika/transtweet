use error_chain::error_chain;
use once_cell::sync::Lazy;
use reqwest;
use select::document::Document;
use select::predicate::Name;
use std::collections::BTreeMap;

static KEY: Lazy<String> = Lazy::new(|| std::env::var("API_KEY").unwrap_or("DUMMY".to_string()));
static KEY_SEC: Lazy<String> =
    Lazy::new(|| std::env::var("API_KEY_SEC").unwrap_or("DUMMY".to_string()));
static BEARER: Lazy<String> =
    Lazy::new(|| std::env::var("BEARER_TOKEN").unwrap_or("DUMMY".to_string()));

error_chain! {
    foreign_links {
        EnvVar(std::env::VarError);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let t = reqwest::Client::new();

    let v = std::env::args().nth(1);

    // if there is a searching query given as the first argument,
    // then set it as a parameter
    let url = match v {
        None => "https://api.twitter.com/1.1/search/tweets.json?lang=ja&q=twitter".to_string(),
        Some(query) => format!(
            "https://api.twitter.com/1.1/search/tweets.json?lang=ja&q={}",
            query
        ),
    };

    let resp = t
        .get(&url)
        .send()
        .await?
        .json::<BTreeMap<String, String>>()
        .await?;
    println!("{:#?}", resp);

    Ok(())
}

async fn extract_url(source: String) -> Result<Vec<String>> {
    let res = reqwest::get(source).await.expect("Cannot access the resource.").text().await.expect("Cannot parse the received html.");
    let mut urls = Vec::new();

    Document::from(res.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| urls.push(x.to_string()));

    Ok(urls)
}

#[cfg(test)]
mod tests {
//    use super::*;

/*     #[test]
#     fn extract_none() {
#         let u = extract_url("https://httpbin.org".to_string());
#         let compare: Vec<String> = vec![];
#         assert_eq!(u, compare);
#     } */
}