use std::fs;
use std::path::PathBuf;
use std::process::Command;
use serde::Serialize;
use std::error::Error;

#[derive(Debug)]
pub struct Arguments {
  pub dir_path: String,
  pub author: String,
  pub min_date: String,
}

#[derive(Debug, Serialize)]
pub struct CommitInfo {
  pub sha: String,
  pub date: String,
  pub message: String,
}

#[derive(Debug, Serialize)]
pub struct RepositoryCommits {
  pub organization_name: String,
  pub repository_name: String,
  pub commits: Vec<CommitInfo>,
}

pub fn read_commits(args: Arguments) -> Result<Vec<RepositoryCommits>, Box<dyn Error>> {
  let mut repositories = Vec::new();
  for entry in fs::read_dir(&args.dir_path)? {
    let entry = entry?;
    let path = entry.path();
    if path.is_dir() {
      repositories.push(process_repository(path, &args.author, &args.min_date)?);
    }
  }

  Ok(repositories)
}

fn get_organization_name(repo_path: &PathBuf) -> Result<String, Box<dyn Error>> {
  let output = Command::new("git")
    .arg("-C")
    .arg(repo_path.to_str().ok_or("Path conversion error")?)
    .arg("config")
    .arg("--get")
    .arg("remote.origin.url")
    .output()?;

  let url = std::str::from_utf8(&output.stdout)?.trim();

  // Extract the organization from the SSH URL format
  //   git@github.com:organization/repository_name
  let parts: Vec<&str> = url.split(':').collect();
  if let Some(part) = parts.get(1) {
    let parts: Vec<&str> = part.split('/').collect();
    if let Some(organization) = parts.get(0) {
      return Ok(organization.to_string());
    }
  }

  Err("Failed to extract organization name from repository URL".into())
}

fn process_repository(repo_path: PathBuf, author: &str, min_date: &str) -> Result<RepositoryCommits, Box<dyn Error>> {
  let organization_name = get_organization_name(&repo_path)?;
  let repo_name = repo_path.file_name()
    .ok_or("Invalid repository path")
    ?.to_string_lossy()
    .into_owned();
  let output = Command::new("git")
    .arg("-C")
    .arg(repo_path.to_str().ok_or("Path conversion error")?)
    .arg("log")
    .arg("--pretty=format:%H%n%ad%n%BEND")
    .arg(format!("--after={}", min_date))
    .arg(format!("--author={}", author))
    .arg("--date=short")
    .output()?;

  let stdout = std::str::from_utf8(&output.stdout)?;

  let commits = stdout.split("END\n").filter_map(|part| {
    let lines: Vec<&str> = part.lines().collect();
    if lines.len() >= 3 {
      Some(CommitInfo {
        sha: lines[0].to_string(),
        date: lines[1].to_string(),
        message: lines[2..].join("\n"),
      })
    } else {
      None
    }
  }).collect();

  let commits = RepositoryCommits {
    organization_name: organization_name,
    repository_name: repo_name,
    commits,
  };
  Ok(commits)
}
