use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::env;
use std::str;
use std::{thread, time};

#[derive(Serialize, Deserialize, Debug)]
struct TweetData {
    author_id: String,
    created_at: String,
    id: String,
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct MatchingRule {
    tag: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    username: String,
    created_at: String,
    id: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Includes {
    users: Vec<User>
}

#[derive(Serialize, Deserialize, Debug)]
struct Tweet {
    data: TweetData,
    matching_rules: Vec<MatchingRule>,
    includes: Includes
}

async fn set_rules(token: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = reqwest::Client::new();
    let body = client
        .post("https://api.twitter.com/2/tweets/search/stream/rules")
        .header("content-type", "application/json")
        .header("authorization", format!("bearer {}", token))
        .body(r#"{"add":[{"value":"election", "tag": "election"}]}"#)
        .send()
        .await?
        .text()
        .await?;

    println!("{}", body);

    Ok(())
}

async fn search_tweets(
    token: &str,
) -> Result<reqwest::Response, Box<dyn std::error::Error + Send + Sync>> {
    let _url = "https://api.twitter.com/2/tweets/search/stream";
    let _fields = "tweet.fields=created_at&expansions=author_id&user.fields=created_at";
    let url: &str = &format!("{}?{}", _url, _fields)[..];
    let client = reqwest::Client::new();
    let resp = client
        .get(url)
        .header("content-type", "application/json")
        .header("authorization", format!("bearer {}", token))
        .send()
        .await?;

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let token = env::var("TWITTER_BEARER_TOKEN").expect("Twitter Bearer Token is needed");

    set_rules(&token).await?;
    let mut stream = search_tweets(&token).await?.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let data = &chunk?;
        let length = &data.len();
        let content = str::from_utf8(&data)?;
        let tweet: Tweet = serde_json::from_str(&content)?;
        println!("Length: {}, Content: {:#?}", length, tweet);

        let one_second = time::Duration::from_millis(1000);
        thread::sleep(one_second);
    }

    Ok(())
}
