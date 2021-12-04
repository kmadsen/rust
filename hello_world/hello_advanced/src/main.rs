mod functions_and_closures;
mod macros_example;
mod trait_associated_functions;
mod trait_newtype_pattern;
mod trait_operator_overloading;
mod trait_supertrait;
mod type_examples;
mod unsafe_examples;

fn main() {
  // https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html
  // unsafe_examples::run();

  // https://doc.rust-lang.org/book/ch19-03-advanced-traits.html
  // trait_operator_overloading::run();
  // trait_associated_functions::run();
  // trait_supertrait::run();
  // trait_newtype_pattern::run();

  // https://doc.rust-lang.org/book/ch19-04-advanced-types.html
  // type_examples::run();

  // https://doc.rust-lang.org/book/ch19-04-advanced-types.html
  // functions_and_closures::run();

  // https://doc.rust-lang.org/book/ch19-06-macros.html
  macros_example::run();
}
