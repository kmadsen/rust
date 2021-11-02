use std::io::BufReader;
use std::path::PathBuf;
use std::io::Write;
use std::fs::File;
use std::io::prelude::*;
use anyhow::{Context, Result};

pub fn find_matches(
    path_buf: &PathBuf,
    pattern: &str,
    mut writer: impl Write
) -> std::result::Result<(), std::io::Error> {
    let content = File::open(&path_buf)?;
    let reader = BufReader::new(content);
    let read_line_iter = reader.lines()
        .filter_map(|result| result.ok());
      
    for line in read_line_iter {
        if line.contains(pattern) {
            writeln!(writer, "{}", line).unwrap();
        }
    }
    Ok(())
}

pub fn find_matches_anyhow(
    path_buf: &PathBuf,
    pattern: &str,
    mut writer: impl Write
) -> Result<()> {
    let content = File::open(&path_buf)
        .with_context(|| format!("could not read file {}", path_buf.display()))?;
    let reader = BufReader::new(content);
    let read_line_iter = reader.lines()
        .filter_map(|result| result.ok());
      
    for line in read_line_iter {
        if line.contains(pattern) {
            writeln!(writer, "{}", line).unwrap();
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::str;

  #[test]
  fn should_find_a_match() {
      let path_buf = PathBuf::from("resources/tests/find_matches.txt");
      let mut result = Vec::new();
      
      find_matches(&path_buf, "multiply", &mut result).unwrap();
      let result_str = str::from_utf8(&result).unwrap();
      let expected = "ludicrous multiply\n";
      assert_eq!(expected, result_str, "expected {} but was {}", expected, result_str);
  }

  #[test]
  fn should_error_when_file_is_unknown() {
      let path_buf = PathBuf::from("unknown/file.txt");
      let mut result = Vec::new();
      
      let result_err = find_matches(&path_buf, "multiply", &mut result);
      assert_eq!(&true, &result_err.is_err(), "Error expected");
  }

  #[test]
  #[should_panic(expected = "could not read file unknown/file.txt")]
  fn handle_errors_with_anyhow() {
      let path_buf = PathBuf::from("unknown/file.txt");
      let mut result = Vec::new();
      
      find_matches_anyhow(&path_buf, "multiply", &mut result).unwrap();
  }
}
