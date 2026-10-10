mod tdv;
use crate::tdv::Id;
use std::io::{self, Write};
use std::{env, path::PathBuf, process};
use tdv::ToDoValue;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, StandardStreamLock, WriteColor};
use trl_json::JsonValue;
fn main() {
    let stdout = StandardStream::stdout(ColorChoice::Auto);
    let Some(mut path) = get_cache_dir() else {
        eprintln!("No cashe dir");
        process::exit(1)
    };
    path.push("trl_cli");
    let json = match JsonValue::create_from_file(&path) {
        Ok(json) => json,
        _ => JsonValue::new(),
    };
    let mut todoval = match ToDoValue::from_json(json) {
        Ok(val) => val,
        Err(e) => {
            dbg!(e);
            process::exit(1)
        }
    };
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(String::as_str) {
        // TODO: get read of this unwrap
        Some("list") => list(&todoval, stdout.lock()).unwrap(),
        Some(a) => {
            eprintln!("wrong argument: \"{}\"", a)
        }
        None => {
            eprintln!("no aruments provided use -h");
            process::exit(1);
        }
    }
}

#[cfg(target_os = "linux")]
fn get_cache_dir() -> Option<PathBuf> {
    absolute_env("XDG_CACHE_HOME").or_else(|| absolute_env("HOME").map(|home| home.join(".cache")))
}
#[cfg(target_os = "windows")]
fn get_cache_dir() -> Option<PathBuf> {
    absolute_env("LOCALAPPDATA").or_else(|| {
        absolute_env("USERPROFILE").map(|profile| profile.join("AppData").join("Local"))
    })
}
#[cfg(target_os = "macos")]
fn get_cache_dir() -> Option<PathBuf> {
    absolute_env("HOME").map(|home| home.join("Library").join("Caches"))
}

fn absolute_env(name: &str) -> Option<PathBuf> {
    let path = PathBuf::from(env::var_os(name)?);
    path.is_absolute().then_some(path)
}

// this function displeays all thinks to do
fn list(todo: &Vec<ToDoValue>, mut lock: StandardStreamLock) -> io::Result<()> {
    for item in todo {
        //displaing id of think to do in magenta color
        let id = item.get_id().to_string();
        lock.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)))?;
        write!(&mut lock, "{id} ")?;
        lock.reset()?;

        //displeaing name of think to do
        write!(&mut lock, "{} ", item.get_name())?;

        //displeaing complition of think to do
        if item.is_done() {
            lock.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
            writeln!(&mut lock, " [ done ]")?;
            lock.reset()?;
        } else {
            lock.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
            writeln!(&mut lock, " [undone]")?;
            lock.reset()?;
        }
        //displeaing sub thinks to do
        let sub = item.sub().unwrap_or_default();
        for s in sub {
            // displeaing sub id
            let sid = s.get_id().to_string();
            lock.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)))?;
            write!(&mut lock, "   {sid} ")?;
            lock.reset()?;

            // displaing sub name
            write!(&mut lock, "{} ", s.get_name())?;

            // displeaing complition of sub
            if s.is_done() {
                lock.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
                writeln!(&mut lock, " [ done ]")?;
                lock.reset()?;
            } else {
                lock.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
                writeln!(&mut lock, " [undne]")?;
                lock.reset()?;
            }
        }
    }
    Ok(())
}
// function adds new think to do
fn add(
    args: Vec<String>,
    todoval: &mut Vec<ToDoValue>,
    mut lock: StandardStreamLock,
) -> Result<(), String> {
    match args.get(2) {
        Some(n) => Ok(()),
        None => Err("no name provided".to_owned()),
    };
}
