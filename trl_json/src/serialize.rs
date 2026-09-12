use crate::JsonValue;
use crate::JsonValue::*;
use std::collections::HashMap;
use std::string::String;
pub struct Serializer {
    json_value: JsonValue,
}
impl Serializer {
    pub fn new(input: &JsonValue) -> Self {
        Serializer {
            json_value: input.to_owned(),
        }
    }
    pub fn serialize(&self) -> Result<String, String> {
        let mut output = String::new();
        match self.json_value {
            Number(n) => output.push_str(&Self::serialize_number(&n)?),
            String(ref s) => output.push_str(&Self::serialize_string(&s)),
            Array(ref v) => output.push_str(&Self::serialize_array(&v)?),
            Object(ref o) => output.push_str(&Self::serialize_object(o)?),
            Null => output.push_str(&Self::serialize_null()),
            Boolean(b) => output.push_str(&Self::serialize_boolean(&b)),
        }
        Ok(output)
    }
    fn serialize_object(object: &HashMap<String, JsonValue>) -> Result<String, String> {
        let mut output = String::new();
        output.push('{');
        let mut map = object.iter().peekable();
        while let Some(o) = &mut map.next() {
            output.push_str(&format!("\"{0}\" : ", o.0));
            match o.1 {
                Number(n) => output.push_str(&Self::serialize_number(n)?),
                String(s) => output.push_str(&Self::serialize_string(s)),
                Array(v) => output.push_str(&Self::serialize_array(v)?),
                Object(o) => output.push_str(&Self::serialize_object(o)?),
                Null => output.push_str(&Self::serialize_null()),
                Boolean(b) => output.push_str(&Self::serialize_boolean(b)),
            };
            if let Some(_) = map.peek() {
                output.push_str(", ")
            }
        }
        output.push('}');
        Ok(output)
    }
    fn serialize_number(number: &f64) -> Result<String, String> {
        if number.is_nan() || number.is_infinite() {
            return Err(format!("{} is not valid JSON number", number));
        }

        Ok(number.to_string())
    }
    fn serialize_string(string: &String) -> String {
        let mut output = String::with_capacity(string.len() + 2);
        output.push('"');
        for c in string.chars() {
            match c {
                '"' => output.push_str("\\\""),
                '\\' => output.push_str("\\\\"),
                '\n' => output.push_str("\\n"),
                '\r' => output.push_str("\\r"),
                '\t' => output.push_str("\\t"),
                c if (c as u32) < 0x20 => output.push_str(&format!("\\u{:04x}", c as u32)),
                c => output.push(c),
            }
        }
        output.push('"');
        output
    }

    fn serialize_array(vec: &Vec<JsonValue>) -> Result<String, String> {
        let mut output = String::from("[");
        let mut array = vec.iter().peekable();
        while let Some(j) = array.next() {
            match j {
                Number(n) => output.push_str(&Self::serialize_number(n)?),
                String(s) => output.push_str(&Self::serialize_string(s)),
                Array(a) => output.push_str(&Self::serialize_array(a)?),
                Object(o) => output.push_str(&Self::serialize_object(o)?),
                Null => output.push_str(&Self::serialize_null()),
                Boolean(b) => output.push_str(&Self::serialize_boolean(b)),
            }
            if let Some(_) = array.peek() {
                output.push_str(", ");
            }
        }
        output.push(']');
        Ok(output)
    }
    fn serialize_null() -> String {
        "null".to_owned()
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
        let serializer = Serializer::new(&json);
        if let Boolean(b) = serializer.json_value {
            let anser = Serializer::serialize_boolean(&b);
            let correct = String::from("false");
            assert_eq!(anser, correct)
        } else {
            panic!("this shudl be inposible");
        }
        let json = JsonValue::Boolean(true);
        let serializer = Serializer::new(&json);
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
        let serializer = Serializer::new(&json);
        if let Array(v) = serializer.json_value {
            let anser = Serializer::serialize_array(&v).unwrap();
            let correct = "[null, false, 1, \"test\", {}]".to_owned();
            assert_eq!(anser, correct)
        } else {
            panic!("this is not posible")
        }
    }
    #[test]
    fn serializer_string_test() {
        let json = JsonValue::String("test".to_owned());
        let serializer = Serializer::new(&json);
        if let String(s) = serializer.json_value {
            let anser = Serializer::serialize_string(&s);
            let correct = "\"test\"".to_owned();
            assert_eq!(anser, correct)
        } else {
            panic!("this is not posible")
        }
    }
    #[test]
    fn serialize_number_integer_test() {
        let json = JsonValue::Number(1.0);
        let serializer = Serializer::new(&json);
        if let Number(n) = serializer.json_value {
            let answer = Serializer::serialize_number(&n).unwrap();
            let correct = String::from("1");
            assert_eq!(answer, correct)
        } else {
            panic!("this should be impossible");
        }
    }

