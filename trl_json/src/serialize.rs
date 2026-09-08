use crate::JsonValue;
use crate::JsonValue::*;
use std::collections::HashMap;
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
            Number(n) => output.push_str(&Self::serialize_number(&n)),
            String(ref s) => output.push_str(&Self::serialize_string(&s)),
            Array(ref v) => output.push_str(&Self::serialize_array(&v)),
            Object(ref o) => output.push_str(&Self::serialize_object(o.to_owned())),
            Null => output.push_str(&Self::serialize_null()),
            Boolean(b) => output.push_str(&Self::serialize_boolean(&b)),
        }
        Ok(output)
    }
    fn serialize_object(object: HashMap<String, JsonValue>) -> String {
        let mut output = String::new();
        output.push('{');
        let mut map = object.iter().peekable();
        while let Some(o) = &mut map.next() {
            output.push_str(&format!("\"{0}\" : ", o.0));
            match o.1 {
                Number(n) => output.push_str(&Self::serialize_number(n)),
                String(s) => output.push_str(&Self::serialize_string(s)),
                Array(v) => output.push_str(&Self::serialize_array(v)),
                Object(o) => output.push_str(&Self::serialize_object(o.to_owned())),
                Null => output.push_str(&Self::serialize_null()),
                Boolean(b) => output.push_str(&Self::serialize_boolean(b)),
            };
            if let Some(_) = map.peek() {
                output.push_str(", ")
            }
        }
        output.push('}');
        output
    }
    fn serialize_number(number: &f64) -> String {
        let mut output = String::new();
        output.push_str(&number.to_string());
        output
    }
    fn serialize_string(string: &String) -> String {
        format!("\"{}\"", string)
    }
    fn serialize_array(vec: &Vec<JsonValue>) -> String {
        let mut output = String::from("[");
        let mut array = vec.iter().peekable();
        while let Some(j) = array.next() {
            match j {
                Number(n) => output.push_str(&Self::serialize_number(n)),
                String(s) => output.push_str(&Self::serialize_string(s)),
                Array(a) => output.push_str(&Self::serialize_array(a)),
                Object(o) => output.push_str(&Self::serialize_object(o.to_owned())),
                Null => output.push_str(&Self::serialize_null()),
                Boolean(b) => output.push_str(&Self::serialize_boolean(b)),
            }
            if let Some(_) = array.peek() {
                output.push_str(", ");
            }
        }
        output.push(']');
        output
    }
    fn serialize_null() -> String {
        format!("null")
    }
    fn serialize_boolean(bool: &bool) -> String {
        if bool.to_owned() {
            "true".to_owned()
        } else {
            "false".to_owned()
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::*;
    #[test]
    fn serialize_boolean_test() {
        let json = JsonValue::Boolean(false);
        let serializer = Serializer::new(json);
        if let Boolean(b) = serializer.json_value {
            let anser = Serializer::serialize_boolean(&b);
            let correct = String::from("false");
            assert_eq!(anser, correct)
        } else {
            panic!("this shudl be inposible");
        }
        let json = JsonValue::Boolean(true);
        let serializer = Serializer::new(json);
        if let Boolean(b) = serializer.json_value {
            let anser = Serializer::serialize_boolean(&b);
            let correct = String::from("true");
            assert_eq!(anser, correct)
        } else {
            panic!("this shudl be inposible");
        }
    }

    #[test]
    fn serialize_null_test() {
        let anser = Serializer::serialize_null();
        assert_eq!(anser, "null".to_owned())
    }

    #[test]
    fn serialize_array_test() {
        let json = JsonValue::Array(vec![
            JsonValue::Null,
            JsonValue::Boolean(false),
            JsonValue::Number(1.0),
            JsonValue::String("test".to_owned()),
            JsonValue::Object(HashMap::new()),
        ]);
        let serializer = Serializer::new(json);
        if let Array(v) = serializer.json_value {
            let anser = Serializer::serialize_array(&v);
            let correct = "[null, false, 1, \"test\", {}]".to_owned();
            assert_eq!(anser, correct)
        } else {
            panic!("this is not posible")
        }
    }
    #[test]
    fn serializer_string_test() {
        let json = JsonValue::String("test".to_owned());
        let serializer = Serializer::new(json);
        if let String(s) = serializer.json_value {
            let anser = Serializer::serialize_string(&s);
            let correct = "\"test\"".to_owned();
            assert_eq!(anser, correct)
        } else {
            panic!("this is not posible")
        }
    }
}
