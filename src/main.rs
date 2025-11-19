use reqwest;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // URL to scrape
    let url = "https://example.com/";

    println!("Fetching HTML from: {}", url);

    // 1. Fetch the page
    let body = reqwest::get(url).await?.text().await?;

    // 2. Parse the HTML document
    let document = Html::parse_document(&body);

    // 3. Select all <h1> elements
    let selector = Selector::parse("h1").unwrap();

    println!("\nFound H1 titles:");

    // 4. Loop through and print them
    for element in document.select(&selector) {
        let text = element.text().collect::<Vec<_>>().join(" ");
        println!("- {}", text);
    }

    Ok(())
}

