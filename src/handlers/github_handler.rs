use crate::models::github_models::{CommonFilters, Tag};
use reqwest::blocking::Client;
use serde_json::from_str;

pub fn getReleases(repo: &str, filter: Option<CommonFilters>) -> Vec<Tag> {
    let client = Client::new();

    let per_page = match &filter {
        Some(f) => f.per_page,
        None => 30,
    };

    let page = match &filter {
        Some(f) => f.page,
        None => 1,
    };

    let url = format!("https://api.github.com/repos/{}/tags?per_page={}&page={}", repo, per_page, page);
    let response = client.get(&url)
        .header("User-Agent", "request")
        .send()
        .expect("Failed to send request");

    let body = response.text().expect("Failed to read response text");
    let tags: Vec<Tag> = from_str(&body).expect("Failed to deserialize response");

    tags
}