use std::{
    collections::HashMap,
    fs::{read_to_string, write},
    path::Path,
};
pub mod parser;
pub mod serialize;
#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}
impl JsonValue {
    pub fn create_from_file(path: &Path) -> Result<JsonValue, String> {
        let content = read_to_string(path).map_err(|e| e.to_string())?;

        let mut parser = parser::Parser::new(&content);
        parser.parse()
    }
    pub fn save_to_file(path: &Path, json: &JsonValue) -> Result<(), String> {
        let ser = serialize::Serializer::new(json);
        let out = ser.serialize()?;
        write(path, out).map_err(|e| e.to_string())?;

        Ok(())
    }
}
