use std::{
  fs,
  path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use toml::{from_str, to_string};

use crate::{clipboard::ClipboardBackend, create_dirs, Res};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
  pub save_path: PathBuf,
  pub default: Option<String>,
  pub clipboard_backend: ClipboardBackend,
  pub date_format: String,
  pub notify: bool,
}

impl Config {
  pub fn read() -> Res<Self> {
    let user = create_dirs()?;
    let path = format!("/home/{}/.fu/config.toml", user);
    let config_path = Path::new(&path);

    if !config_path.exists() {
      let config = Config {
        save_path: PathBuf::from(format!("/home/{}/Pictures", user)),
        default: None,
        clipboard_backend: ClipboardBackend::XCLIP,
        date_format: "%Y-%m-%d_%H:%M:%S.png".to_string(),
        notify: true
      };

      fs::write(config_path, to_string(&config)?)?;

      Ok(config)
    } else {
      let contents = fs::read_to_string(config_path)?;
      let config = from_str(&contents)?;

      Ok(config)
    }
  }
}
