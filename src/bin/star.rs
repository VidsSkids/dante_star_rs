#![warn(clippy::all)]
#![warn(unused_crate_dependencies)]
#![warn(missing_docs)]

//! This binary will solve a perfect maze

use lib::init_log;

use colored as _;
use log as _;
use pretty_env_logger as _;

fn main() {
    init_log();
    println!("Hello, world!");
}
