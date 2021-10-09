use std::{fs, path::Path};

use crate::{create_dirs, error, success, Res};

pub fn delete(uploader: String) -> Res<()> {
  let user = create_dirs()?;

  let path = format!("/home/{}/.fu/uploaders/{}.sxcu", user, uploader);
  let path = Path::new(&path);

  if !path.exists() {
    error!("the uploader provided didn't exist: {}", uploader);
  }

  fs::remove_file(path)?;

  success!("removed uploader {}", uploader);

  Ok(())
}
