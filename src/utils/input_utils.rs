use crate::user_input;
use colored::Colorize;
use std::{
    fmt,
    io::{stdin, stdout, Write},
    str::FromStr,
};

fn get_one_argument<T>(argument: &str) -> T
where
    T: FromStr,
    T: fmt::Display,
    T: fmt::Debug,
    <T as FromStr>::Err: fmt::Debug,
    <T as FromStr>::Err: fmt::Display,
{
    loop {
        let mut buf = String::new();
        print!("{}", user_input!(argument));
        stdout().flush().unwrap();
        let readline_result = stdin().read_line(&mut buf);
        if readline_result.is_err() {
            error!(
                "Cannot read the user input : {}",
                readline_result.unwrap_err()
            )
        }
        buf = buf.replace('\n', "");
        let argument_res = T::from_str(&buf);
        if let Ok(result) = argument_res {
            return result;
        } else {
            error!(
                "{} {}",
                "Parsing error: ".bold().red(),
                argument_res.unwrap_err().to_string().bold().red()
            );
        }
    }
}

/// This function is used to get the user input
///
/// # Return
///
/// First i32 is the height of the maze<br>
/// Second i32 is the width
pub fn get_user_input() -> (i32, i32) {
    let height = get_one_argument::<i32>("height");
    let width = get_one_argument::<i32>("width");
    if height < 1 || width < 1 {
        error!("{}", "Height and width must greater than 1".bold().red());
        return get_user_input();
    }
    (height, width)
}
