use std::path::Path;

use notify_rust::Notification;

use crate::{
  clipboard::{copy, xclip_copy_image, ClipboardBackend},
  config::Config,
  create_dirs, error,
  flameshot::{screenshot, ScreenshotType},
  sxcu::SXCU,
  Res,
};

pub fn ss(screenshot_type: ScreenshotType, no_upload: bool) -> Res<()> {
  let user = create_dirs()?;
  let config = Config::read()?;

  let bytes = screenshot(screenshot_type)?;
  let st = String::from_utf8_lossy(&bytes);
  if st == "screenshot aborted\n" {
    if config.notify {
      Notification::new()
        .summary("Flameshot")
        .body("Screenshot aborted")
        .icon("flameshot")
        .show()?;
    }
    error!("screenshort aborted");
  }

  if let Some(default) = config.default {
    if no_upload {
      let path = SXCU::save_image(bytes)?;

      if ClipboardBackend::XCLIP == config.clipboard_backend {
        xclip_copy_image(path)?;
        if config.notify {
          Notification::new()
            .summary("Flameshot")
            .body("Image copied to clipboard")
            .icon("flameshot")
            .show()?;
        }
      }
    } else {
      let sxcu_path = format!("/home/{}/.fu/uploaders/{}.sxcu", user, default);
      let path = Path::new(&sxcu_path);
      if !path.exists() {
        error!("uploader {}.sxcu didn't exist aborting...", default);
      }

      let sxcu = SXCU::from_file(path)?;
      let upload = sxcu.upload_screenshot(bytes);
      if let Err(e) = upload {
        error!("{}", e);
      }

      let out = upload?;

      copy(&config.clipboard_backend, &out.1)?;
      if config.notify {
        Notification::new()
          .summary("Image URL Copied")
          .body(&out.1)
          .icon("flameshot")
          .show()?;
      }
    }
  } else {
    let path = SXCU::save_image(bytes)?;

    if ClipboardBackend::XCLIP == config.clipboard_backend {
      xclip_copy_image(path)?;
      if config.notify {
        Notification::new()
          .summary("Flameshot")
          .body("Image copied to clipboard")
          .icon("flameshot")
          .show()?;
      }
    }
  }

  Ok(())
}
