use structopt::StructOpt;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

#[derive(StructOpt)]
struct Cli {
    pattern: String,

    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    let content = File::open(&args.path)
        .expect("could not read file");
    find_matches(&content, &args.pattern, &mut std::io::stdout())
}

fn find_matches(content: &File, pattern: &str, mut writer: impl std::io::Write) {
    let reader = BufReader::new(content);
    let read_line_iter = reader.lines()
        .filter_map(|result| result.ok());
        
    for line in read_line_iter {
        if line.contains(pattern) {
            writeln!(writer, "{}", line).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str;

    #[test]
    fn should_find_a_match() {
        let content = File::open("resources/tests/find_matches.txt")
            .expect("could not read file");
        let mut result = Vec::new();
        
        find_matches(&content, "multiply", &mut result);
        let result_str = str::from_utf8(&result).unwrap();
        let expected = "ludicrous multiply\n";
        assert_eq!(expected, result_str, "expected {} but was {}", expected, result_str);
    }
}
