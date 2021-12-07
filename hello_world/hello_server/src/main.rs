use env_logger::Builder;

mod multithreaded;
mod single_thread;

fn main() {
  // Enables logging with timestampes
  std::env::set_var("RUST_LOG", "trace");
  Builder::from_default_env().format_timestamp_micros().init();

  // single_thread::run();
  multithreaded::run();
}
