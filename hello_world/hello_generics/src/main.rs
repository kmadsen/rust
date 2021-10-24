
mod point;
mod find;

fn main() {
    let integer = point::Point { x: 5, y: 10 };
    let float = point::Point { x: 5.2, y: 10.6 };

    println!("access {} {}", integer.x(), integer.y());
    println!("ijson {}", integer.json());
    println!("imag {}", integer.magnitude());
    println!("fmag {}", float.magnitude());

    let char_list = vec!['y', 'm', 'a', 'q'];
    let char_result = find::largest(&char_list);
    let s: String = char_result.0.into_iter().collect();
    println!("largest {} is {}", s, char_result.1);

    let floats = vec![
        point::Point { x: 5.2, y: 10.6 },
        point::Point { x: 5.23, y: 10.6 },
        point::Point { x: 5.2, y: 10.61 },
        point::Point { x: 5.23, y: 10.6 }
    ];
    let floats_result = find::largest(&floats);
    print!("largest");
    for name in floats_result.0.iter() { print!(" {}", name); }
    println!(" is {}", floats_result.1)
}
