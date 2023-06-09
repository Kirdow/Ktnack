use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::ltypes::*;
use crate::strings::*;

pub fn convert_string_to_lvalue(s: &String) -> LValueType {
    if s.starts_with("\"") && s.ends_with("\"") {
        return LValueType::Text((s[1..s.len() - 1]).to_string());
    } else if s.starts_with("'") && s.ends_with("'") {
        if let Some(val) = s.chars().nth(1) {
            return LValueType::Char(val as i64);
        }
    } else if let Ok(i) = s.parse::<i64>() {
        return LValueType::Number(i);
    }

    return LValueType::Symbol(s.clone());
}

fn load_macros_and_expand(raw_code: Vec<LValueType>) -> Vec<LValueType> {
    let mut macros: HashMap<String, LMacro> = HashMap::new();
    let mut code: Vec<LValueType> = Vec::new();

    fn is_macro_end(lvalue: Option<LValueType>, count: &mut i32) -> bool {
        if let Some(lvalue) = lvalue {
            if let LValueType::Symbol(sym) = lvalue {
                if sym == "end" {
                    if *count == 0 {
                        return true;
                    }
                    *count -= 1;
                } else if sym == "if" || sym == "while" {
                    *count += 1;
                }
            }
        }

        return false;
    }

    fn get_macro_text(lvalue: Option<LValueType>) -> Option<String> {
        if let Some(lvalue) = lvalue {
            if let LValueType::Symbol(sym) = lvalue {
                return Option::Some(sym.to_owned());
            }
        }

        return Option::None;
    }

    fn is_macro_start(lvalue: Option<LValueType>) -> bool {
        if let Some(lvalue) = lvalue {
            if let LValueType::Symbol(sym) = lvalue {
                if sym == "macro" {
                    return true;
                }
            }
        }

        return false;
    }

    fn clone_lvalue(lvalue: Option<&LValueType>) -> Option<LValueType> {
        match lvalue {
            None => None,
            Some(value) => Some(value.to_owned()),
        }
    }

    let mut it = raw_code.iter();
    while let Some(item) = it.next() {
        if !is_macro_start(Option::Some(item.clone())) {
            code.push(item.to_owned());
            continue;
        }
        
        let next = it.next();
        let mut macro_name: Option<String> = Option::None;
        match get_macro_text(clone_lvalue(next)) {
            Option::None => {
                println!("Failed to lex macro! No name found!");
                return code;
            },
            Option::Some(text) => {
                macro_name = Option::Some(text);
            }
        }

        let macro_name = macro_name.unwrap();

        let mut body: Vec<LValueType> = Vec::new();
        let mut success: bool = false;
        let mut count: i32 = 0;
        while let Some(value) = it.next() {
            if is_macro_end(Option::Some(value.to_owned()), &mut count) {
                let mcro = LMacro::new(&macro_name, &body);
                macros.insert(macro_name, mcro);
                success = true;
                break;
            } else {
                body.push(value.to_owned());
            }
        }

        if !success {
            println!("Failed to read macro!");
            return Vec::new();
        }
    }

    let mut result: Vec<LValueType> = Vec::new();

    let mut it = code.iter();
    while let Some(item) = it.next() {
        match get_macro_text(Option::Some(item.clone())) {
            None => {
                result.push(item.to_owned());
            },
            Some(text) => {
                if let Some(mcro) = macros.get(&text) {
                    let mut expanded = mcro.expand(&macros, 8);
                    result.append(&mut expanded);
                } else {
                    result.push(item.to_owned());
                }
            }
        }
    }
    
    return result;
}

