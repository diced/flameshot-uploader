#[cfg(not(target_os = "linux"))]
compile_error!("can only be compiled on linux ;)");

use std::path::PathBuf;

use fu::{
  cli::{add, default, delete, show, ss},
  flameshot::ScreenshotType,
  Res,
};

use clap::{AppSettings, Clap};

#[derive(Clap, Debug)]
#[clap(version = "0.1.0", setting = AppSettings::ColoredHelp)]
struct Opts {
  #[clap(subcommand)]
  command: Command,
}

#[derive(Clap, Debug)]
enum Command {
  #[clap(about = "Screenshot a specific region")]
  Gui(ScreenshotOption),

  #[clap(about = "Screenshot the entire desktop")]
  Screen(ScreenshotOption),

  #[clap(about = "Screenshot the entire screen")]
  Full(ScreenshotOption),

  #[clap(about = "Add an SXCU compatible uploader")]
  Add(AddOption),

  #[clap(about = "Delete an uploader")]
  Delete(DeleteOption),

  #[clap(about = "Set an uploader as the default one")]
  Default(DefaultOption),

  #[clap(about = "Show all uploaders")]
  Show,
}

#[derive(Clap, Debug)]
struct ScreenshotOption {
  #[clap(short, long, about = "don't upload")]
  no_upload: bool
}

#[derive(Clap, Debug)]
struct AddOption {
  #[clap()]
  path: PathBuf,
}

#[derive(Clap, Debug)]
struct DeleteOption {
  #[clap()]
  uploader: String,
}

#[derive(Clap, Debug)]
struct DefaultOption {
  #[clap()]
  uploader: String,
}

fn main() -> Res<()> {
  let opts = Opts::parse();

  match opts.command {
    Command::Gui(opt) => ss(ScreenshotType::GUI, opt.no_upload),
    Command::Screen(opt) => ss(ScreenshotType::SCREEN, opt.no_upload),
    Command::Full(opt) => ss(ScreenshotType::FULL, opt.no_upload),
    Command::Add(opt) => add(opt.path),
    Command::Delete(opt) => delete(opt.uploader),
    Command::Default(opt) => default(opt.uploader),
    Command::Show => show(),
  }
}
