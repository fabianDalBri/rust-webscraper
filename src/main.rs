use std::env;
use reqwest;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Read CLI arguments
    let args: Vec<String> = env::args().collect();

    // 2. Use provided URL or default
    let url = if args.len() > 1 {
        &args[1]
    } else {
        "https://example.com"
    };

    println!("Fetching HTML from: {}", url);

    // 3. Fetch the page
    let body = reqwest::get(url).await?.text().await?;

    // 4. Parse HTML
    let document = Html::parse_document(&body);

    // Feature 1: HEADINGS (H1, H2, H3)
    let heading_levels = ["h1", "h2", "h3"];

    for level in &heading_levels {
        let selector = Selector::parse(level).unwrap();

        println!("\nFound <{}> headings:", level.to_uppercase());

        for element in document.select(&selector) {
            let text = element
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string();

            if !text.is_empty() {
                println!("- {}", text);
            }
        }
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
