fn different_closures() {
    fn add_one_v1 (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| { x + 1 };
    let add_one_v4 = |x| x + 1;
    println!("add_one_v1: {}", add_one_v1(1));
    println!("add_one_v2: {}", add_one_v2(1));
    println!("add_one_v3: {}", add_one_v3(1));
    println!("add_one_v4: {}", add_one_v4(1));
}

mod example_13_1;
mod example_13_2;

fn main() {
    different_closures();
    example_13_2::different_iterators();
    example_13_1::generate_workout(20, 3);
}
