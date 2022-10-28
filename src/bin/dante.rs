#![warn(clippy::all)]
#![warn(unused_crate_dependencies)]
#![warn(missing_docs)]

//! This binary will generate a perfect maze

use lib::{
    init_log,
    utils::{input_utils::get_user_input, maze::Maze},
};

use colored as _;
use log as _;
use pretty_env_logger as _;

fn main() {
    init_log();
    let (height, width) = get_user_input();
    println!("{}", Maze::new(width, height));
}
