use std::env;
use reqwest;
use scraper::{Html, Selector};

/// Simple web scraper that fetches a URL and prints all <h1> titles.
///
/// Usage:
///   cargo run https://example.com
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read CLI arguments (the first arg is the program name, so we skip it)
    let args: Vec<String> = env::args().collect();

    // If the user provided a URL, use it. Otherwise, fall back to a default.
    let url = if args.len() > 1 {
        &args[1]
    } else {
        "https://example.com"
    };

    println!("Fetching HTML from: {}", url);

    // 1. Fetch the page
    let body = reqwest::get(url).await?.text().await?;

    // 2. Parse the HTML document
    let document = Html::parse_document(&body);

    // 3. Select all <h1> elements
    let selector = Selector::parse("h1").unwrap();

    println!("\nFound <h1> titles:");

    // 4. Loop through and print them
    for element in document.select(&selector) {
        let text = element.text().collect::<Vec<_>>().join(" ");
        println!("- {}", text);
    }

    Ok(())
}
