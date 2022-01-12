use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use reqwest;
use once_cell::sync::Lazy;

static API_KEY: Lazy<String> = Lazy::new(|| std::env::var(API_KEY));
static API_KEY_SEC: Lazy<String> = Lazy::new(|| std::env::var(API_KEY_SEC));
static BEARER: Lazy<String> = Lazy::new(|| std::env::var(BEARER_TOKEN));

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

    //let url = Url::parse(&url)?;
    //let token = std::env::var("TOKEN").unwrap();
    //let token = format!("Bearer {}", token);
    //let client = reqwest::Client::new().get(&url).header("authorization", token);
    //println!("{:#?}", client);

    let resp = t
        .get(&url)
        .send()
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);

    Ok(())
}
