//! # Hello crate
//!
//! `hello_crate` is a testing rust crates!
//! This comment is testing comment documentation
//! Check it out with cargo doc --open

mod rectangle;

// This api is public and exposes rectangle like this
//   use hello_crate::Rectangle;
pub use self::rectangle::Rectangle;
