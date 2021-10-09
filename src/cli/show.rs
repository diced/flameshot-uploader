use std::fs;

use crate::{config::Config, create_dirs, Res};

use colored::Colorize;

pub fn show() -> Res<()> {
  let user = create_dirs()?;
  let config = Config::read()?;

  let path = format!("/home/{}/.fu/uploaders/", user);
  let dirs = fs::read_dir(path)?;

  for dir in dirs {
    let dirent = dir?;
    let path = dirent.path();
    let name = path.file_stem().unwrap().to_str().unwrap();
    let default = config.default.eq(&Some(name.to_string()));
    println!(
      "{}",
      if default {
        name.bold().bright_green()
      } else {
        name.green()
      }
    )
  }

  Ok(())
}
