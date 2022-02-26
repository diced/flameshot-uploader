use serde::{Deserialize, Serialize};
use std::{
  path::PathBuf,
  process::{Command, Stdio},
};

use crate::Res;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ClipboardBackend {
  XCLIP,
  XSEL,
}

pub fn copy(backend: &ClipboardBackend, text: &str) -> Res<()> {
  match backend {
    ClipboardBackend::XCLIP => xclip_copy(text),
    ClipboardBackend::XSEL => xsel_copy(text),
  }
}

pub fn xclip_copy_image(path: PathBuf) -> Res<()> {
  Command::new("xclip")
    .arg("-selection")
    .arg("clipboard")
    .arg("-t")
    .arg("image/png")
    .arg("-i")
    .arg(path.to_str().unwrap())
    .spawn()?
    .wait_with_output()?;

  Ok(())
}

pub fn xclip_copy(text: &str) -> Res<()> {
  let echo = Command::new("echo")
    .arg("-n")
    .arg(text)
    .stdout(Stdio::piped())
    .spawn()?;

  Command::new("xclip")
    .stdin(echo.stdout.unwrap())
    .arg("-selection")
    .arg("clipboard")
    .spawn()?
    .wait_with_output()?;

  Ok(())
}

pub fn xsel_copy(text: &str) -> Res<()> {
  let echo = Command::new("echo")
    .arg("-n")
    .arg(text)
    .stdout(Stdio::piped())
    .spawn()?;

  Command::new("xsel")
    .stdin(echo.stdout.unwrap())
    .arg("-i")
    .arg("-b")
    .spawn()?
    .wait_with_output()?;

  Ok(())
}
