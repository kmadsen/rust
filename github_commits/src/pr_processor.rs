use crate::results_writer::CommitDataRow;
use crate::pr_fetcher;
use futures::future::join_all;
use std::env;

pub async fn prs_for_commits(commits: &mut Vec<CommitDataRow>) {
  let futures: Vec<_> = commits.iter_mut().map(|commit| async {
    pr_for_commit(commit).await;
  }).collect();

  join_all(futures).await;
  println!("Fetched PRs for {} commits", commits.len());
}

async fn pr_for_commit(commit: &mut CommitDataRow) {
  if !commit.pr_url.is_empty() && !commit.pr_url.contains("commit") {
    return;
  }
  let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
  let result = pr_fetcher::fetch_pull_request(
    &commit.repository,
    &commit.sha,
    &github_token,
  ).await;
  let pull_request = match result {
    Ok(url) => url.value,
    Err(e) => {
      eprintln!("No pull request found for https://github.com/{}/commit/{} due to {}", commit.repository, commit.sha, e);
      "".to_string()
    }
  };
  commit.pr_url = pull_request;
}
