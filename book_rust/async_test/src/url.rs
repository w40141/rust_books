use scraper::Selector;
use std::{fs::File, io::Write};
use tokio::time;

pub async fn download_image(title: &str) {
    let shodou_url = "https://uta.pw/shodou";
    let url = format!(
        "{shodou_url}/index.php?titles&show&title={}",
        urlencoding::encode(title)
    );
    println!("get: {url}");
    let html = reqwest::get(url).await.unwrap().text().await.unwrap();
    let doc = scraper::Html::parse_document(&html);
    let sel = Selector::parse(".articles img").unwrap();
    for (i, node) in doc.select(&sel).enumerate() {
        let src = node.value().attr("src").unwrap();
        let img_url = format!("{shodou_url}/{src}");
        println!("{img_url}");
        let filename = format!("shodou_{title}_{i}.png");
        let bytes = reqwest::get(img_url).await.unwrap().bytes().await.unwrap();
        let mut file = File::create(filename).unwrap();
        file.write_all(&bytes).unwrap();
        time::sleep(time::Duration::from_millis(1000)).await;
    }
}
