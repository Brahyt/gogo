use serde_json::{Value};
mod github;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://swapi.dev/api/people/1")
        .await?
        .text()
        .await?;

    let deserialized: Value = serde_json::from_str(&resp)?;

    github::create_branch("");

    println!("{:#?}", deserialized);
    Ok(())
}
