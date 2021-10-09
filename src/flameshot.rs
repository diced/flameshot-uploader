use std::process::{Command, Stdio};

use crate::Res;

pub enum ScreenshotType {
  GUI,
  SCREEN,
  FULL,
}

pub fn screenshot(screenshot_type: ScreenshotType) -> Res<Vec<u8>> {
  match screenshot_type {
    ScreenshotType::GUI => screenshot_gui(),
    ScreenshotType::SCREEN => screenshot_screen(),
    ScreenshotType::FULL => screenshot_full(),
  }
}

pub fn screenshot_gui() -> Res<Vec<u8>> {
  let output = Command::new("flameshot")
    .arg("gui")
    .arg("-r")
    .stdout(Stdio::piped())
    .spawn()?
    .wait_with_output()?;

  Ok(output.stdout)
}

pub fn screenshot_screen() -> Res<Vec<u8>> {
  let output = Command::new("flameshot")
    .arg("screen")
    .arg("-r")
    .stdout(Stdio::piped())
    .spawn()?
    .wait_with_output()?;

  Ok(output.stdout)
}

pub fn screenshot_full() -> Res<Vec<u8>> {
  let output = Command::new("flameshot")
    .arg("full")
    .arg("-r")
    .stdout(Stdio::piped())
    .spawn()?
    .wait_with_output()?;

  Ok(output.stdout)
}
