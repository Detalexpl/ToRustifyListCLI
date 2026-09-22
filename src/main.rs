use std::env::args;
mod tdv;
struct ToDoValue {
    name: String,
    id: usize,
    sub: Option<Vec<Sub>>,
    done: bool,
}
struct Sub {
    name: String,
    done: bool,
    id: usize,
}
fn main() {
    println!("Hello, world!");
}
fn tes() {
    let mut args: Vec<String> = args().collect();
    args.remove(0);
    match args[0].as_str() {
        "add" => println!("tset"),
        _ => {}
    }
}
