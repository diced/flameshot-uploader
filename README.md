# Flameshot Uploader (fu)
Allowing you to supply a `.sxcu` file for uploading files with [Flameshot](https://flameshot.org)

# Usage
```
fu 1.0.0

USAGE:
    fu <SUBCOMMAND>

FLAGS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    add        Add an SXCU compatible uploader
    default    Set an uploader as the default one
    delete     Delete an uploader
    full       Screenshot the entire screen
    gui        Screenshot a specific region
    help       Print this message or the help of the given subcommand(s)
    screen     Screenshot the entire desktop
    show       Show all uploaders
```

# Todo
There are many features that SXCU supports, yet `fu` doesn't, like regexes, and non `multipart/form-data` bodies. I hope to add these later down the line.
- [ ] Support `$regex:*$`
- [x] Support `$json:*$`
- [ ] Support `$response$` - when the request doesn't have json (text/plain)
- [ ] Support other uploading options other than `multipart/form-data`

# Adding SXCU Files
You can either move the `.sxcu` file into `~/.fu/uploaders`
or run the command
```sh
fu /path/to/uploader.sxcu
```

# Removing Uploaders
The name will be the `<name>.sxcu` 
```sh
fu uploader
```

# Show all uploaders
```sh
fu show
```
The default uploader will be bolded to differentiate it from other uploaders

# Set a default uploader
```sh
fu default uploader
```
If there is no default uploader, it will just take a screenshot and copy the image to the clipboard

# Taking screenshots

## GUI
Use the Flameshot GUI to take a screenshot of a certain region. You can append the `-n`/`--no-upload` flag to not upload files using the default uploader.
```sh
fu gui
```

## Screen
Take a screenshot of the entire desktop
```sh
fu gui
```

## Full
Take a screenshot of everything
```sh
fu gui
```

# Configuration File
```toml
save_path = "/home/diced/Pictures"
default = "upload"
clipboard_backend = "XCLIP"
date_format = "%Y-%m-%d_%H:%M:%S.png"
notify = true
```

## `save_path`
The path images will be saved to. When running `fu` for the first time it will set it to `$HOME/Pictures`
## `default`
The default uploader to use.
## `clipboard_backend`
It can be either `XCLIP` or `XSEL`. This is based on your preference, yet `XCLIP` will copy images directly to the clipboard if no default uploader is set.
## `date_format`
The date format images should be saved as. For more reference visit [chrono::format::strfttime](https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html).
## `notify`
Whether or not to send a notification when any screenshot actions are performed.


# Requirements
This tool assumes that you have `flameshot` and `xsel`/`xclip`.