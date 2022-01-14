use error_chain::error_chain;
use once_cell::sync::Lazy;
use reqwest;
use reqwest::header::{Authorization, UserAgent};
use std::collections::HashMap;

static API_KEY: Lazy<Result<String, _>> = Lazy::new(|| std::env::var(API_KEY));
static API_KEY_SEC: Lazy<Result<String, _>> = Lazy::new(|| std::env::var(API_KEY_SEC));
static BEARER: Lazy<Result<String, _>> = Lazy::new(|| std::env::var(BEARER_TOKEN));

error_chain! {
    foreign_links {
        EnvVar(env::VarError);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let t = reqwest::Client::new();

    let v = std::env::args().nth(1);

    // if there is a searching query given as the first argument,
    // then set it as a parameter
    let url = match v {
        None => "https://api.twitter.com/1.1/search/tweets.json?lang=ja".to_string(),
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
