mod help;
mod tdv;
use crate::tdv::Id;
use std::error::Error;
use std::io::Write;
use std::process::ExitCode;
use std::{env, path::PathBuf};
use tdv::ToDoValue;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, StandardStreamLock, WriteColor};
use trl_json::JsonValue;
fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
fn run() -> Result<(), Box<dyn Error>> {
    let stdout = StandardStream::stdout(ColorChoice::Auto);

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
            help::print_help(&mut stdout.lock())?;
        }
        other => return Err(format!("unknown argument: \"{other}\"").into()),
    }
    let stdout = StandardStream::stdout(ColorChoice::Auto);
    list(&todos, stdout.lock())?;
    if changed {
        save(&path, todos)?;
    }
    Ok(())
}
fn save(path: &PathBuf, todos: Vec<ToDoValue>) -> Result<(), Box<dyn Error>> {
    trl_json::JsonValue::save_to_file(path, &ToDoValue::to_json(todos))?;
    Ok(())
}

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
fn add(args: &[String], todos: &mut Vec<ToDoValue>) -> Result<(), String> {
    match args {
        [name] => ToDoValue::add(todos, name.clone()),
        [parent, name] => {
            let found = match parent.parse::<usize>() {
                Ok(id) => todos.iter_mut().find(|t| t.get_id() == id),
                Err(_) => todos.iter_mut().find(|t| t.get_name() == parent.as_str()),
            };
            found
                .ok_or_else(|| format!("bo item: {parent}"))?
                .add_sub(name.clone());
        }
        _ => return Err("usage: --add [<id|name>] <text>".to_owned()),
    }
    Ok(())
}
fn del(args: &[String], todos: &mut Vec<ToDoValue>) -> Result<(), String> {
    let parse = |s: &str| {
        s.parse::<usize>()
            .map_err(|_| format!("\"{s}\" is not a valid number"))
    };
    match args {
        // usuń całe zadanie
        [id] => {
            let id = parse(id)?;
            let pos = todos
                .iter()
                .position(|t| t.get_id() == id)
                .ok_or_else(|| format!("no item with id: {id}"))?;
            todos.remove(pos);
        }
        // usuń podzadanie
        [id, sub] => {
            let (id, sub) = (parse(id)?, parse(sub)?);
            todos
                .iter_mut()
                .find(|t| t.get_id() == id)
                .ok_or_else(|| format!("no item with id: {id}"))?
                .del_sub(sub)?;
        }
        _ => return Err("wrong number of arguments".to_owned()),
    }
    Ok(())
}
fn set_done(args: &[String], todos: &mut [ToDoValue], done: bool) -> Result<(), String> {
    match args {
        [id] => {
            let item = find_mut(todos, parse_id(id)?)?;
            if done {
                item.to_done();
            } else {
                item.to_undone();
            }
        }
        [id, sub] => {
            let item = find_mut(todos, parse_id(id)?)?;
            let sub = parse_id(sub)?;
            if done {
                item.done_sub(sub)?;
            } else {
                item.undone_sub(sub)?;
            }
        }
        _ => return Err("wrong number of arguments".to_owned()),
    }
    Ok(())
}
fn find_mut(todos: &mut [ToDoValue], id: usize) -> Result<&mut ToDoValue, String> {
    todos
        .iter_mut()
        .find(|t| t.get_id() == id)
        .ok_or_else(|| format!("no item with id: {id}"))
}
fn parse_id(s: &str) -> Result<usize, String> {
    s.parse()
        .map_err(|_| format!("\"{s}\" is not a valid number"))
}
