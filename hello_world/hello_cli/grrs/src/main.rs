use structopt::StructOpt;
use std::str::FromStr;

#[derive(StructOpt)]
struct Cli {
    pattern: String,

    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,

    #[structopt(default_value = "normal")]
    method: Method,
}

#[derive(StructOpt)]
enum Method {
    Normal,
    Anyhow
}

// any error type implementing Display is acceptable.
type ParseError = &'static str;

impl FromStr for Method {
    type Err = ParseError;
    fn from_str(day: &str) -> Result<Self, Self::Err> {
        match day {
            "normal" => Ok(Method::Normal),
            "anyhow" => Ok(Method::Anyhow),
            _ => Err("Could not parse a day"),
        }
    }
}

mod lib;

fn main() {
    let args = Cli::from_args();
    match &args.method {
        Method::Normal => lib::find_matches(
            &args.path,
            &args.pattern,
            &mut std::io::stdout()
        ).unwrap(),
        Method::Anyhow => lib::find_matches_anyhow(
            &args.path,
            &args.pattern,
            &mut std::io::stdout()
        ).unwrap()
    }
}
