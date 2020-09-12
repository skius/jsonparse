use std::collections::HashMap;
pub use self::Value::*;
use std::str::Chars;
use std::iter::Peekable;
use std::fmt::{Display, Debug, Formatter};
use std::fmt;
use std::ops::Index;

#[derive(Clone, PartialEq)]
pub enum Value {
    Int(i32),
    Float(f32),
    JsonString(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
    Bool(bool),
    Null,
}

impl Value {
    pub fn get_arr(&self, i: usize) -> Option<&Value> {
        match self {
            Value::Array(v) => v.get(i),
            _ => None,
        }
    }

    pub fn get_map(&self, key: &str) -> Option<&Value> {
        match self {
            Value::Object(map) => map.get(key),
            _ => None,
        }
    }
}

impl Index<&str> for Value {
    type Output = Value;

    fn index(&self, index: &str) -> &Self::Output {
        match self {
            Value::Object(map) => &map[index],
            _ => panic!("{} is not string-indexable", self),
        }
    }
}

impl Index<usize> for Value {
    type Output = Value;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Value::Array(v) => &v[index],
            _ => panic!("{} is not integer-indexable", self),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Null => write!(f, "null"),
            Int(i) => write!(f, "{}", i),
            Float(fl) => write!(f, "{}", fl),
            JsonString(j_s) => write!(f, "\"{}\"", j_s),
            Bool(b) => write!(f, "{}", b),
            Array(v) => {
                write!(f, "{:#?}", v)
            },
            Object(map) => write!(f, "{:#?}", map),
        }
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self, f)
    }
}

#[derive(Debug)]
pub enum Token {
    Value(Value),
    CurlyBracketOpen,
    CurlyBracketClose,
    BracketOpen,
    BracketClose,
    Comma,
    Colon,
}

pub struct Tokenizer<'a> {
    to_parse: Peekable<Chars<'a>>,
}

impl Tokenizer<'_> {
    pub fn new(to_parse: &str) -> Tokenizer {
        Tokenizer {
            to_parse: to_parse.chars().peekable(),
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        match self.to_parse.next()? {
            '{' => Some(Token::CurlyBracketOpen),
            '}' => Some(Token::CurlyBracketClose),
            '[' => Some(Token::BracketOpen),
            ']' => Some(Token::BracketClose),
            ',' => Some(Token::Comma),
            ':' => Some(Token::Colon),
            '"' => self.next_string(),
            c if c.is_whitespace() => self.next_token(),
            c if c == 't' => self.next_true(),
            c if c == 'f' => self.next_false(),
            c if c == 'n' => self.next_null(),
            c @ '0'..='9' => self.next_number(c),
            c => {
                println!("Couldn't parse: {}", c);
                None
            }
        }
    }

    fn next_number(&mut self, prev: char) -> Option<Token> {
        let mut found_number = String::from(prev);

        while let Some(c) = self.to_parse.peek() {
            if !('0'..='9').contains(c) && *c != '.' {
                break;
            }
            found_number.push(self.to_parse.next().unwrap());
        }

        if let Ok(i) = found_number.parse::<i32>() {
            return Some(Token::Value(Int(i)));
        } else if let Ok(f) = found_number.parse::<f32>() {
            return Some(Token::Value(Float(f)));
        }

        None
    }

    fn next_true(&mut self) -> Option<Token> {
        // we know prev char is t
        for _ in 0..3 {
            self.to_parse.next();
        }

        Some(Token::Value(Bool(true)))
    }

    fn next_false(&mut self) -> Option<Token> {
        // we know prev char is f
        for _ in 0..4 {
            self.to_parse.next();
        }

        Some(Token::Value(Bool(false)))
    }

    fn next_null(&mut self) -> Option<Token> {
        // we know prev char is n
        for _ in 0..3 {
            self.to_parse.next();
        }

        Some(Token::Value(Null))
    }

    fn next_string(&mut self) -> Option<Token> {
        // we know prev char is "
        let mut found_str: String = String::new();

        while let Some(c) = self.to_parse.next() {
            if c == '"' {
                break;
            }
            found_str.push(c);
        }

        // println!("found_str: {:?}", found_str);

        Some(Token::Value(JsonString(found_str)))
    }
}

impl Iterator for Tokenizer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}


pub struct Parser<'a> {
    t: Peekable<Tokenizer<'a>>,
}

