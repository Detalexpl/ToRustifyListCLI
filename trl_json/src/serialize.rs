use crate::JsonValue;
use crate::JsonValue::*;
use std::collections::HashMap;
use std::iter::Peekable;
use std::string::String;
pub struct Serializer {
    json_value: JsonValue,
}
impl Serializer {
    pub fn new(input: JsonValue) -> Self {
        Serializer { json_value: input }
    }
    pub fn serialize(&mut self) -> Result<String, String> {
        let mut output = String::new();
        match self.json_value {
            Object(j) => output.push_str(&Self::serialize_object(j)?),
        }
        Ok(output)
    }
    fn serialize_object(self, object: HashMap<String, JsonValue>) -> Result<String, String> {
        let mut output = String::new();
        output.push('{');
        let map = object.iter().peekable();
        while let String(o) = map.next();
        Ok(output)
    }
    fn serialize_number(number: f64) -> String {
        let mut output = String::new();
        output.push_str(&number.to_string());
        output
    }
}
