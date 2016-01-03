use std::collections::HashSet;

#[derive(PartialEq)]
enum CharTypes {
    IsNumeric,
    IsAlphabetic,
    IsSpace,
    IsOther,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    IsVariable,
    IsNumeric,
    IsFunction,
    IsOperation,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub param: String,
    pub id: TokenType,
}

macro_rules! seq {
    ($($x:expr),+) => {
        [$($x,)+].iter().map(|&x| x).collect()
    }
}

impl Token {
    pub fn new<I>(buffer: I, id: TokenType) -> Token where I: Into<String> {
        Token {
            param: buffer.into(),
            id: id,
        }
    }
}

trait SpecTypes {
    fn get_type(&self) -> CharTypes;
}

impl SpecTypes for char {
    // get character type
    fn get_type(&self) -> CharTypes {
        if self.is_numeric() || *self == '.' {
            CharTypes::IsNumeric
        } else if self.is_alphabetic() {
            CharTypes::IsAlphabetic
        } else if *self == ' ' {
            CharTypes::IsSpace
        } else {
            CharTypes::IsOther
        }
    }
}

fn identify(item: &str) -> TokenType {
    // available functions
    let funcs: HashSet<&'static str> = seq!["exit", "print", "read", "stack"];
    // get first character from string
    let chr = item.chars().nth(0).unwrap();
    // check character type
    if chr.is_numeric() {
        TokenType::IsNumeric
    } else if funcs.contains(item) {
        TokenType::IsFunction
    } else if chr.is_alphabetic() {
        TokenType::IsVariable
    } else {
        TokenType::IsOperation
    }
}

pub fn tokenize<'a>(buffer: &'a str) -> Vec<Token> {
    // unsafe copy function
    fn strcpy<'a>(buffer: &'a str, start: usize, stop: usize) -> &'a str {
        unsafe { buffer.slice_unchecked(start, stop) }
    }
    let mut stack: Vec<Token> = Vec::new();
    let mut last_pos: usize = 0;
    let mut curr_pos: usize = 0;
    let mut last_type = CharTypes::IsOther;
    for character in buffer.chars() {
        let curr_type = character.get_type();
        if curr_type != last_type {
            // ignore empty data
            if curr_pos - last_pos > 0 && last_type != CharTypes::IsSpace {
                let tmp = strcpy(buffer, last_pos, curr_pos);
                let token_type = identify(tmp);
                stack.push(Token::new(tmp, token_type));
            }
            last_pos = curr_pos;
            last_type = curr_type;
        }
        curr_pos += 1;
    }
    // add last token to stack
    if last_type != CharTypes::IsSpace {
        let tmp = strcpy(buffer, last_pos, curr_pos);
        let token_type = identify(tmp);
        stack.push(Token::new(tmp, token_type));
    }
    stack
}
