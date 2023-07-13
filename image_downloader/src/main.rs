use reqwest;
use scraper::{Html, Selector};
use std::fs::File;
use std::io::copy;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://nyahentai.re/fanzine/re2590036/"; // URLを指定

    let resp = reqwest::get(url).await?;
    let body = resp.text().await?;

    let fragment = Html::parse_document(&body);
    let selector = Selector::parse("#post-comic img").unwrap(); // idがpost-comicの要素内のimgタグを選択

    for element in fragment.select(&selector) {
        let img_url = element.value().attr("src").unwrap(); // src属性を取得
        let resp = reqwest::get(img_url).await?;
        let mut out = File::create("/Users/metolone/Rust_Files/image_downloader")?; // 保存するパスを指定
        let mut content = resp.bytes().await?;
        copy(&mut content.as_ref(), &mut out)?; // ファイルをコピー
    }

    Ok(())
}
