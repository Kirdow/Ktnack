#![allow(unused)]

mod utils;
mod ltypes;
mod stack;
mod src;

use std::collections::HashMap;
use utils::*;
use ltypes::*;
use stack::*;
use src::*;

struct Runtime {
    code: Vec<LOpType>,
    stack: Vec<LValue>,
    ptr: u64,
    loops: Vec<u64>,
}

impl Runtime {
    fn new(path: &str) -> Self {
        Self {
            code: load_and_lex_code(path),
            stack: Vec::new(),
            ptr: 0,
            loops: Vec::new(),
        }
    }
}

impl std::fmt::Display for Runtime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Runtime(code:{:?},stack:{:?},ptr:{},loops:{:?})", self.code, self.stack, self.ptr, self.loops)
    }
}

fn main() {
    let mut runtime = Runtime::new("code.ktnck");
    let mut i = 0;
    let max_iter = 32768;
    debugln!("Start {}", runtime);
    while runtime.next() {
        i += 1;

        debugln!("Iter({}) {}({}) {}", i, runtime.ptr, runtime.get(), runtime);
        if i > max_iter {
            println!("Max Iter Reached : {}", max_iter);
            break;
        }
    }
    debugln!("End {}", runtime);
}



impl Runtime {
    fn idx(&self, ip: u64) -> Option<u64> {
        let index = ((self.code.len() as u64).wrapping_sub(ip).wrapping_sub(1)) as usize;
        if index >= self.code.len() {
            return Option::None;
        }

        return Option::Some(index as u64);        
    }

    fn is_sym(&self, ip: u64, sym: LOpType) -> Option<bool> {
        let index = self.idx(self.ptr);
        if let None = index {
            return None;
        }

        let index = index.unwrap() as usize;
        let value = self.code.get(index);
        if let Option::None = value {
            return None;
        }

        let value = value.unwrap();
        match value {
            LOpType::Push(_) => {
                return None;
            },
            _ => {
                return Some(std::mem::discriminant(&sym) == std::mem::discriminant(value));
            }
        }

    }

    fn get(&self) -> LOpType {
        let index = self.idx(self.ptr);
        if let None = index {
            return LOpType::Nop;
        }

        let index = index.unwrap() as usize;

        let value = self.code.get(index);
        if let Option::None = value {
            return LOpType::Nop;
        }

        return value.unwrap().clone();
    }

    fn get_next(&mut self) -> LOpType {
        let value = self.get();
        self.ptr += 1;
        return value;
    }

    fn next(&mut self) -> bool {
        let value = self.get();
        if let LOpType::Nop = value {
            return false;
        }

        let success = self.use_next(value);
        self.ptr += 1;
        success
    }

    fn use_next(&mut self, value: LOpType) -> bool {

        return match value {
            LOpType::Add => self.sym_add(),
            LOpType::Sub => self.sym_sub(),
            LOpType::Mul => self.sym_mul(),
            LOpType::Div => self.sym_div(),
            LOpType::Mod => self.sym_mod(),
            LOpType::Log => self.sym_log(),
            LOpType::Swap => self.sym_swap(),
            LOpType::Dup => self.sym_dup(),
            LOpType::Greater => self.sym_gt(),
            LOpType::GreaterEqual => self.sym_gte(),
            LOpType::Less => self.sym_lt(),
            LOpType::LessEqual => self.sym_lte(),
            LOpType::Equal => self.sym_eq(),
            LOpType::NotEqual => self.sym_neq(),
            LOpType::Push(x) => {
                self.stack.push(x);
                true
            },
            LOpType::While => true,
            LOpType::Do(repeat_ip) => {
                let cond = stack_runtime::pop_one(&mut self.stack);
                if let LValue::Number(x) = cond {
                    if x == 0.0 {
                        self.ptr = repeat_ip - 1;
                        true
                    } else {
                        true
                    }
                } else {
                    false
                }
            },
            LOpType::Repeat(while_ip) => {
                self.ptr = while_ip;
                true
            },
            LOpType::Drop => {
                stack_runtime::pop_one(&mut self.stack);
                true
            }
            _ => false,
        };

    }

    fn sym_add(&mut self) -> bool {
        let (a, b) = stack_runtime::pop_two(&mut self.stack);

        if let (LValue::Number(x), LValue::Number(y)) = (&a, &b) {
            let z = x + y;
            self.stack.push(LValue::Number(z));
        } else if let (LValue::Text(x), LValue::Number(y)) = (&a, &b) {
            let z = format!("{}{}", x, y);
            self.stack.push(LValue::Text(z));
        } else if let (LValue::Number(x), LValue::Text(y)) = (&a, &b) {
            let z = format!("{}{}", x, y);
            self.stack.push(LValue::Text(z));
        } else if let (LValue::Text(x), LValue::Text(y)) = (&a, &b) {
            let z = format!("{}{}", x, y);
            self.stack.push(LValue::Text(z));
        } else {
            println!("Error (add) types: {:?}", (a, b));
            return false;
        }

        return true;
    }

    fn sym_sub(&mut self) -> bool {
        let (a, b) = stack_runtime::pop_two(&mut self.stack);

        if let (LValue::Number(x), LValue::Number(y)) = (&a, &b) {
            let z = x - y;
            self.stack.push(LValue::Number(z));
        } else {
            println!("Error (sub) types: {:?}", (a, b));
            return false;
        }

        return true;
    }

