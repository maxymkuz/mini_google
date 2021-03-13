//! A scrapper module intended to work together with the `ThreadPool`'s workers.
//!
//! Scrapes the page for needed data and returns it back to the main thread.
use crate::{ScrapeData, ScrapeParam};
use reqwest::Url;
use select::document::Document;
use select::predicate::{Element, Name, Predicate};
use std::collections::BTreeSet;

/// A scraper for a single webpage. Creates an HTTP client to connect to it,
/// downloads the page, parses it and scrapes all links from it, returning them
/// in a valid format. Also scrapes structured data from the page.
pub async fn scrape(scrape_data: ScrapeParam) -> std::result::Result<ScrapeData, String> {
    let (webpage_url, user_agent, high_level_domain) = (
        &scrape_data.webpage_url,
        &scrape_data.user_agent,
        &scrape_data.high_level_domain,
    );

    // Creating an asynchronous `reqwest` Client to send HTTP requests with
    let client = match reqwest::Client::builder().user_agent(user_agent).build() {
        Ok(client) => client,
        Err(_) => return Err(webpage_url.to_string()),
    };

    // Sending an asynchronous get request, forcing the `Result` we get up the tree to the worker
    let res = match client.get(webpage_url).send().await {
        Ok(res) => res,
        Err(_) => return Err(webpage_url.to_string()),
    };

    let res = match res.text().await {
        Ok(res) => res,
        Err(_) => return Err(webpage_url.to_string()),
    };

    // Converting the HTML page we get into the parser's internal structure. This is actually
    // where a better part of our compute power is spent
    let res = Document::from(&res[..]);

    // Scrapping all text from the page
    let full_text = get_full_text(&res);

    // Finding all links on the page
    let all_links = get_links_from_html(&res, &webpage_url, &high_level_domain);

    // Searching for structured data on the page.
    // We are looking for <script type="application/ld+json"> and we need all of its contents
    // TODO: probably move this out into a separate function and parse this into pure text
    let structured_data: String = res
        .find(Name("script"))
        .filter(|n| n.attr("type") == Some("application/ld+json"))
        .map(|x| x.text())
        .nth(0)
        .unwrap_or("The page didn't have structured data".to_string());
    let webpage = webpage_url.to_string();

    Ok(ScrapeData {
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

    // Delete the '#' fragment and '?' queries from the url string
    joined.set_fragment(None);
    joined.set_query(None);

    if joined.has_host() && joined.host_str().unwrap() == high_level_domain {
        Some(joined.to_string())
    } else {
        None
    }
}