    #[test]
    fn serialize_number_decimal_test() {
        let json = JsonValue::Number(3.14);
        let serializer = Serializer::new(&json);
        if let Number(n) = serializer.json_value {
            let answer = Serializer::serialize_number(&n).unwrap();
            let correct = String::from("3.14");
            assert_eq!(answer, correct)
        } else {
            panic!("this should be impossible");
        }
    }

    #[test]
    fn serialize_number_negative_test() {
        let json = JsonValue::Number(-7.0);
        let serializer = Serializer::new(&json);
        if let Number(n) = serializer.json_value {
            let answer = Serializer::serialize_number(&n).unwrap();
            let correct = String::from("-7");
            assert_eq!(answer, correct)
        } else {
            panic!("this should be impossible");
        }
    }

    // --- serialize_object ---

    #[test]
    fn serialize_object_empty_test() {
        let json = JsonValue::Object(HashMap::new());
        let serializer = Serializer::new(&json);
        if let Object(o) = serializer.json_value {
            let answer = Serializer::serialize_object(&o).unwrap();
            let correct = "{}".to_owned();
            assert_eq!(answer, correct)
        } else {
            panic!("this should be impossible");
        }
    }

    #[test]
    fn serialize_object_single_key_test() {
        let mut map = HashMap::new();
        map.insert("key".to_owned(), JsonValue::Number(1.0));
        let json = JsonValue::Object(map);
        let serializer = Serializer::new(&json);
        if let Object(o) = serializer.json_value {
            let answer = Serializer::serialize_object(&o).unwrap();
            let correct = "{\"key\" : 1}".to_owned();
            assert_eq!(answer, correct)
        } else {
            panic!("this should be impossible");
        }
    }

    #[test]
    fn serialize_object_multiple_keys_test() {
        // HashMap nie gwarantuje kolejności iteracji, więc zamiast porównywać
        // dokładny string, sprawdzamy strukturę i obecność obu par.
        let mut map = HashMap::new();
        map.insert("a".to_owned(), JsonValue::Number(1.0));
        map.insert("b".to_owned(), JsonValue::Boolean(true));
        let json = JsonValue::Object(map);
        let serializer = Serializer::new(&json);
        if let Object(o) = serializer.json_value {
            let answer = Serializer::serialize_object(&o).unwrap();
            assert!(answer.starts_with('{') && answer.ends_with('}'));
            assert!(answer.contains("\"a\" : 1"));
            assert!(answer.contains("\"b\" : true"));
            assert_eq!(answer.matches(", ").count(), 1);
        } else {
            panic!("this should be impossible");
        }
    }

    #[test]
    fn serialize_object_nested_test() {
        let mut map = HashMap::new();
        map.insert(
            "items".to_owned(),
            JsonValue::Array(vec![JsonValue::Number(1.0), JsonValue::Number(2.0)]),
        );
        let json = JsonValue::Object(map);
        let serializer = Serializer::new(&json);
        if let Object(o) = serializer.json_value {
            let answer = Serializer::serialize_object(&o).unwrap();
            let correct = "{\"items\" : [1, 2]}".to_owned();
            assert_eq!(answer, correct)
        } else {
            panic!("this should be impossible");
        }
    }

    // --- serialize() (główna metoda publiczna) ---

    #[test]
    fn serialize_method_number_test() {
        let json = JsonValue::Number(2.5);
        let serializer = Serializer::new(&json);
        let answer = serializer.serialize().unwrap();
        assert_eq!(answer, "2.5".to_owned());
    }

    #[test]
    fn serialize_method_string_test() {
        let json = JsonValue::String("hello".to_owned());
        let serializer = Serializer::new(&json);
        let answer = serializer.serialize().unwrap();
        assert_eq!(answer, "\"hello\"".to_owned());
    }

    #[test]
    fn serialize_method_boolean_test() {
        let json = JsonValue::Boolean(true);
        let serializer = Serializer::new(&json);
        let answer = serializer.serialize().unwrap();
        assert_eq!(answer, "true".to_owned());
    }

    #[test]
    fn serialize_method_null_test() {
        let json = JsonValue::Null;
        let serializer = Serializer::new(&json);
        let answer = serializer.serialize().unwrap();
        assert_eq!(answer, "null".to_owned());
    }

    #[test]
    fn serialize_method_array_test() {
        let json = JsonValue::Array(vec![JsonValue::Number(1.0), JsonValue::Null]);
        let serializer = Serializer::new(&json);
        let answer = serializer.serialize().unwrap();
        assert_eq!(answer, "[1, null]".to_owned());
    }

    #[test]
    fn serialize_method_object_test() {
        let mut map = HashMap::new();
        map.insert("k".to_owned(), JsonValue::Boolean(false));
        let json = JsonValue::Object(map);
        let serializer = Serializer::new(&json);
        let answer = serializer.serialize().unwrap();
        assert_eq!(answer, "{\"k\" : false}".to_owned());
    }
}
