use std::env;

mod commit_reader;
mod pr_fetcher;
mod pr_processor;
mod results_writer;
mod commit_processor;

use commit_reader::Arguments;
use pr_processor::prs_for_commits;
use commit_processor::map_to_data_rows;
use commit_processor::merge_commit_data;

#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();

  let args: Vec<String> = env::args().collect();
  if args.len() < 4 {
    eprintln!("Usage: {} <directory_path> <author> <min_date>", args[0]);
    std::process::exit(1);
  }

  let arguments = Arguments {
    dir_path: args[1].clone(),
    author: args[2].clone(),
    min_date: args[3].clone(),
  };

  let existing_data = results_writer::read_commits();
  let existing_data_len = existing_data.len();

  println!("Existing commit data: {}", existing_data_len);
  match commit_reader::read_commits(arguments) {
    Ok(commits) => {
      let new_data = map_to_data_rows(commits);
      let mut merged_data = merge_commit_data(existing_data, new_data);
      prs_for_commits(&mut merged_data).await;
      results_writer::write_commits(merged_data).unwrap();
    },
    Err(e) => {
      eprintln!("Error reading commits: {}", e);
    }
  }
}
