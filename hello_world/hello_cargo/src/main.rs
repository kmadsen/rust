use hello_crate::Rectangle;

fn main() {
    let smaller = create_rectangle(10, 20);
    let larger = create_rectangle(11, 21);

    if smaller.can_hold(&larger) {
        println!("{:?} can hold {:?}", smaller, larger);
    } else {
        println!("{:?} cannot hold {:?}", smaller, larger);
    }
}

fn create_rectangle(width: u32, height: u32) -> Rectangle {
    return Rectangle {
        width: width,
        height: height
    }
}
