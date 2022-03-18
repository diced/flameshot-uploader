use std::process::{Command, Stdio};

use crate::Res;

pub enum ScreenshotType {
  GUI,
  SCREEN,
  FULL,
}

pub fn screenshot(screenshot_type: ScreenshotType) -> Res<Option<Vec<u8>>> {
  match screenshot_type {
    ScreenshotType::GUI => screenshot_gui(),
    ScreenshotType::SCREEN => screenshot_screen(),
    ScreenshotType::FULL => screenshot_full(),
  }
}

pub fn screenshot_gui() -> Res<Option<Vec<u8>>> {
  let output = Command::new("flameshot")
    .arg("gui")
    .arg("-r")
    .stdout(Stdio::piped())
    .spawn()?
    .wait_with_output()?;

  if !output.status.success() {
    Ok(None)
  } else {
    Ok(Some(output.stdout))
  }
}

pub fn screenshot_screen() -> Res<Option<Vec<u8>>> {
  let output = Command::new("flameshot")
    .arg("screen")
    .arg("-r")
    .stdout(Stdio::piped())
    .spawn()?
    .wait_with_output()?;

  if !output.status.success() {
    Ok(None)
  } else {
    Ok(Some(output.stdout))
  }
}

pub fn screenshot_full() -> Res<Option<Vec<u8>>> {
  let output = Command::new("flameshot")
    .arg("full")
    .arg("-r")
    .stdout(Stdio::piped())
    .spawn()?
    .wait_with_output()?;

  if !output.status.success() {
    Ok(None)
  } else {
    Ok(Some(output.stdout))
  }
}
