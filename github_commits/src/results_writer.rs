use std::{error::Error, fs::{self, File}};

use csv::{Reader, Writer};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CommitDataRow {
  pub repository: String,
  pub sha: String,
  pub date: String,
  pub message: String,
  pub pr_url: String,
}

// constatnt for file name
const FILE_NAME: &str = "pull_requests.csv";

pub fn read_commits() -> Vec<CommitDataRow> {
  // Check if the file exists.
  if !fs::metadata(FILE_NAME).is_ok() {
    return Vec::new();
  }

  // Create a reader now that the file exists.
  let mut reader = match Reader::from_path(FILE_NAME) {
    Ok(reader) => reader,
    Err(_) => return Vec::new(),
  };

  let mut commits = Vec::new();
  for result in reader.records() {
    match result {
      Ok(record) => {
        commits.push(CommitDataRow {
          repository: record[0].to_string(),
          sha: record[1].to_string(),
          date: record[2].to_string(),
          message: record[3].to_string(),
          pr_url: record[4].to_string(),
        });
      },
      Err(e) => eprintln!("Error reading record: {}", e)
    }
  }
  commits
}

pub fn write_commits(commits: Vec<CommitDataRow>) -> Result<(), Box<dyn Error>> {
  let mut writer: Writer<File> = Writer::from_path(FILE_NAME)?;
  writer.write_record(&["Repository", "Commit SHA", "Date", "Message", "PR URL"])?;
  for commit in commits {
    writer.write_record(&[
      &commit.repository,
      &commit.sha,
      &commit.date,
      &commit.message,
      &commit.pr_url,
    ])?;
  }
  writer.flush()?;
  Ok(())
}
