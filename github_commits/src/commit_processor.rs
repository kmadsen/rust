use crate::commit_reader::RepositoryCommits;
use crate::results_writer::CommitDataRow;
use std::collections::HashMap;

pub fn map_to_data_rows(commits: Vec<RepositoryCommits>) -> Vec<CommitDataRow> {
  let mut results = Vec::new();
  commits.iter().for_each(|repo_commit| {
    repo_commit.commits.iter().for_each(|commit| {
      let data_row = CommitDataRow {
        repository: format!("{}/{}", repo_commit.organization_name, repo_commit.repository_name),
        sha: commit.sha.clone(),
        date: commit.date.clone(),
        message: commit.message.clone(),
        pr_url: "".to_string(),
      };
      results.push(data_row);
    });
  });
  results
}

pub fn merge_commit_data(existing_data: Vec<CommitDataRow>, new_data: Vec<CommitDataRow>) -> Vec<CommitDataRow> {
  let mut merged_data = Vec::new();
  let mut sha_map: HashMap<String, usize> = HashMap::new();

  for data_row in existing_data.into_iter() {
    let index = merged_data.len();
    sha_map.insert(data_row.sha.clone(), index);
    merged_data.push(data_row);
  }

  for new_row in new_data.into_iter() {
    let sha_key = new_row.sha.clone();
    if let Some(&index) = sha_map.get(&sha_key) {
      if merged_data[index].pr_url.is_empty() && !new_row.pr_url.is_empty() {
        merged_data[index] = new_row;
      }
    } else {
      sha_map.insert(sha_key, merged_data.len());
      merged_data.push(new_row);
    }
  }

  println!("Merged data size: {}", merged_data.len());
  merged_data
}
