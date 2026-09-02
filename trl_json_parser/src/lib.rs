use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

pub struct Parser<'a> {
    chars: Peekable<Chars<'a>>,
}
impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser {
            chars: input.chars().peekable(),
        }
    }
    pub fn parse(&mut self) -> Result<JsonValue, String> {
        self.skip_witespaces();
        let val = self.parse_value()?;
        self.skip_witespaces();

        if self.chars.peek().is_some() {
            return Err("aditional chars at end of JSON file".to_string());
        }
        Ok(val)
    }
    fn skip_witespaces(&mut self) {
        while let Some(&c) = self.chars.peek() {
            if c.is_whitespace() {
                self.chars.next();
            } else {
                break;
            }
        }
    }
    fn parse_value(&mut self) -> Result<JsonValue, String> {
        self.skip_witespaces();
        match self.chars.peek() {
            Some('{') => self.parse_object(),
            Some('[') => self.parse_arrey(),
            Some('"') => self.parse_string(),
            Some('t') | Some('f') => self.parse_literal_bool(),
            Some('n') => self.parse_literal_null(),
            Some(&c) if c == '-' || c.is_ascii_digit() => self.parse_number(),
            _ => return Err("couldnt parse JSON".to_string()),
        }
    }
    fn parse_object(&mut self) -> Result<JsonValue, String> {
        self.chars.next();
        let mut map = HashMap::new();

        if let Some('}') = self.chars.peek() {
            self.chars.next();
            return Ok(JsonValue::Object(map));
        }

        loop {
            self.skip_witespaces();
            let key = match self.parse_string()? {
                JsonValue::String(s) => s,
                _ => return Err("key must be string".to_string()),
            };

            self.skip_witespaces();
            if self.chars.next() != Some(':') {
                return Err(r#"Expected ':' after key"#.to_string());
            }

            let val = self.parse_value()?;
            map.insert(key, val);

            self.skip_witespaces();
            match self.chars.next() {
                Some('}') => break,
                Some(',') => continue,
                _ => return Err(r#"Expected ',' or '}' in object"#.to_string()),
            }
        }
        Ok(JsonValue::Object(map))
    }
    fn parse_string(&mut self) -> Result<JsonValue, String> {
        self.chars.next();
        let mut s = String::new();

        while let Some(c) = self.chars.next() {
            if c == '"' {
                self.chars.next();
                return Ok(JsonValue::String(s));
            }
            s.push(c);
        }
        Err(r#"no closing quotation '"' mark"#.to_string())
    }
    fn parse_arrey(&mut self) -> Result<JsonValue, String> {
        self.chars.next();
        let mut arr = Vec::new();
        self.skip_witespaces();

        if let Some(']') = self.chars.peek() {
            self.chars.next();
            return Ok(JsonValue::Array(arr));
        }

        loop {
            arr.push(self.parse_value()?);
            self.skip_witespaces();
            match self.chars.next() {
                Some(']') => break,
                Some(',') => continue,
                _ => return Err(r#"Expected ',' or ']' in array"#.to_string()),
            }
        }
        return Ok(JsonValue::Array(arr));
    }
    fn parse_literal_bool(&mut self) -> Result<JsonValue, String> {
        let mut bool = String::new();
        while let Some(c) = self.chars.peek() {
            if c.is_ascii_lowercase() {
                bool.push(self.chars.next().unwrap())
            } else {
                break;
            }
        }
        match bool.as_str() {
            "true" => Ok(JsonValue::Boolean(true)),
            "false" => Ok(JsonValue::Boolean(false)),
            _ => Err(format!("unknown logic literal: {}", bool)),
        }
    }
    fn parse_literal_null(&mut self) -> Result<JsonValue, String> {
        let mut s = String::new();
        while let Some(c) = self.chars.peek() {
            if c.is_ascii_lowercase() {
                s.push(self.chars.next().unwrap());
            } else {
                break;
            }
        }
        if s == "null" {
            return Ok(JsonValue::Null);
        } else {
            return Err(format!("Expected null get: {}", s));
        }
    }
    fn parse_number(&mut self) -> Result<JsonValue, String> {
        let mut num_str = String::new();
        while let Some(&c) = self.chars.peek() {
            if c == '-' || c == '.' || c == 'e' || c == 'E' || c.is_ascii_digit() {
                num_str.push(self.chars.next().unwrap());
            } else {
                break;
            }
        }
        num_str
            .parse::<f64>()
            .map(JsonValue::Number)
            .map_err(|_| "couldn't get number".to_string())
    }
}
