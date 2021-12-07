use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use thread_pool::ThreadPool;

pub fn run() {
  let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
  let pool = ThreadPool::new(4);

  // Hack to only take 2 requests, this makes it so
  // we can see the logging for a server shutdown.
  //
  // Also note that I tried some implementations that
  // would allow for a shutdown signal, but didn't find
  // a good way to cancel the TcpListener.
  for stream in listener.incoming().take(2) {
    let stream = stream.unwrap();

    log::info!("Connecting..");
    pool.execute(|| {
      handle_connection(stream);
      log::info!("Connection established!");
    });
  }

  log::info!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
  let mut buffer = [0; 1024];
  stream.read(&mut buffer).unwrap();
  log::info!("Prepare response..");
  let get = b"GET / HTTP/1.1\r\n";
  let sleep = b"GET /sleep HTTP/1.1\r\n";

  let (status_line, filename) = if buffer.starts_with(get) {
    ("HTTP/1.1 200 OK", "response/hello.html")
  } else if buffer.starts_with(sleep) {
    thread::sleep(Duration::from_secs(5));
    ("HTTP/1.1 200 OK", "response/hello.html")
  } else {
    ("HTTP/1.1 404 NOT FOUND", "response/404.html")
  };
  let contents = fs::read_to_string(filename).unwrap();
  let response = format!(
    "{}\r\nContent-Length: {}\r\n\r\n{}",
    status_line,
    contents.len(),
    contents
  );

  stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap();
  log::info!("Responded!")
}
