use std::env;
use reqwest;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read CLI arguments
    let args: Vec<String> = env::args().collect();

    // Use provided URL or default
    let url = if args.len() > 1 {
        &args[1]
    } else {
        "https://example.com"
    };

    println!("Fetching HTML from: {}", url);

    // Fetch the page
    let body = reqwest::get(url).await?.text().await?;

    // Parse HTML
    let document = Html::parse_document(&body);

    // Feature 1: SCRAPE <h1>
    let h1_selector = Selector::parse("h1").unwrap();

    println!("\nFound <h1> titles:");
    for element in document.select(&h1_selector) {
        let text = element.text().collect::<Vec<_>>().join(" ");
        println!("- {}", text);
    }

    // Feature 2: SCRAPE LINKS
    let link_selector = Selector::parse("a").unwrap();

    println!("\nFound links:");
    for element in document.select(&link_selector) {
        let link_text = element
            .text()
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_string();

        if let Some(href) = element.value().attr("href") {
            println!("- [{}] ({})", link_text, href);
        }
    }

    Ok(())
}
