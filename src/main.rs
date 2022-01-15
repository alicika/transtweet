use error_chain::error_chain;
use once_cell::sync::Lazy;
use reqwest;
use reqwest::header::Authorization;
use std::collections::HashMap;

static API_KEY: Lazy<String> = Lazy::new(|| std::env::var(API_KEY).unwrap_or("DUMMY"));
static API_KEY_SEC: Lazy<String> = Lazy::new(|| std::env::var(API_KEY_SEC).unwrap_or("DUMMY"));
static BEARER: Lazy<String> = Lazy::new(|| std::env::var(BEARER_TOKEN).unwrap_or("DUMMY"));

error_chain! {
    foreign_links {
        EnvVar(env::VarError);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<(), foreign_links> {
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
        .bearer_auth(from_env("BEARER_TOKEN"))
        .send()
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);

    Ok(())
}
