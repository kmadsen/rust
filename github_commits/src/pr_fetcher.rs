use serde::{Serialize, Deserialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct HtmlUrl {
  pub value: String,
}

pub async fn fetch_pull_request(
  repository: &str,
  sha: &str,
  github_token: &str,
) -> Result<HtmlUrl, Box<dyn Error>> {
  let client = reqwest::Client::new();
  println!("Fetching PR for repository {} commit {}", repository, sha);
  let request_url = format!(
    "https://api.github.com/search/issues?q=repo:{}+{}+type:pr",
    repository, sha,
  );

  let response = client.get(&request_url)
    .header("Authorization", format!("token {}", github_token))
    .header("User-Agent", "request")
    .header("Accept", "application/vnd.github.v3+json")
    .send()
    .await?
    .json::<serde_json::Value>()
    .await?;

  // Simplified: Extract the first PR from the response, if available
  if let Some(item) = response["items"].as_array().and_then(|items| items.first()) {
    let url = serde_json::from_value::<HtmlUrl>(item.clone())
      .map_err(|e| Box::new(e) as Box<dyn Error>)?;
    Ok(url)
  } else {
    let url = HtmlUrl { value: format!("https://github.com/{}/commit/{}", repository, sha) };
    Ok(url)
    // let url = HtmlUrl { value: "".to_string() };
    // Ok(url)
  }
}
