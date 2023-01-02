#!is-tested strict

use is_tested::is_tested;
use is_not_tested::is_not_tested;

#[is_tested("src/other.rs")]
#[is_not_tested]
fn main() {}

#[allow(dead_code)]
fn x() {}
