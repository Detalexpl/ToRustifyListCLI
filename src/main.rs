mod tdv;
use crate::tdv::Id;
use std::error::Error;
use std::io::Write;
use std::{env, path::PathBuf, process};
use tdv::ToDoValue;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, StandardStreamLock, WriteColor};
use trl_json::JsonValue;
fn main() {
    let stdout = StandardStream::stdout(ColorChoice::Auto);
    let Some(mut path) = get_data_dir() else {
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
        Some("--list") | Some("-l") => {
            list(&todoval, stdout.lock()).unwrap_or_else(|e| eprintln!("{e}"))
        }
        Some("--add") | Some("-a") => {
            add(args, &mut todoval, stdout.lock()).unwrap_or_else(|e| eprintln!("{e}"))
        }
        Some("--deleate") | Some("-d") => {
            del(args, &mut todoval, stdout.lock()).unwrap_or_else(|e| eprintln!("{e}"));
        }
        Some("--complete") | Some("-c") => {
            done(args, &mut todoval, stdout.lock()).unwrap_or_else(|e| eprintln!("{e}"))
        }
        Some("--incomplete") | Some("-i") => {
            undone(args, &mut todoval, stdout.lock()).unwrap_or_else(|e| eprintln!("{e}"))
        }
        Some(a) => {
            eprintln!("wrong argument: \"{}\"", a)
        }
        None => {
            eprintln!("no aruments provided use -h");
            process::exit(1);
        }
    }
    let json = ToDoValue::to_json(todoval);
    JsonValue::save_to_file(&path, &json).unwrap();
}

fn run() -> Result<(), Box<dyn Error>> {
    let path = get_data_dir().ok_or("no data dir")?.join("trl_cli");
    let json = if path.exists() {
        JsonValue::create_from_file(&path).map_err(|e| format!("cannot read file: {e:?}"))?
    } else {
        JsonValue::new() // pusta lista tylko wtedy, gdy pliku naprawdę nie ma
    };
    let mut todos = ToDoValue::from_json(json).map_err(|e| format!("invalid data: {e:?}"))?;

    let args: Vec<String> = env::args().skip(1).collect();
    let Some((cmd, rest)) = args.split_first() else {
        return Err("no arguments, use -h".into());
    };
    let mut changed = true;
    match cmd.as_str() {
        "--add" | "-a" => add(rest, &mut todos)?,
        "--delete" | "-d" => del(rest, &mut todos)?,
        "--complete" | "-c" => set_done(rest, &mut todos, true)?,
        "--incomplete" | "-i" => set_done(rest, &mut todos, false)?,
        "--list" | "-l" => changed = false,
        "--help" | "-h" => {
            //print_help();
            return Ok(());
        }
        other => return Err(format!("unknown argument: \"{other}\"").into()),
    }
    if changed {
        save(&path, &todos)?; // create_dir_all + plik tymczasowy + rename
    }
    let stdout = StandardStream::stdout(color_choice());
    list(&todos, stdout.lock())?;
    Ok(())
}
fn save(path: &PathBuf, todos: &Vec<ToDoValue>) {}

#[cfg(target_os = "linux")]
fn get_data_dir() -> Option<PathBuf> {
    absolute_env("XDG_CACHE_HOME").or_else(|| absolute_env("HOME").map(|home| home.join(".cache")))
}
#[cfg(target_os = "windows")]
fn get_datadir() -> Option<PathBuf> {
    absolute_env("LOCALAPPDATA").or_else(|| {
        absolute_env("USERPROFILE").map(|profile| profile.join("AppData").join("Local"))
    })
}
#[cfg(target_os = "macos")]
fn get_data_dir() -> Option<PathBuf> {
    absolute_env("HOME").map(|home| home.join("Library").join("Caches"))
}

fn absolute_env(name: &str) -> Option<PathBuf> {
    let path = PathBuf::from(env::var_os(name)?);
    path.is_absolute().then_some(path)
}

