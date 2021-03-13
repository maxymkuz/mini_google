//! A scrapper module intended to work together with the `ThreadPool`'s workers.
//!
//! Scrapes the page for needed data and returns it back to the main thread.
use crate::{Result, ScrapeData, ScrapeRes};
use reqwest::Url;
use select::document::Document;
use select::predicate::{Element, Name, Predicate};
use std::collections::BTreeSet;

/// A scraper for a single webpage. Creates an HTTP client to connect to it,
/// downloads the page, parses it and scrapes all links from it, returning them
/// in a valid format. Also scrapes structured data from the page.
pub async fn scrape(scrape_data: ScrapeData) -> Result<ScrapeRes> {
    let (webpage_url, user_agent, high_level_domain) = (
        &scrape_data.webpage_url,
        &scrape_data.user_agent,
        &scrape_data.high_level_domain,
    );

    // Creating an asynchronous `reqwest` Client to send HTTP requests with
    let client = reqwest::Client::builder()
        .user_agent(user_agent)
        .build()
        .unwrap();

    // Sending an asynchronous get request, unwrapping the `Result` we get
    let res = client.get(webpage_url).send().await?.text().await?;

    let res = Document::from(&res[..]);

    // Scrapping all text from the page
    let full_text = get_full_text(&res);

    // Finding all links on the page
    let all_links = get_links_from_html(&res, &webpage_url, &high_level_domain);

    // Searching for structured data on the page.
    // We are looking for <script type="application/ld+json"> and we need all of its contents
    let structured_data: String = res
        .find(Name("script"))
        .filter(|n| n.attr("type") == Some("application/ld+json"))
        .map(|x| x.text())
        .nth(0)
        .unwrap_or("The page didn't have structured data".to_string());
    let webpage = webpage_url.to_string();

    Ok(ScrapeRes {
        webpage,
        all_links,
        structured_data,
        full_text,
    })
}

/// Receives an HTML text file, parses it and returns only the text on it without
/// the tags and attributes
fn get_full_text(html: &Document) -> String {
    let blacklist: BTreeSet<&'static str> =
        ["style", "html", "meta", "head", "script", "p", "a", "head"]
            .iter()
            .cloned()
            .collect();
    html.find(Element)
        .filter(|n| match n.name() {
            Some(name) => !blacklist.contains(name),
            None => true,
        })
        .map(|x| x.text())
        .collect::<Vec<String>>()
        .join(" ")
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
}

/// Looks for all elements in the HTML body that are valid links, returning unique ones.
/// Requires the `base_url` parameter - the page from which the link was collected to
/// work with relative links properly, and `high_level_domain` parameter to discard
/// links we do not want to crawl at all.
fn get_links_from_html(
    html: &Document,
    base_url: &str,
    high_level_domain: &str,
) -> BTreeSet<String> {
    html.find(Name("a").or(Name("link")))
        .filter_map(|n| n.attr("href"))
        .filter_map(|x| normalize_url(x, base_url, high_level_domain))
        .collect::<BTreeSet<String>>()
}

/// Checks whether the URL was valid and whether it has a host and whether
/// this host is the high level domain we accept
fn normalize_url(url: &str, base_url: &str, high_level_domain: &str) -> Option<String> {
    let base = match Url::parse(base_url) {
        Ok(base) => base,
        Err(_) => return None,
    };
    let mut joined = match base.join(url) {
        Ok(joined) => joined,
        Err(_) => return None,
    };

    // Delete the '#' fragment from the url string
    joined.set_fragment(None);

    if joined.has_host() && joined.host_str().unwrap() == high_level_domain {
        Some(joined.to_string())
    } else {
        None
    }
}
