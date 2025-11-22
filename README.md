# Rust Web Scraper  
A fast, asynchronous web scraper built in Rust.  
Fetches HTML, extracts headings and links, normalizes URLs, and exports the results to a structured JSON file.

---
Features:

Fetch HTML from any URL
Uses `reqwest` and async/await to perform fast HTTP GET requests.

Extract headings (h1, h2, h3)  
Parses HTML with CSS selectors using the `scraper` crate.

Extract and normalize links  
Collects all `<a>` tags and converts relative paths (e.g. `/about`) into fully qualified URLs.

Save results to JSON  
Outputs a clean, structured `scrape_output.json` file using `serde`.

Simple CLI interface  
Run directly using `cargo run <url>`.

---
Technologies Used:

- Rust
- Tokio — async runtime  
- Reqwest — HTTP client  
- Scraper — HTML parsing with CSS selectors  
- Serde / Serde JSON — serialization  
- Url crate — resolving relative links  
