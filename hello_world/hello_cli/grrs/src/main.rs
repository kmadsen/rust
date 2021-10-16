use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    pattern: String,

    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

mod lib;

fn main() {
    let args = Cli::from_args();
    lib::find_matches(
        &args.path,
        &args.pattern,
        &mut std::io::stdout()
    ).unwrap();
}
