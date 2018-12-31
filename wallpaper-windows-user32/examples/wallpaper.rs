use std::env;
use std::error;
use std::path::Path;
use wallpaper_windows_user32::*;

static USAGE: &'static str = r#"Usage:
    wallpaper          - get wallpaper
    wallpaper [path]   - set wallpaper
"#;

fn main() -> Result<(), Box<error::Error>> {
    let args: Vec<_> = env::args().collect();
    match args.len() {
        2 => {
            let path: &Path = Path::new(&args[1]);
            set(path.as_os_str())?;
            Ok(())
        }
        1 => {
            let path = get()?;
            let path = path
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
