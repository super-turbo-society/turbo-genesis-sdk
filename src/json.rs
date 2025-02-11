use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    String(String),
    Number(f64),
    Object(BTreeMap<String, JsonValue>),
    Array(Vec<JsonValue>),
    Boolean(bool),
    Null,
}
impl JsonValue {
    pub fn as_str(&self) -> Result<&str, &'static str> {
        if let JsonValue::String(s) = self {
            Ok(s)
        } else {
            Err("Not a string")
        }
    }

    pub fn as_string(&self) -> Result<String, &'static str> {
        if let JsonValue::String(s) = self {
            Ok(s.clone())
        } else {
            Err("Not a string")
        }
    }

    pub fn as_f64(&self) -> Result<f64, &'static str> {
        if let JsonValue::Number(n) = self {
            Ok(*n)
        } else {
            Err("Not a number")
        }
    }

    pub fn as_u32(&self) -> Result<u32, &'static str> {
        if let JsonValue::Number(n) = self {
            Ok(*n as u32)
        } else {
            Err("Not a number")
        }
    }

    pub fn as_object(&self) -> Result<&BTreeMap<String, JsonValue>, &'static str> {
        if let JsonValue::Object(o) = self {
            Ok(o)
        } else {
            Err("Not an object")
        }
    }

    pub fn as_array(&self) -> Result<&Vec<JsonValue>, &'static str> {
        if let JsonValue::Array(a) = self {
            Ok(a)
        } else {
            Err("Not an array")
        }
    }

    pub fn as_bool(&self) -> Result<bool, &'static str> {
        if let JsonValue::Boolean(b) = self {
            Ok(*b)
        } else {
            Err("Not a boolean")
        }
    }

    pub fn as_null(&self) -> Result<(), &'static str> {
        if let JsonValue::Null = self {
            Ok(())
        } else {
            Err("Not null")
        }
    }

    // Convert JSON array to Vec<T>
    pub fn as_vec<T, F>(&self, convert: F) -> Result<Vec<T>, &'static str>
    where
        F: Fn(&JsonValue) -> Result<T, &'static str>,
    {
        if let JsonValue::Array(array) = self {
            let mut result = Vec::new();
            for item in array {
                result.push(convert(item)?);
            }
            Ok(result)
        } else {
            Err("Not an array")
        }
    }

    // Convert JSON value to Option<T>
    pub fn as_option<T, F>(&self, convert: F) -> Result<Option<T>, &'static str>
    where
        F: Fn(&JsonValue) -> Result<T, &'static str>,
    {
        if let JsonValue::Null = self {
            Ok(None)
        } else {
            Ok(Some(convert(self)?))
        }
    }
}

fn parse_string(chars: &mut std::iter::Peekable<std::str::Chars>) -> String {
    chars.next(); // consume the opening quote
    let mut result = String::new();
    while let Some(&c) = chars.peek() {
        match c {
            '"' => {
                chars.next(); // consume the closing quote
                break;
            }
            _ => result.push(chars.next().unwrap()),
        }
    }
    result
}

fn parse_number(chars: &mut std::iter::Peekable<std::str::Chars>) -> f64 {
    let mut num_str = String::new();
    while let Some(&c) = chars.peek() {
        if c.is_numeric() || c == '.' || c == '-' {
            num_str.push(chars.next().unwrap());
        } else {
            break;
        }
    }
    num_str.parse::<f64>().unwrap()
}

fn parse_value(chars: &mut std::iter::Peekable<std::str::Chars>) -> JsonValue {
    while let Some(&c) = chars.peek() {
        match c {
            '"' => return JsonValue::String(parse_string(chars)),
            '0'..='9' | '-' => return JsonValue::Number(parse_number(chars)),
            '{' => return JsonValue::Object(parse_object(chars)),
            '[' => return JsonValue::Array(parse_array(chars)),
            't' => {
                for _ in 0..4 {
                    chars.next();
                }
                return JsonValue::Boolean(true);
            }
            'f' => {
                for _ in 0..5 {
                    chars.next();
                }
                return JsonValue::Boolean(false);
            }
            'n' => {
                for _ in 0..4 {
                    chars.next();
                }
                return JsonValue::Null;
            }
            _ => {
                chars.next();
            }
        }
    }
    JsonValue::Null
}

fn parse_object(chars: &mut std::iter::Peekable<std::str::Chars>) -> BTreeMap<String, JsonValue> {
    chars.next(); // consume the opening brace
    let mut object = BTreeMap::new();
    loop {
        while let Some(&c) = chars.peek() {
            if c == '}' {
                chars.next(); // consume the closing brace
                return object;
            } else if c == '"' {
                let key = parse_string(chars);
                while let Some(&c) = chars.peek() {
                    if c == ':' {
                        chars.next(); // consume the colon
                        break;
                    } else {
                        chars.next();
                    }
                }
                let value = parse_value(chars);
                object.insert(key, value);
            } else {
                chars.next();
            }
        }
    }
}

fn parse_array(chars: &mut std::iter::Peekable<std::str::Chars>) -> Vec<JsonValue> {
    chars.next(); // consume the opening bracket
    let mut array = Vec::new();
    loop {
        while let Some(&c) = chars.peek() {
            if c == ']' {
                chars.next(); // consume the closing bracket
                return array;
            } else if c != ',' {
                array.push(parse_value(chars));
            }
            chars.next();
        }
    }
}

pub fn parse(input: &str) -> JsonValue {
    let mut chars = input.chars().peekable();
    parse_value(&mut chars)
}
