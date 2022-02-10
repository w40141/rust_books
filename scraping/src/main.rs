use anyhow::Result;
use tokio;

fn main() -> Result<()> {
    body = get("https://www.rust-lang.org")
    // let body = reqwest::blocking::get("https://www.rust-lang.org")?.text()?;
    println!("body = {res:?}");
    Ok(())
}

async fn get(url: String) -> String {
    reqwest::get(url)
        .await?
        .text()
        .await?
}
