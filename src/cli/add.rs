use std::{fs, path::PathBuf};

use crate::{create_dirs, error, success, sxcu::SXCU, Res};

pub fn add(path: PathBuf) -> Res<()> {
  let user = create_dirs()?;

  if !path.exists() {
    error!("uploader provided doesn't exist: {}", path.display());
  }

  if let Err(e) = SXCU::from_file(&path) {
    error!("the file was not a sxcu compatible file: {}", e);
  }

  let base = path.file_name().unwrap().to_str().unwrap();
  fs::copy(&path, format!("/home/{}/.fu/uploaders/{}", user, base))?;

  success!("added {}", base);

  Ok(())
}
