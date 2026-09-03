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
                _ => return Err("Expected ',' or ']' in array".to_string()),
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
            if c == '-' || c == '.' || c == 'e' || c == 'E' || c == '+' || c.is_ascii_digit() {
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
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn skip_withspaces_test() {
        let input = "   test";
        let mut anser = String::new();
        let mut mook_parser = Parser::new(input);
        mook_parser.skip_witespaces();
        while let Some(c) = mook_parser.chars.next() {
            anser.push(c);
        }
        assert_eq!(anser, "test".to_string())
    }
    #[test]
    fn parse_string_test() {
        let input = r#"" AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpRrSsTtWwUuZzXxYy 1234567890 ! # @ . /""#;
        let mut mook_parser = Parser::new(input);
        let anser = mook_parser.parse_string();
        assert_eq!(
            anser,
            Ok(JsonValue::String(String::from(
                " AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpRrSsTtWwUuZzXxYy 1234567890 ! # @ . /"
            )))
        );

        let input2 = r#"" Test2"#;
        let mut mook_parser2 = Parser::new(input2);
        let anser2 = mook_parser2.parse_string();
        assert_eq!(anser2, Err(r#"no closing quotation '"' mark"#.to_string()))
    }

    #[test]
    fn parse_literal_bool_test() {
        let input = "true";
        let mut mook_parser = Parser::new(input);
        let anser = mook_parser.parse_literal_bool();
        assert_eq!(anser, Ok(JsonValue::Boolean(true)));

        let input2 = "false";
        let mut mook_parser2 = Parser::new(input2);
        let anser2 = mook_parser2.parse_literal_bool();
        assert_eq!(anser2, Ok(JsonValue::Boolean(false)));

        let input3 = "test";
        let mut mook_parser3 = Parser::new(input3);
        let anser3 = mook_parser3.parse_literal_bool();
        assert_eq!(anser3, Err("unknown logic literal: test".to_string()))
    }

    #[test]
    fn parse_literal_null_test() {
        let input = "null";
        let mut mook_parser = Parser::new(input);
        let anser = mook_parser.parse_literal_null();
        assert_eq!(anser, Ok(JsonValue::Null));

        let input2 = "test";
        let mut mook_parser2 = Parser::new(input2);
        let anser = mook_parser2.parse_literal_null();
        assert_eq!(anser, Err("Expected null get: test".to_string()))
    }

    #[test]
    fn parse_arrey_test() {
        let input = r#"["test1","test2","test3" ]"#;
        let mut mook_parser = Parser::new(input);
        let anser = mook_parser.parse_arrey();
        let correct_vec = vec![
            JsonValue::String("test1".to_string()),
            JsonValue::String("test2".to_string()),
            JsonValue::String("test3".to_string()),
        ];
        let correct: Result<JsonValue, String> = Ok(JsonValue::Array(correct_vec));
        assert_eq!(anser, correct);

        let input2 = r#"[false,true,false]"#;
        let mut mook_parser2 = Parser::new(input2);
        let anser2 = mook_parser2.parse_arrey();
        let correct_vec2 = vec![
            JsonValue::Boolean(false),
            JsonValue::Boolean(true),
            JsonValue::Boolean(false),
        ];
        let correct2 = Ok(JsonValue::Array(correct_vec2));
        assert_eq!(anser2, correct2);

        let input2 = r#"[1,true,"test4"]"#;
        let mut mook_parser2 = Parser::new(input2);
        let anser2 = mook_parser2.parse_arrey();
        let correct_vec2 = vec![
            JsonValue::Number(1.0),
            JsonValue::Boolean(true),
            JsonValue::String("test4".to_string()),
        ];
        let correct2 = Ok(JsonValue::Array(correct_vec2));
        assert_eq!(anser2, correct2);

        let input3 = r#"[1,true,"test4""#;
        let mut mook_parser3 = Parser::new(input3);
        let anser3 = mook_parser3.parse_arrey();
        let correct3 = Err("Expected ',' or ']' in array".to_string());
        assert_eq!(anser3, correct3);

        let input4 = "[1,2 3]";
        let mut mook_parser4 = Parser::new(input4);
        let anser4 = mook_parser4.parse_arrey();
        let correct4: Result<JsonValue, String> = Err("Expected ',' or ']' in array".to_string());
        assert_eq!(anser4, correct4)
    }
    #[test]
    fn parse_number_test() {
        let input = "123";
        let mut mook_parser = Parser::new(input);
        let anser = mook_parser.parse_number();
        let correct: Result<JsonValue, String> = Ok(JsonValue::Number(123.0));
        assert_eq!(anser, correct);

        let input2 = "-123";
        let mut mook_parser2 = Parser::new(input2);
        let anser2 = mook_parser2.parse_number();
        let correct2: Result<JsonValue, String> = Ok(JsonValue::Number(-123.0));
        assert_eq!(anser2, correct2);

        let input3 = "1.0e+5";
        let mut mook_parser3 = Parser::new(input3);
        let anser3 = mook_parser3.parse_number();
        let correct3: Result<JsonValue, String> = Ok(JsonValue::Number(100000.0));
        assert_eq!(anser3, correct3);

        let input4 = "1.0ee5";
        let mut mook_parser4 = Parser::new(input4);
        let anser4 = mook_parser4.parse_number();
        let correct4: Result<JsonValue, String> = Err("couldn't get number".to_string());
        assert_eq!(anser4, correct4);
    }
    #[test]
    fn parse_object_test() {
        let input = r#"{
            "teststring" : "test",
            "testnumber" : 123,
            "testnull" : null,
            "testarrey" : [1, 2, 3],
            "testbool" : false,

        }"#;
        let mut mook_parser = Parser::new(input);
        let anser4 = mook_parser.parse_object();
        let mut correct_map: HashMap<String, JsonValue> = HashMap::new();
        correct_map.insert(
            "teststring".to_owned(),
            JsonValue::String("test".to_owned()),
        );
        correct_map.insert("testnumber".to_owned(), JsonValue::Number(123.0));
        correct_map.insert("testnull".to_owned(), JsonValue::Null);
        let correct_vec = vec![
            JsonValue::Number(1.0),
            JsonValue::Number(2.0),
            JsonValue::Number(3.0),
        ];
        correct_map.insert("testarrey".to_owned(), JsonValue::Array(correct_vec));
        correct_map.insert("testbool".to_owned(), JsonValue::Boolean(false));

        let correct: Result<JsonValue, String> = Ok(JsonValue::Object(correct_map));
    }
}
