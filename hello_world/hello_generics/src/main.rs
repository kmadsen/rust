
mod point;

fn main() {
    let integer = point::Point { x: 5, y: 10 };
    let float = point::Point { x: 5.2, y: 10.6 };

    println!("access {} {}", integer.x(), integer.y());
    println!("ijson {}", integer.json());
    println!("imag {}", integer.magnitude());
    println!("fmag {}", float.magnitude());
}
