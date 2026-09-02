use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Null,
    boolean(bool),
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
        let val = 6;
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
    fn parse_value(&mut self) ->Result<JsonValue,String>{
        match self.chars.peek(){
            Some("{") => self.parse_objet(),
        }
    }
    fn parse_obect
}
