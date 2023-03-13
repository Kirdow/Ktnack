#![allow(unused)]

use std::fmt::Display;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashMap;

pub enum LValueType {
    Number(f32),
    Text(String),
    Symbol(String),
    None,
}

impl Clone for LValueType {
    fn clone(&self) -> Self {
        match self {
            Self::Number(x) => Self::Number(x.clone()),
            Self::Symbol(x) => Self::Symbol(x.clone()),
            Self::Text(x) => Self::Text(x.clone()),
            Self::None => Self::None,
        }
    }
}

impl std::fmt::Display for LValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LValueType::None => write!(f, "_"),
            LValueType::Text(x) => write!(f, "T\"{}\"", x),
            LValueType::Symbol(x) => write!(f, "S{}", x),
            LValueType::Number(x) => write!(f, "F{}", x),
        }
    }
}

impl std::fmt::Debug for LValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LValueType::None => write!(f, "_"),
            LValueType::Text(x) => write!(f, "T\"{}\"", x),
            LValueType::Symbol(x) => write!(f, "S{}", x),
            LValueType::Number(x) => write!(f, "F{}", x),
        }
    }
}

mod stack_code {
    use super::LValueType;

    pub fn pop_one(list: &mut Vec<String>) -> LValueType {
        let value = list.pop();
        if let Option::None = value {
            return LValueType::None;
        }

        let value = value.unwrap();

        if value.starts_with("\"") && value.ends_with("\"") {
            return LValueType::Text((&value[1..value.len() - 1]).to_string());
        } else if let Ok(f) = value.parse::<f32>() {
            return LValueType::Number(f);
        }

        return LValueType::Symbol(value);
    }

    pub fn pop_two(list: &mut Vec<String>) -> (LValueType, LValueType) {
        let first = pop_one(list);
        let second = pop_one(list);

        (first, second)
    }
}

mod stack_runtime {
    use super::LValueType;

    pub fn pop_one(list: &mut Vec<LValueType>) -> LValueType {
        list.pop().unwrap_or_else(|| LValueType::None)
    }


    pub fn pop_two(list: &mut Vec<LValueType>) -> (LValueType, LValueType) {
        let first = pop_one(list);
        let second = pop_one(list);

        (first, second)
    }
}

fn load_code(path: &str) -> Vec<String> {
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

    let mut code: Vec<String> = text.trim_start().split(" ").map(String::from).collect();
    let mut result: Vec<String> = Vec::new();

    code.reverse();
    while let Some(value) = code.pop() {
        if value.starts_with("\"") {
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

struct Runtime {
    code: Vec<String>,
    stack: Vec<LValueType>,
}

impl Runtime {
    fn new(path: &str) -> Self {
        Self {
            code: load_code(path),
            stack: Vec::new(),
        }
    }
}

impl Display for Runtime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Runtime(code:{:?},stack:{:?})", self.code, self.stack)
    }
}

fn main() {
    let mut runtime = Runtime::new("code.ktnck");
    let mut i = 0;
    let max_iter = 100;
    while runtime.next() {
        i += 1;

        if i > max_iter {
            println!("Max Iter Reached : {}", max_iter);
            break;
        }
    }
}

impl Runtime {
    fn next(&mut self) -> bool {
        let value = stack_code::pop_one(&mut self.code);
        if let LValueType::None = value {
            return false;
        }

        return self.use_next(value);
    }

    fn use_next(&mut self, value: LValueType) -> bool {
        match value {
            LValueType::Symbol(s) => {
                if s == "add" {
                    return self.sym_add();
                } else if s == "sub" {
                    return self.sym_sub();
                } else if s == "mul" {
                    return self.sym_mul();
                } else if s == "div" {
                    return self.sym_div();
                } else if s == "mod" {
                    return self.sym_mod();
                } else if s == "log" {
                    return self.sym_log();
                } else if s == "swap" {
                    return self.sym_swap();
                } else if s == "dup" {
                    return self.sym_dup();
                } else {
                    return false;
                }
            },
            _ => {
                self.stack.push(value);
            },
        }

        return true;
    }

    fn sym_add(&mut self) -> bool {
        let (a, b) = stack_runtime::pop_two(&mut self.stack);

        if let (LValueType::Number(x), LValueType::Number(y)) = (&a, &b) {
            let z = x + y;
            self.stack.push(LValueType::Number(z));
        } else if let (LValueType::Text(x), LValueType::Number(y)) = (&a, &b) {
            let z = format!("{}{}", x, y);
            self.stack.push(LValueType::Text(z));
        } else if let (LValueType::Number(x), LValueType::Text(y)) = (&a, &b) {
            let z = format!("{}{}", x, y);
            self.stack.push(LValueType::Text(z));
        } else if let (LValueType::Text(x), LValueType::Text(y)) = (&a, &b) {
            let z = format!("{}{}", x, y);
            self.stack.push(LValueType::Text(z));
        } else {
            println!("Error (add) types: {:?}", (a, b));
            return false;
        }

        return true;
    }

    fn sym_sub(&mut self) -> bool {
        let (a, b) = stack_runtime::pop_two(&mut self.stack);

        if let (LValueType::Number(x), LValueType::Number(y)) = (&a, &b) {
            let z = x - y;
            self.stack.push(LValueType::Number(z));
        } else {
            println!("Error (sub) types: {:?}", (a, b));
            return false;
        }

        return true;
    }

    fn sym_mul(&mut self) -> bool {
        let (a, b) = stack_runtime::pop_two(&mut self.stack);

        if let (LValueType::Number(x), LValueType::Number(y)) = (&a, &b) {
            let z = x * y;
            self.stack.push(LValueType::Number(z));
        } else {
            println!("Error (mul) types: {:?}", (a, b));
            return false;
        }

        return true;
    }

    fn sym_div(&mut self) -> bool {
        let (a, b) = stack_runtime::pop_two(&mut self.stack);

        if let (LValueType::Number(x), LValueType::Number(y)) = (&a, &b) {
            let z = x / y;
            self.stack.push(LValueType::Number(z));
        } else {
            println!("Error (div) types: {:?}", (a, b));
            return false;
        }

        return true;
    }

    fn sym_mod(&mut self) -> bool {
        let (a, b) = stack_runtime::pop_two(&mut self.stack);

        if let (LValueType::Number(x), LValueType::Number(y)) = (&a, &b) {
            let z = x % y;
            self.stack.push(LValueType::Number(z));
        } else {
            println!("Error (mod) types: {:?}", (a, b));
            return false;
        }

        return true;
    }

    fn sym_log(&mut self) -> bool {
        let a = stack_runtime::pop_one(&mut self.stack);

        if let LValueType::Number(x) = &a {
            println!("{}", x);
        } else if let LValueType::Text(x) = &a {
            println!("{}", x);
        } else {
            println!("Error (log) type: {:?}", a);
            return false;
        }

        return true;
    }

    fn sym_swap(&mut self) -> bool {
        let (a, b) = stack_runtime::pop_two(&mut self.stack);
        self.stack.push(a);
        self.stack.push(b);
        return true;
    }

    fn sym_dup(&mut self) -> bool {
        let a = stack_runtime::pop_one(&mut self.stack);
        self.stack.push(a.clone());
        self.stack.push(a);
        return true;
    }
}