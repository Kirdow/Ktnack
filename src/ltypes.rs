
use std::collections::HashMap;

pub enum LValueType {
    Number(i64),
    Text(String),
    Symbol(String),
    None
}

pub struct Loop {
    start: u64,
    cond: u64,
    end: u64,
}

pub enum LValue {
    Number(i64),
    Text(String),
}

pub enum LOpType {
    Nop(String),
    Push(LValue),
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Shl,
    Shr,
    Bor,
    Band,
    Log,
    Swap,
    Dup,
    Over,
    If(u64),
    Else(u64),
    While,
    Do(u64),
    End(u64),
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    Equal,
    NotEqual,
    Drop,
    Mem,
    Load,
    Store,
    Puts(bool),
}

pub struct LMacro {
    name: String,
    body: Vec<LValueType>,
}

impl Clone for LValue {
    fn clone(&self) -> Self {
        match self {
            Self::Number(x) => Self::Number(x.clone()),
            Self::Text(x) => Self::Text(x.clone()),
        }
    }
}

impl Clone for LOpType {
    fn clone(&self) -> Self {
        match self {
            Self::Nop(x) => Self::Nop(x.clone()),
            Self::Push(x) => Self::Push(x.clone()),
            Self::Add => Self::Add,
            Self::Sub => Self::Sub,
            Self::Mul => Self::Mul,
            Self::Div => Self::Div,
            Self::Mod => Self::Mod,
            Self::Shl => Self::Shl,
            Self::Shr => Self::Shr,
            Self::Bor => Self::Bor,
            Self::Band => Self::Band,
            Self::Log => Self::Log,
            Self::Swap => Self::Swap,
            Self::Dup => Self::Dup,
            Self::Over => Self::Over,
            Self::If(x) => Self::If(x.clone()),
            Self::Else(x) => Self::Else(x.clone()),
            Self::While => Self::While,
            Self::Do(x) => Self::Do(x.clone()),
            Self::End(x) => Self::End(x.clone()),
            Self::Greater => Self::Greater,
            Self::Less => Self::Less,
            Self::GreaterEqual => Self::GreaterEqual,
            Self::LessEqual => Self::LessEqual,
            Self::Equal => Self::Equal,
            Self::NotEqual => Self::NotEqual,
            Self::Drop => Self::Drop,
            Self::Mem => Self::Mem,
            Self::Load => Self::Load,
            Self::Store => Self::Store,
            Self::Puts(x) => Self::Puts(x.clone()),
        }
    }
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

impl std::fmt::Display for LValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LValue::Number(x) => write!(f, "Val({:?})", x),
            LValue::Text(x) => write!(f, "Val(\"{:?}\")", x),
        }
    }
}

impl std::fmt::Debug for LValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LValue::Number(x) => write!(f, "Val({:?})", x),
            LValue::Text(x) => write!(f, "Val(\"{:?}\")", x),
        }
    }
}

impl std::fmt::Display for LOpType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LOpType::Nop(x) => write!(f, "Nop({})", x),
            LOpType::Add => write!(f, "Add"),
            LOpType::Sub => write!(f, "Sub"),
            LOpType::Mul => write!(f, "Mul"),
            LOpType::Div => write!(f, "Div"),
            LOpType::Mod => write!(f, "Mod"),
            LOpType::Shl => write!(f, "Shl"),
            LOpType::Shr => write!(f, "Shr"),
            LOpType::Bor => write!(f, "Bor"),
            LOpType::Band => write!(f, "Band"),
            LOpType::Log => write!(f, "Log"),
            LOpType::Swap => write!(f, "Swap"),
            LOpType::Dup => write!(f, "Dup"),
            LOpType::Over => write!(f, "Over"),
            LOpType::Greater => write!(f, "Greater"),
            LOpType::Less => write!(f, "Less"),
            LOpType::GreaterEqual => write!(f, "GreaterEqual"),
            LOpType::LessEqual => write!(f, "LessEqual"),
            LOpType::Equal => write!(f, "Equal"),
            LOpType::NotEqual => write!(f, "NotEqual"),
            LOpType::Push(value) => write!(f, "Push({:?})", value),
            LOpType::If(x) => write!(f, "If(block:{})", x),
            LOpType::Else(x) => write!(f, "Else(block:{})", x),
            LOpType::While => write!(f, "While"),
            LOpType::Do(x) => write!(f, "Do(end:{})", x),
            LOpType::End(x) => write!(f, "End(block:{})", x),
            LOpType::Drop => write!(f, "Drop"),
            LOpType::Mem => write!(f, "Mem"),
            LOpType::Load => write!(f, "Load"),
            LOpType::Store => write!(f, "Store"),
            LOpType::Puts(x) => write!(f, "Puts(nl:{})", x),
        }
    }
}

