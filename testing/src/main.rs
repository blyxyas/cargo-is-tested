#! is tested
use is_tested_attribute::is_tested;

#[is_tested(path = "x")]
fn main() {
    println!("Hello, world!");
}
