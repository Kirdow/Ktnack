use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::ltypes::*;

pub fn convert_string_to_lvalue(s: &String) -> LValueType {
    if s.starts_with("\"") && s.ends_with("\"") {
        return LValueType::Text((s[1..s.len() - 1]).to_string());
    } else if let Ok(f) = s.parse::<f32>() {
        return LValueType::Number(f);
    }

    return LValueType::Symbol(s.clone());
}

pub fn load_and_lex_code(path: &str) -> Vec<LOpType> {
    let mut code = load_code(path);
    code.reverse();
    let mut result: Vec<LOpType> = Vec::new();

    let mut ip = 0;

    let mut stack: Vec<i32> = Vec::new();

    for item in code.iter() {
        let value = convert_string_to_lvalue(item);
        
        let op_type = match value {
            LValueType::Number(x) => LOpType::Push(LValue::Number(x)),
            LValueType::Text(x) => LOpType::Push(LValue::Text(x)),
            LValueType::Symbol(sym) => {
                if sym == "add" || sym == "+" {
                    LOpType::Add
                } else if (sym == "sub" || sym == "-") {
                    LOpType::Sub
                } else if (sym == "mul" || sym == "*") {
                    LOpType::Mul
                } else if (sym == "div" || sym == "/") {
                    LOpType::Div
                } else if (sym == "mod" || sym == "%") {
                    LOpType::Mod
                } else if (sym == "log" || sym == ".") {
                    LOpType::Log
                } else if (sym == "swap" || sym == "s") {
                    LOpType::Swap
                } else if (sym == "dup") {
                    LOpType::Dup
                } else if (sym == ">") {
                    LOpType::Greater
                } else if (sym == "<") {
                    LOpType::Less
                } else if (sym == ">=") {
                    LOpType::GreaterEqual
                } else if (sym == "<=") {
                    LOpType::LessEqual
                } else if (sym == "=") {
                    LOpType::Equal
                } else if (sym == "!=") {
                    LOpType::NotEqual
                } else if (sym == "while") {
                    stack.push(ip);
                    LOpType::While
                } else if (sym == "do") {
                    let while_ip = stack.pop().unwrap_or(-1);
                    stack.push(ip);
                    LOpType::Do(while_ip as u64)
                } else if (sym == "repeat") {
                    let block_ip = stack.pop().unwrap_or(-1);
                    if let LOpType::Do(x) = result.get(block_ip as usize).unwrap().clone() {
                        result[block_ip as usize] = LOpType::Do((ip + 1) as u64);
                        LOpType::Repeat(x)
                    } else {
                        LOpType::Nop
                    }
                } else if (sym == "drop") {
                    LOpType::Drop
                } else {
                    LOpType::Nop
                }
            },
            LValueType::None => LOpType::Nop,
        };

        result.push(op_type);

        ip += 1;
    }

    result.reverse();
    return result;
}

pub fn load_code(path: &str) -> Vec<String> {
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    
    let mut text = String::new();
    for line in buffered.lines() {
        match line {
            Ok(x) => {
                text.push_str(format!(" {}", x).as_str());
            },
            Err(_) => todo!(),
        }
    }

    let mut code: Vec<String> = text.trim_start().split(" ").map(|p| p.trim()).filter(|p| !p.is_empty()).map(String::from).collect();
    let mut result: Vec<String> = Vec::new();

    code.reverse();
    while let Some(value) = code.pop() {
        if value.starts_with("\"") {
            if (value.ends_with("\"") && value.len() >= 2) {
                result.push(value);
                continue;
            }
            let mut list: Vec<String> = vec![value];
            while let Some(value2) = code.pop() {
                let end = value2.ends_with("\"");
                list.push(value2);
                if end {
                    break;
                }
            }

            result.push(list.join(" "));
        } else {
            result.push(value);
        }
    }

    result.reverse();

    return result;
}