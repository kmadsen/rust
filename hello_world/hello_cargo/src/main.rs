use hello_crate::Rectangle;

fn main() {
 let tv_cga_screen = create_rectangle(320, 200);
 let tv_sif_screen = create_rectangle(384, 288);

 if tv_sif_screen.can_hold(&tv_cga_screen) {
   println!("qvga can hold sif");
 } else {
   println!("qvga cannot hold sif")
 }
}

/// A rectangle is like a TV.
///
/// # Examples
///
/// ```
/// let tv_cga_screen = create_rectangle(320, 200);
/// let tv_sif_screen = create_rectangle(384, 288);
///
/// if tv_sif_screen.can_hold(&tv_cga_screen) {
///   println!("qvga can hold sif");
/// } else {
///   println!("qvga cannot hold sif")
/// }
/// ```
fn create_rectangle(width: u32, height: u32) -> Rectangle {
    return Rectangle {
        width: width,
        height: height,
    };
}
