use std::process::Command;
use std::str;
use regex::Regex;

fn main() {
  let commit_re: Regex = Regex::new(r"(commit) (\b[0-9a-f]{5,40}\b)").unwrap();
  let author_re: Regex = Regex::new(r"(Author:)(\s+)(.*)(\s+)<.*>").unwrap();
  let date_re: Regex = Regex::new(r"(Date:)(\s+)(.*)").unwrap();

  let arguments: Vec<String> = std::env::args().collect();
  let author = match arguments.get(1) {
    Some(author) => author,
    None => "kyle",
  };
  let min_date = match arguments.get(2) {
    Some(min_date) => min_date,
    None => "2022-01-01",
  };
  println!("running for {}", author);

  // lol really?
  let repository: String = std::env::current_dir().unwrap()
    .file_name().unwrap().to_str().unwrap().to_string();
  println!("repository {}", repository);
  
  let output = Command::new("git")
    .arg("log")
    .arg(format!("--after={}", min_date))
    .arg(format!("--author={}", author))
    .output()
    .expect("ls command failed to start");
  let stdout: &str = std::str::from_utf8(&output.stdout).unwrap();
  for s in stdout.lines() {
      if let Some(captures) = commit_re.captures(s) {
        println!(); // Start each commit on a new line
        let sha = captures.get(2).map(|m| m.as_str()).unwrap();
        print!("{}\t{}", repository, sha);
      } else if let Some(captures) = date_re.captures(s) {
        let date = captures.get(3).map(|m| m.as_str()).unwrap();
        print!("\t{}\t", date);
      } else if let Some(_) = author_re.captures(s) {
        // ignoring author lines 
      } else {
        print!("{}", s.trim());
      }
  }
  println!(); // End with a new line
}