impl Parser<'_> {
    pub fn new(input: &str) -> Parser {
        Parser {
            t: Tokenizer::new(input).peekable(),
        }
    }

    pub fn parse(mut self) -> Option<Value> {
        self.parse_value()
    }

    fn parse_object(&mut self) -> Option<Value> {
        let mut map: HashMap<String, Value> = HashMap::new();

        // Consume {
        self.t.next();

        while let Some(Token::Value(JsonString(_))) = self.t.peek() {
            match (self.t.next(), self.t.next()) {
                (Some(Token::Value(JsonString(s))), Some(Token::Colon)) => {
                    if let Some(val) = self.parse_value() {
                        map.insert(s, val);
                        if let Some(&Token::Comma) = self.t.peek() {
                            self.t.next();
                            continue;
                        } else {
                            break;
                        }
                    } else {
                        println!("Expected Value, got None");

                        break;
                    }
                },
                (_, Some(tok)) => {
                    println!("Unexpected Token: {:?}, expected ':'", tok);
                    return None;
                },
                (_, None) => {
                    println!("Unexpected EOF, expected ':'");
                    return None;
                }
            }
        }

        // Consume }
        match self.t.next() {
            Some(Token::CurlyBracketClose) => Some(Value::Object(map)),
            Some(tok) => {
                println!("Unexpected Token: {:?}, expected '}}'", tok);
                None
            }
            None => {
                println!("Unexpected EOF");
                None
            }
        }
    }

    fn parse_value(&mut self) -> Option<Value> {
        match self.t.peek()? {
            Token::CurlyBracketOpen => self.parse_object(),
            Token::BracketOpen => self.parse_array(),
            Token::Value(_) => if let Token::Value(val) = self.t.next().unwrap() {
                Some(val)
            } else {
                println!("Something majorly broken, peek returns valid Value but next not??");
                None
            },
            tok => {
                println!("Unexpected Token: {:?} while trying to parse Value", tok);
                None
            },

        }
    }

    fn parse_array(&mut self) -> Option<Value> {
        let mut vec: Vec<Value> = Vec::new();

        // Consume [
        self.t.next();

        if let Some(Token::BracketClose) = self.t.peek() {
            self.t.next();
            return Some(Array(vec));
        }

        while let Some(val) = self.parse_value() {
            vec.push(val.clone());

            // Consuming , or ]
            if let Some(tok) = self.t.next() {
                match tok {
                    Token::Comma => continue,
                    Token::BracketClose => break,
                    _ => {
                        println!("Matched something unexpected: {:?}", tok);
                        return None;
                    }
                }
            } else {
                println!("Unexpected EOF");
                return None;
            }
        }

        Some(Array(vec))
    }
}




pub fn example() {
    let json_str = r#"
    {
        "no": false,
        "inner_obj": {
            "inner_field": null,
            "inner_array_of_objects": [
                {
                    "in_obj_1_a": true,
                    "in_obj_1_b": 32.12345
                },
                {
                    "in_obj_2_a": [2,[3]]
                }
            ]
        },
        "some_number": 32,
        "array_thingy": [
            2, 3, "noooo"
        ],
        "a_string": "my_string",
        "test_string": "no"
    }
    "#;

    // Version with Option<&Value> sugar, returns None if index not found:

    let p = Parser::new(json_str);
    let val = p.parse();

    println!("{:?}", val.as_ref().get_map("inner_obj").get_map("inner_array_of_objects").get_arr(1));


    // Version with Index, panics if index not found:

    let p = Parser::new(json_str);
    let val = p.parse().unwrap();

    println!("{:?}", val["inner_obj"]["inner_array_of_objects"][1]);
}


// Extending Option<Value> to provide some sugar to work with Value
pub trait OptionValueExt {
    fn get_arr(&self, i: usize) -> Option<&Value>;
    fn get_map(&self, key: &str) -> Option<&Value>;
}

impl OptionValueExt for Option<&Value> {
    fn get_arr(&self, i: usize) -> Option<&Value> {
        match self {
            Some(val) => {
                val.get_arr(i)
            }
            _ => None
        }
    }

    fn get_map(&self, key: &str) -> Option<&Value> {
        match self {
            Some(val) => {
                val.get_map(key)
            }
            _ => None
        }
    }
}
