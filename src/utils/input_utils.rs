use colored::Colorize;
use std::{
    fmt,
    io::{stdin, stdout, Write},
    str::FromStr,
};

/// This function is used to get the user input
///
/// # Arguments
///
/// * `argument` - The argument to get the value from
pub fn get_one_argument<T>(argument: &str) -> T
where
    T: FromStr,
    T: fmt::Display,
    T: fmt::Debug,
    <T as FromStr>::Err: fmt::Debug,
    <T as FromStr>::Err: fmt::Display,
{
    loop {
        let mut buf = String::new();
        print!("{}", argument);
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
