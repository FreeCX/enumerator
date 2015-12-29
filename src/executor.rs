use tokenizer::{Token, TokenType};
use std::collections::HashMap;

// get operation priority
fn get_priority(operator: &str) -> Option<i8> {
    match operator {
        "(" => Some(-1),
        "^" => Some(0),
        "*" | "/" | "%" => Some(1),
        "+" | "-" => Some(2),
        _ => None,
    }
}

// check for the possibility of pulling out of the stack
fn can_pop(op1: &Token, stack: &Vec<Token>) -> bool {
    if stack.len() == 0 {
        return false;
    }
    let p1 = match get_priority(&op1.param) {
        Some(value) => value,
        None => {
            println!("> error: unknown operator '{}'", op1.param);
            return false;
        }
    };
    let last = match stack.last() {
        Some(value) => value,
        None => {
            println!("> error: function stack is empty");
            return false;
        }
    };
    let p2 = match get_priority(&last.param) {
        Some(value) => value,
        None => {
            println!("> error: unknown operator '{}'", last.param);
            return false;
        }
    };
    p1 >= 0 && p2 >= 0 && p1 >= p2
}

// convert infix notation to reverse polish
fn in2rpn(variables: &mut HashMap<String, f32>, tokens: &Vec<Token>) -> Vec<Token> {
    let mut result: Vec<Token> = Vec::new();
    let mut funcs: Vec<Token> = Vec::new();
    let mut token_counter = 0;
    for item in tokens {
        token_counter += 1;
        match item.id {
            // push numeric value to result vector
            TokenType::IsNumeric => {
                result.push(item.clone());
            }
            // get value from hashmap and push to result
            TokenType::IsVariable => {
                if token_counter == 1 {
                    continue;
                }
                let token = match variables.get(&item.param) {
                    Some(value) => format!("{}", value.clone()),
                    None => {
                        println!("> error: can't found {} variable!", item.param);
                        break;
                    }
                };
                result.push(Token::new(token.clone(), TokenType::IsNumeric));
            }
            TokenType::IsFunction => {
                continue;
            }
            TokenType::IsOperation => {
                if item.param == ")" {
                    // push all acumulated functions in brackets
                    while funcs.len() > 0 && funcs.last().unwrap().param != "(" {
                        let function = match funcs.pop() {
                            Some(value) => value,
                            None => {
                                println!("> error: function stack is empty");
                                break;
                            }
                        };
                        result.push(function);
                    }
                    funcs.pop();
                } else if item.param == "<-" {
                    continue;
                } else {
                    // push other functions
                    while can_pop(item, &funcs) {
                        let function = match funcs.pop() {
                            Some(value) => value,
                            None => {
                                println!("> error: function stack is empty");
                                break;
                            }
                        };
                        result.push(function);
                    }
                    funcs.push(item.clone());
                }
            }
        }
    }
    // push rest functions
    if funcs.len() > 0 {
        result.push(funcs.pop().unwrap());
    }
    result
}

// reverse polish notation to value
fn rpn2value(line: &Vec<Token>) -> f32 {
    let mut stack: Vec<f32> = Vec::new();
    for token in line {
        if token.id == TokenType::IsOperation {
            // get first operand
            let a = match stack.pop() {
                Some(value) => value,
                None => {
                    println!("> error: stack is empty!");
                    break;
                }
            };
            // get second operand
            let b = match stack.pop() {
                Some(value) => value,
                None => {
                    println!("> error: stack is empty!");
                    break;
                }
            };
            // calculate statement
            let result: f32 = match token.param.as_ref() {
                "+" => b + a,
                "-" => b - a,
                "*" => b * a,
                "/" => b / a,
                "%" => b % a,
                "^" => b.powf(a),
                _ => continue,
            };
            stack.push(result);
        } else if token.id == TokenType::IsNumeric {
            // add numeric value to stack
            stack.push(match token.param.parse() {
                Ok(value) => value,
                Err(why) => {
                    println!("> error: {}", why);
                    continue;
                }
            });
        }
    }
    // return first value of stack
    *stack.get(0).unwrap()
}

// check tokens type and calculate it
pub fn execute(variables: &mut HashMap<String, f32>, tokens: &Vec<Token>) -> bool {
    // get first token
    let first = tokens.get(0).unwrap();
    // get second token
    let operator = if tokens.len() > 1 {
        tokens.get(1)
    } else {
        None
    };
    // if first token in function
    if first.id == TokenType::IsFunction {
        match first.param.as_ref() {
            "print" => {
                let mut calculate_it = false;
                for item in tokens.iter().skip(1) {
                    if item.id == TokenType::IsOperation {
                        calculate_it = true;
                        break;
                    }
                }
                if calculate_it {
                    // convert infix to reverse polish
                    let rpn_statement = in2rpn(variables, tokens);
                    // calucate reverse polish
                    let value = rpn2value(&rpn_statement);
                    println!("{} ", value);
                } else {
                    // iterate by all tokens (except first)
                    for item in tokens.iter().skip(1) {
                        match item.id {
                            // find variable in hashmap and print
                            TokenType::IsVariable => {
                                match variables.get(&item.param) {
                                    Some(var) => println!("{} ", var),
                                    None => {
                                        println!("> error: variable '{}' is not defined!",
                                                 item.param);
                                        continue;
                                    }
                                }
                            }
                            // print numeric value
                            TokenType::IsNumeric => println!("{}", item.param),
                            // say no way!
                            TokenType::IsFunction | TokenType::IsOperation => {
                                println!("type '{}' : {:?}", item.param, item.id)
                            }
                        }
                    }
                }
            }
            "exit" => return true,
            _ => println!("> error: function '{}' not defined", first.param),
        }
    } else {
        let operator = match operator {
            Some(value) => value,
            None => {
                println!("> error: can't execute code");
                return false;
            }
        };
        // if second token is operation -- caclucate statement
        if operator.id == TokenType::IsOperation {
            match operator.param.as_ref() {
                "<-" => {
                    // convert infix to reverse polish
                    let rpn_statement = in2rpn(variables, tokens);
                    // calucate reverse polish
                    let value = rpn2value(&rpn_statement);
                    // add value to hashmap
                    variables.insert(first.param.to_owned(), value);
                }
                _ => println!("> error: operator '{}' is not defined!", operator.param),
            }
        }
    }
    false
}
