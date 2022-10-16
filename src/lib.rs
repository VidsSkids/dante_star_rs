#![warn(clippy::all)]
#![warn(unused_crate_dependencies)]
#![warn(missing_docs)]

//! This crate is the lib used for the maze generator and solver

#[macro_use]
extern crate log;

use pretty_env_logger::env_logger;

/// This module contain all utils
pub mod utils;

#[macro_export]
/// This macro return the default user input string
macro_rules! user_input {
    ($x:expr) => {{
        use colored::Colorize;
        format!("{} Please input the {} of the maze : ", "➜".green(), $x)
            .bold()
            .cyan()
    }};
}

/// This function is used to initialize the logger
pub fn init_log() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    if std::env::var("RUST_BACKTRACE").is_err() {
        std::env::set_var("RUST_BACKTRACE", "full");
    }

    if cfg!(test) {
        env_logger::builder().is_test(true).try_init().unwrap();
    } else {
        pretty_env_logger::init()
    }
}