// this function displeays all thinks to do
fn list(todo: &Vec<ToDoValue>, mut lock: StandardStreamLock) -> Result<(), String> {
    for item in todo {
        //displaing id of think to do in magenta color
        let id = item.get_id().to_string();
        lock.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)))
            .map_err(|e| e.to_string())?;
        write!(&mut lock, "{id} ").map_err(|e| e.to_string())?;
        lock.reset().map_err(|e| e.to_string())?;

        //displeaing name of think to do
        write!(&mut lock, "{} ", item.get_name()).map_err(|e| e.to_string())?;

        //displeaing complition of think to do
        if item.is_done() {
            lock.set_color(ColorSpec::new().set_fg(Some(Color::Green)))
                .map_err(|e| e.to_string())?;
            writeln!(&mut lock, " [ done ]").map_err(|e| e.to_string())?;
            lock.reset().map_err(|e| e.to_string())?;
        } else {
            lock.set_color(ColorSpec::new().set_fg(Some(Color::Red)))
                .map_err(|e| e.to_string())?;
            writeln!(&mut lock, " [undone]").map_err(|e| e.to_string())?;
            lock.reset().map_err(|e| e.to_string())?;
        }
        //displeaing sub thinks to do
        let sub = item.sub().unwrap_or_default();
        for s in sub {
            // displeaing sub id
            let sid = s.get_id().to_string();
            lock.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)))
                .map_err(|e| e.to_string())?;
            write!(&mut lock, "   {sid} ").map_err(|e| e.to_string())?;
            lock.reset().map_err(|e| e.to_string())?;

            // displaing sub name
            write!(&mut lock, "{} ", s.get_name()).map_err(|e| e.to_string())?;

            // displeaing complition of sub
            if s.is_done() {
                lock.set_color(ColorSpec::new().set_fg(Some(Color::Green)))
                    .map_err(|e| e.to_string())?;
                writeln!(&mut lock, " [ done ]").map_err(|e| e.to_string())?;
                lock.reset().map_err(|e| e.to_string())?;
            } else {
                lock.set_color(ColorSpec::new().set_fg(Some(Color::Red)))
                    .map_err(|e| e.to_string())?;
                writeln!(&mut lock, " [undne]").map_err(|e| e.to_string())?;
                lock.reset().map_err(|e| e.to_string())?;
            }
        }
        lock.reset().map_err(|e| e.to_string())?;
    }
    Ok(())
}
// function adds new think to do
fn add(
    args: Vec<String>,
    todoval: &mut Vec<ToDoValue>,
    lock: StandardStreamLock,
) -> Result<(), String> {
    if args.len() == 4 {
        let arg_2: &str = &args[2];
        match arg_2.parse::<usize>() {
            Ok(key) => {
                if let Some(val) = todoval.iter_mut().find(|v| v.get_id() == key) {
                    val.add_sub(args[3].clone());
                    match val.sub() {
                        Some(mut v) => {
                            v.sort_unstable_by_key(|v| v.get_id());
                        }
                        None => {}
                    }
                }
            }
            Err(_) => {
                if let Some(val) = todoval.iter_mut().find(|v| v.get_name() == arg_2) {
                    val.add_sub(args[3].clone());
                    match val.sub() {
                        Some(mut v) => {
                            v.sort_unstable_by_key(|v| v.get_id());
                        }
                        None => {}
                    }
                } else {
                    return Err(format!("{arg_2} in wrong name"));
                }
            }
        }
        if let Some(val) = todoval.iter_mut().find(|v| v.get_name() == arg_2) {
            val.add_sub(args[3].clone());
            match val.sub() {
                Some(mut v) => {
                    v.sort_unstable_by_key(|v| v.get_id());
                }
                None => {}
            }
        }
    }
    if args.len() == 3 {
        let arg_2 = args[2].clone();
        ToDoValue::add(todoval, arg_2);
    }
    list(todoval, lock)?;
    Ok(())
}
// function to remove think to do
fn del(
    args: Vec<String>,
    todoval: &mut Vec<ToDoValue>,
    lock: StandardStreamLock,
) -> Result<(), String> {
    if args.len() == 4 {
        let arg_2: &str = &args[2];
        let key = arg_2
            .parse::<usize>()
            .map_err(|_| format!("{arg_2} is not valid number"))?;
        if let Some(val) = todoval.iter_mut().find(|v| v.get_id() == key) {
            let arg_3: &str = &args[3];
            let key_2 = arg_3
                .parse::<usize>()
                .map_err(|_| format!("{arg_3} is not valid number"))?;
            val.del_sub(key_2)?;
        }
    } else if args.len() == 3 {
        let arg_2: &str = &args[2];
        let key = arg_2
            .parse::<usize>()
            .map_err(|_| format!("\"{arg_2}\" is not valid number"))?;
        if let Some(_) = todoval.iter_mut().find(|v| v.get_id() == key) {
            todoval.retain(|val| val.get_id() != key);
        } else {
            return Err(format!(" no point with id: {arg_2} "));
        }
    } else {
        return Err("wrong number of aruments".to_owned());
    }
    list(todoval, lock)?;
    Ok(())
}
// function to cheange status to done
fn done(
    args: Vec<String>,
    todoval: &mut Vec<ToDoValue>,
    lock: StandardStreamLock,
) -> Result<(), String> {
    if args.len() == 4 {
        let arg_2: &str = &args[2];
        let key = arg_2
            .parse::<usize>()
            .map_err(|_| format!("{arg_2} is not valid nuber"))?;
        if let Some(val) = todoval.iter_mut().find(|v| v.get_id() == key) {
            let arg_3: &str = &args[3];
            let key = arg_3
                .parse::<usize>()
                .map_err(|_| format!("{arg_3} is not valid number"))?;
            val.done_sub(key)?;
        }
    } else if args.len() == 3 {
        let arg_2: &str = &args[2];
        let key = arg_2
            .parse()
            .map_err(|_| format!("{arg_2} is not valid number"))?;
        if let Some(val) = todoval.iter_mut().find(|v| v.get_id() == key) {
            val.to_done();
        } else {
            return Err(format!("no point with id: {arg_2}"));
        }
    } else {
        return Err("wrong number of arguments".to_owned());
    }
    list(todoval, lock)?;
    Ok(())
}
fn undone(
    args: Vec<String>,
    todoval: &mut Vec<ToDoValue>,
    lock: StandardStreamLock,
) -> Result<(), String> {
    if args.len() == 4 {
        let arg_2: &str = &args[2];
        let key = arg_2
            .parse::<usize>()
            .map_err(|_| format!("{arg_2} is not valid nuber"))?;
        if let Some(val) = todoval.iter_mut().find(|v| v.get_id() == key) {
            let arg_3: &str = &args[3];
            let key = arg_3
                .parse::<usize>()
                .map_err(|_| format!("{arg_3} is not valid number"))?;
            val.undone_sub(key)?;
        }
    } else if args.len() == 3 {
        let arg_2: &str = &args[2];
        let key = arg_2
            .parse()
            .map_err(|_| format!("{arg_2} is not valid number"))?;
        if let Some(val) = todoval.iter_mut().find(|v| v.get_id() == key) {
            val.to_undone();
        } else {
            return Err(format!("no point with id: {arg_2}"));
        }
    } else {
        return Err("wrong number of arguments".to_owned());
    }
    list(todoval, lock)?;
    Ok(())
}