impl std::fmt::Debug for LOpType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LOpType::Nop(x) => write!(f, "Nop({})", x),
            LOpType::Add => write!(f, "Add"),
            LOpType::Sub => write!(f, "Sub"),
            LOpType::Mul => write!(f, "Mul"),
            LOpType::Div => write!(f, "Div"),
            LOpType::Mod => write!(f, "Mod"),
            LOpType::Shl => write!(f, "Shl"),
            LOpType::Shr => write!(f, "Shr"),
            LOpType::Bor => write!(f, "Bor"),
            LOpType::Band => write!(f, "Band"),
            LOpType::Log => write!(f, "Log"),
            LOpType::Swap => write!(f, "Swap"),
            LOpType::Dup => write!(f, "Dup"),
            LOpType::Over => write!(f, "Over"),
            LOpType::Greater => write!(f, "Greater"),
            LOpType::Less => write!(f, "Less"),
            LOpType::GreaterEqual => write!(f, "GreaterEqual"),
            LOpType::LessEqual => write!(f, "LessEqual"),
            LOpType::Equal => write!(f, "Equal"),
            LOpType::NotEqual => write!(f, "NotEqual"),
            LOpType::Push(value) => write!(f, "Push({:?})", value),
            LOpType::If(x) => write!(f, "If(block:{})", x),
            LOpType::Else(x) => write!(f, "Else(block:{})", x),
            LOpType::While => write!(f, "While"),
            LOpType::Do(x) => write!(f, "Do(end:{})", x),
            LOpType::End(x) => write!(f, "End(block:{})", x),
            LOpType::Drop => write!(f, "Drop"),
            LOpType::Mem => write!(f, "Mem"),
            LOpType::Load => write!(f, "Load"),
            LOpType::Store => write!(f, "Store"),
            LOpType::Puts(x) => write!(f, "Puts(nl:{})", x),
        }
    }
}

impl std::fmt::Display for LValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LValueType::None => write!(f, "_"),
            LValueType::Text(x) => write!(f, "T\"{}\"", x),
            LValueType::Symbol(x) => write!(f, "S{}", x),
            LValueType::Number(x) => write!(f, "i{}", x),
        }
    }
}

impl std::fmt::Debug for LValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LValueType::None => write!(f, "_"),
            LValueType::Text(x) => write!(f, "T\"{}\"", x),
            LValueType::Symbol(x) => write!(f, "S{}", x),
            LValueType::Number(x) => write!(f, "i{}", x),
        }
    }
}

impl Loop {
    fn new(start: u64, cond: u64, end: u64) -> Self {
        Self {
            start: start,
            cond: cond,
            end: end,
        }
    }
}

impl Clone for Loop {
    fn clone(&self) -> Self {
        Self {
            start: self.start,
            cond: self.cond,
            end: self.end,
        }
    }
}

impl std::fmt::Display for Loop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Loop(start:{},cond:{},end:{})", self.start, self.cond, self.end)
    }
}

impl std::fmt::Debug for Loop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Loop(start:{},cond:{},end:{})", self.start, self.cond, self.end)
    }
}

impl LMacro {
    pub fn new(name: &String, body: &Vec<LValueType>) -> Self {
        Self {
            name: name.to_owned(),
            body: body.clone(),
        }
    }

    pub fn expand(&self, macros: &HashMap<String, LMacro>, max_depth: i32) -> Vec<LValueType> {
        let mut result: Vec<LValueType> = Vec::new();        
        if max_depth < 0 {
            return result;
        }

        for value in self.body.iter() {
            match value {
                LValueType::Symbol(sym) => {
                    if let Some(mcro) = macros.get(sym) {
                        let mut expanded = mcro.expand(macros, max_depth - 1);
                        result.append(&mut expanded);
                    } else {
                        result.push(value.clone());
                    }
                },
                _ => {
                    result.push(value.clone());
                }
            }
        }

        result
    }
}