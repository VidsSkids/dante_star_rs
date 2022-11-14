#![warn(clippy::all)]
#![warn(unused_crate_dependencies)]
#![warn(missing_docs)]

//! This binary will generate a perfect maze

#[macro_use]
extern crate lib;
#[macro_use]
extern crate log;

use lib::{
    init_log,
    utils::{input_utils::get_one_argument, maze::Maze},
};
use std::env;

use colored::{self as _, Colorize};
use lazy_static as _;
use pretty_env_logger as _;
use rand as _;

fn main() {
    init_log();
    let args: Vec<String> = env::args().collect();
    let height = match args.get(1) {
        Some(arg) => match arg.parse() {
            Ok(height) => height,
            Err(_) => {
                error!(
                    "{} {}",
                    "Parsing error: ".bold().red(),
                    "height".bold().red()
                );
                return;
            }
        },
        None => get_one_argument::<i32>(user_input!("height").to_string().as_str()),
    };
    let width = match args.get(2) {
        Some(arg) => match arg.parse() {
            Ok(width) => width,
            Err(_) => {
                error!(
                    "{} {}",
                    "Parsing error: ".bold().red(),
                    "width".bold().red()
                );
                return;
            }
        },
        None => get_one_argument::<i32>(user_input!("width").to_string().as_str()),
    };
    let mut maze = Maze::new(height, width);
    let maze = maze.generate();
    println!("{maze}");
}
