// In order to make this work at the time of writing.
//  1. Install nightly
//  2. Fix linking issues
// 
// I'm considering this benchmarking strategy incomplete and
// not worth using. But leaving the code here in case I come
// back in the future.
// 
// https://doc.rust-lang.org/unstable-book/library-features/test.html

#![feature(test)]

extern crate test;
extern crate wasm_game_of_life;

#[bench]
fn universe_ticks(bencher: &mut test::Bencher) {
  let mut universe = wasm_game_of_life::Universe::new();

  bencher.iter(|| {
    universe.tick();
  });
}
