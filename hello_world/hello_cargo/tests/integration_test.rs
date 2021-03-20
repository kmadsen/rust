use hello_crate::Rectangle;

#[test]
fn has_access_to_rectangle() {
    let larger = Rectangle {
        width: 8,
        height: 7,
    };
    let smaller = Rectangle {
        width: 5,
        height: 1,
    };
    assert!(larger.can_hold(&smaller));
}