    fn sym_mul(&mut self) -> bool {
        let (a, b) = stack_runtime::pop_two(&mut self.stack);

        if let (LValue::Number(x), LValue::Number(y)) = (&a, &b) {
            let z = x * y;
            self.stack.push(LValue::Number(z));
        } else {
            println!("Error (mul) types: {:?}", (a, b));
            return false;
        }

        return true;
    }

    fn sym_div(&mut self) -> bool {
        let (a, b) = stack_runtime::pop_two(&mut self.stack);

        if let (LValue::Number(x), LValue::Number(y)) = (&a, &b) {
            let z = x / y;
            self.stack.push(LValue::Number(z));
        } else {
            println!("Error (div) types: {:?}", (a, b));
            return false;
        }

        return true;
    }

    fn sym_mod(&mut self) -> bool {
        let (a, b) = stack_runtime::pop_two(&mut self.stack);

        if let (LValue::Number(x), LValue::Number(y)) = (&a, &b) {
            let z = x % y;
            self.stack.push(LValue::Number(z));
        } else {
            println!("Error (mod) types: {:?}", (a, b));
            return false;
        }

        return true;
    }

    fn sym_log(&mut self) -> bool {
        let a = stack_runtime::pop_one(&mut self.stack);

        if let LValue::Number(x) = &a {
            println!("{}", x);
        } else if let LValue::Text(x) = &a {
            println!("{}", x);
        } else {
            println!("Error (log) type: {:?}", a);
            return false;
        }

        return true;
    }

    fn sym_swap(&mut self) -> bool {
        let (a, b) = stack_runtime::pop_two(&mut self.stack);
        self.stack.push(b);
        self.stack.push(a);
        return true;
    }

    fn sym_dup(&mut self) -> bool {
        let a = stack_runtime::pop_one(&mut self.stack);
        self.stack.push(a.clone());
        self.stack.push(a);
        return true;
    }
    
    fn sym_repeat(&mut self) -> bool {
        /*let a = stack_runtime::pop_one(&mut self.stack);
        if let LValue::Number(x) = &a {
            let ret = *x != 0.0;
            let result = self.pop_loop(ret);

            if let None = result {
                println!("Unexpected (repeat): No loop!");
                return false;
            }
        } else {
            println!("Unexpected (repeat) type: {}", (a));
            return false;
        }*/

        return false;
    }

    fn sym_gt(&mut self) -> bool {
        let (a, b) = stack_runtime::pop_two(&mut self.stack);
        if let (LValue::Number(x), LValue::Number(y)) = (&a, &b) {
            self.stack.push(LValue::Number(if x > y { 1.0 } else { 0.0 }));
        } else {
            println!("Unexpected (>) types: {:?}", (a, b));
            return false;
        }

        return true;
    }

    fn sym_lt(&mut self) -> bool {
        let (a, b) = stack_runtime::pop_two(&mut self.stack);
        if let (LValue::Number(x), LValue::Number(y)) = (&a, &b) {
            self.stack.push(LValue::Number(if x < y { 1.0 } else { 0.0 }));
        } else {
            println!("Unexpected (<) types: {:?}", (a, b));
            return false;
        }

        return true;
    }

    fn sym_gte(&mut self) -> bool {
        let (a, b) = stack_runtime::pop_two(&mut self.stack);
        if let (LValue::Number(x), LValue::Number(y)) = (&a, &b) {
            self.stack.push(LValue::Number(if x >= y { 1.0 } else { 0.0 }));
        } else {
            println!("Unexpected (>=) types: {:?}", (a, b));
            return false;
        }

        return true;
    }

    fn sym_lte(&mut self) -> bool {
        let (a, b) = stack_runtime::pop_two(&mut self.stack);
        if let (LValue::Number(x), LValue::Number(y)) = (&a, &b) {
            self.stack.push(LValue::Number(if x <= y { 1.0 } else { 0.0 }));
        } else {
            println!("Unexpected (<=) types: {:?}", (a, b));
            return false;
        }

        return true;
    }

    fn sym_eq(&mut self) -> bool {
        let (a, b) = stack_runtime::pop_two(&mut self.stack);
        if let (LValue::Number(x), LValue::Number(y)) = (&a, &b) {
            self.stack.push(LValue::Number(if x == y { 1.0 } else { 0.0 }));
        } else if let (LValue::Text(x), LValue::Number(y)) = (&a, &b) {
            self.stack.push(LValue::Number(0.0));
        } else if let (LValue::Number(x), LValue::Text(y)) = (&a, &b) {
            self.stack.push(LValue::Number(0.0));
        } else if let (LValue::Text(x), LValue::Text(y)) = (&a, &b) {
            self.stack.push(LValue::Number(if *x == *y { 1.0 } else { 0.0 }));
        } else {
            println!("Unexpected (==) types: {:?}", (a, b));
            return false;
        }

        return true;
    }

    fn sym_neq(&mut self) -> bool {
        let (a, b) = stack_runtime::pop_two(&mut self.stack);
        if let (LValue::Number(x), LValue::Number(y)) = (&a, &b) {
            self.stack.push(LValue::Number(if x != y { 1.0 } else { 0.0 }));
        } else if let (LValue::Text(x), LValue::Number(y)) = (&a, &b) {
            self.stack.push(LValue::Number(1.0));
        } else if let (LValue::Number(x), LValue::Text(y)) = (&a, &b) {
            self.stack.push(LValue::Number(1.0));
        } else if let (LValue::Text(x), LValue::Text(y)) = (&a, &b) {
            self.stack.push(LValue::Number(if *x != *y { 1.0 } else { 0.0 }));
        } else {
            println!("Unexpected (!=) types: {:?}", (a, b));
            return false;
        }

        return true;
    }
}