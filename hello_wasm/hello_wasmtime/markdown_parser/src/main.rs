// import a CLI parsing library
extern crate structopt;

use structopt::StructOpt;
use std::path::PathBuf;

// Import a markdown parser library
extern crate pulldown_cmark;

use pulldown_cmark::{html, Parser};

use std::fs;

#[derive(StructOpt)]
#[structopt(
  name = "rust_wasi_markdown_parser",
  about = "Markdown to HTML renderer CLI"
)]
pub struct Options {
  #[structopt(parse(from_os_str))]
  filename: PathBuf,
}

fn main() {
  let options = Options::from_args();
  let contents = fs::read_to_string(options.filename).expect("Could not read file");
  let result = render_markdown(contents);
  println!("{}", result);
}

pub fn render_markdown(markdown: String) -> String {
    let mut html_buf = String::new();
    let parser = Parser::new(&markdown[..]);
    html::push_html(&mut html_buf, parser);
    html_buf
}