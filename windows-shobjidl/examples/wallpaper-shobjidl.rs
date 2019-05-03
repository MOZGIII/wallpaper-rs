use std::env;
use std::error;
use std::ffi::OsStr;
use std::path::Path;
use wallpaper_windows_shobjidl::desktop_wallpaper;

static USAGE: &'static str = r#"Usage:
    wallpaper-shobjidl          - get wallpaper
    wallpaper-shobjidl [path]   - set wallpaper
"#;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<_> = env::args().collect();
    match args.len() {
        2 => {
            let path: &Path = Path::new(&args[1]);
            let mut dw = desktop_wallpaper().lock()?;
            dw.set_wallpaper::<&OsStr, _>(None, path)?;
            Ok(())
        }
        1 => {
            let mut dw = desktop_wallpaper().lock()?;
            let path = dw.get_wallpaper::<&OsStr>(None)?;
            let path = path
                .into_os_string()
                .into_string()
                .or(Err("Path is not a valid UTF-8 string"))?;
            println!("{}", path);
            Ok(())
        }
        _ => {
            eprintln!("{}", USAGE);
            Ok(())
        }
    }
}
