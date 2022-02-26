use std::{env, fs};

pub mod cli;
pub mod clipboard;
pub mod config;
pub mod flameshot;
pub mod sxcu;

pub type Res<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub fn create_dirs() -> Res<String> {
  let user = env::var("USER")?;
  fs::create_dir_all(format!("/home/{}/.fu/uploaders", user))?;

  Ok(user)
}

#[macro_export]
macro_rules! info {
  ($($arg:tt)*) => ({
    use colored::Colorize;
    println!("{}{}", "info: ".white().bold(), std::format_args!($($arg)*));
  })
}

#[macro_export]
macro_rules! success {
  ($($arg:tt)*) => ({
    use colored::Colorize;
    println!("\n\t{}{}\n", "success: ".green().bold(), std::format_args!($($arg)*));
  })
}

#[macro_export]
macro_rules! error {
  ($($arg:tt)*) => ({
    use colored::Colorize;
    println!("{}{}", "error: ".red().bold(), std::format_args!($($arg)*));
    std::process::exit(1);
  })
}
