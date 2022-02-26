use std::{fs, path::Path};

use toml::to_string;

use crate::{config::Config, create_dirs, error, success, Res};

pub fn default(uploader: String) -> Res<()> {
  let user = create_dirs()?;
  let mut config = Config::read()?;

  let path = format!("/home/{}/.fu/uploaders/{}.sxcu", user, uploader);
  let path = Path::new(&path);

  if !path.exists() {
    error!("the uploader provided didn't exist: {}", uploader);
  }

  config.default = Some(uploader);
  let st = to_string(&config)?;
  fs::write(format!("/home/{}/.fu/config.toml", user), st)?;

  success!("updated default uploader to {}", config.default.unwrap());

  Ok(())
}