pub fn load_and_lex_code(path: &str) -> Vec<LOpType> {
    let code: Vec<LValueType> = load_code(path).iter().map(|x|convert_string_to_lvalue(x)).rev().collect();
    let code = load_macros_and_expand(code);

    let mut result: Vec<LOpType> = Vec::new();

    let mut ip = 0;

    let mut stack: Vec<i32> = Vec::new();

    for value in code.iter() {
        
        let op_type = match value {
            LValueType::Number(x) => LOpType::Push(LValue::Number(x.clone())),
            LValueType::Text(x) => LOpType::Push(LValue::Text(x.clone())),
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
                } else if (sym == "shl" || sym == "<<") {
                    LOpType::Shl
                } else if (sym == "shr" || sym == ">>") {
                    LOpType::Shr
                } else if (sym == "bor" || sym == "|") {
                    LOpType::Bor
                } else if (sym == "band" || sym == "&") {
                    LOpType::Band
                } else if (sym == "log" || sym == ".") {
                    LOpType::Log
                } else if (sym == "swap" || sym == "s") {
                    LOpType::Swap
                } else if (sym == "dup") {
                    LOpType::Dup
                } else if (sym == "over") {
                    LOpType::Over
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
                } else if (sym == "if") {
                    stack.push(ip);
                    LOpType::If(0)
                } else if (sym == "else") {
                    let block_ip = stack.pop().unwrap_or(-1);
                    let result_sym = result.get(block_ip as usize).unwrap();
                    if let LOpType::If(x) = &result_sym {
                        result[block_ip as usize] = LOpType::If((ip + 1) as u64);
                        stack.push(ip);
                        LOpType::Else(0)
                    } else {
                        LOpType::Nop(format!("else/sym:{:?}", result_sym).to_string())
                    }
                } else if (sym == "while") {
                    stack.push(ip);
                    LOpType::While
                } else if (sym == "do") {
                    let while_ip = stack.pop().unwrap_or(-1);
                    stack.push(ip);
                    LOpType::Do(while_ip as u64)
                } else if (sym == "end") {
                    let block_ip = stack.pop().unwrap_or(-1);
                    let op = result.get(block_ip as usize).unwrap();
                    if let LOpType::If(x) = op.clone() {
                        result[block_ip as usize] = LOpType::If((ip + 1) as u64);
                        LOpType::End((ip + 1) as u64)
                    } else if let LOpType::Else(x) = op.clone() {
                        result[block_ip as usize] = LOpType::Else((ip + 1) as u64);
                        LOpType::End((ip + 1) as u64)
                    } else if let LOpType::Do(x) = op.clone() {
                        result[block_ip as usize] = LOpType::Do((ip + 1) as u64);
                        LOpType::End(x)
                    } else {
                        LOpType::Nop(format!("end/sym:{:?}", op).to_string())
                    }
                } else if (sym == "drop") {
                    LOpType::Drop
                } else if (sym == "store" || sym == "S") {
                    LOpType::Store
                } else if (sym == "load" || sym == "L") {
                    LOpType::Load
                } else if (sym == "@") {
                    LOpType::Mem
                } else if (sym == "P") {
                    LOpType::Puts(true)
                } else if (sym == "p") {
                    LOpType::Puts(false)
                } else {
                    LOpType::Nop(format!("lex:{}", sym).to_string())
                }
            },
            LValueType::Char(x) => {
                LOpType::Push(LValue::Number(x.clone()))
            },
            LValueType::None => LOpType::Nop(String::from("Invalid token type!")),
        };

        result.push(op_type);

        ip += 1;
    }

    result.reverse();
    return result;
}

fn load_code_file(path: &str) -> String {
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);

    let mut text = String::new();
    for line in buffered.lines() {
        match line {
            Ok(x) => {
                if x.starts_with("inc ") {
                    let sub_text = load_code_file(format!("{}.ktnck", &x[4..]).as_str());
                    text.push_str(format!(" {}", sub_text).as_str());
                } else {
                    text.push_str(format!(" {}", x).as_str());
                }
            },
            Err(_) => panic!(),
        }
    }

    return text.trim_start().to_owned();
}

fn get_code_words(text: &String) -> Vec<String> {
    let mut words: Vec<String> = text.trim_start()
        .split(" ")
        .map(|p| p.trim())
        .filter(|p| !p.is_empty())
        .map(String::from)
        .collect();
    words.reverse();
    words
}

fn get_code_words_and_strings(text: &String) -> Vec<String> {
    let mut code = get_code_words(text);
    let mut result: Vec<String> = Vec::new();

    while let Some(value) = code.pop() {
        result.push(fetch_string(&value, &mut code));
    }

    result.reverse();

    result
}

pub fn load_code(path: &str) -> Vec<String> {
    let text = load_code_file(path);

    let code_tokens = get_code_words_and_strings(&text);

    println!("Code tokens: {}", code_tokens.len());

    code_tokens
}