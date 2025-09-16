#![feature(prelude_import)]
#![feature(custom_inner_attributes)]
#![::fake_main::cfg_(not(feature = "skip"))]

//! Macro example.

/// The real main fn.
fn main() {
    println!("Real fn main");
}
