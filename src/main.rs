use std::env;
use reqwest;
use scraper::{Html, Selector};
use serde::Serialize;

#[derive(Serialize)]
struct LinkInfo {
    text: String,
    href: String,
}

#[derive(Serialize)]
struct ScrapeResult {
    url: String,
    headings: Headings,
    links: Vec<LinkInfo>,
}

#[derive(Serialize)]
struct Headings {
    h1: Vec<String>,
    h2: Vec<String>,
    h3: Vec<String>,
}

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
    let mut headings = Headings {
        h1: Vec::new(),
        h2: Vec::new(),
        h3: Vec::new(),
    };

    let heading_levels = [
        ("h1", &mut headings.h1),
        ("h2", &mut headings.h2),
        ("h3", &mut headings.h3),
    ];

    for (level, store) in heading_levels {
        let selector = Selector::parse(level).unwrap();

        for element in document.select(&selector) {
            let text = element
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string();

            if !text.is_empty() {
                store.push(text);
            }
        }
    }

    // Feature 2: SCRAPE LINKS
    let mut links: Vec<LinkInfo> = Vec::new();

    let link_selector = Selector::parse("a").unwrap();

    for element in document.select(&link_selector) {
        let link_text = element
            .text()
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_string();

        if let Some(href) = element.value().attr("href") {
            links.push(LinkInfo {
                text: link_text,
                href: href.to_string(),
            });
        }
    }

    let result = ScrapeResult {
        url: url.to_string(),
        headings,
        links,
    };

    let json_string = serde_json::to_string_pretty(&result)?;
    std::fs::write("output.json", json_string)?;

    println!("\nSaved results to output.json");

    Ok(())
}
