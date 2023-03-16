use crate::ltypes::*;
use crate::src::load_and_lex_code;
use crate::stack::stack_runtime;
use crate::asm::*;
use std::path::Path;

pub struct Compiler {
    pub code: Vec<LOpType>,
    name: String,
}

impl Compiler {
    pub fn new(path: &str) -> Self {
        let file_name = Path::new(path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();
        let end_index = file_name.rfind('.').unwrap_or_else(|| file_name.len());
        let file_name = file_name[0..end_index].to_string();

        Self {
            code: load_and_lex_code(path),
            name: file_name,
        }
    }
}

impl std::fmt::Display for Compiler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Compiler(code:{:?})", self.code)
    }
}

impl Compiler {
    fn idx(&self, ip: u64) -> Option<u64> {
        let index = ((self.code.len() as u64).wrapping_sub(ip).wrapping_sub(1)) as usize;
        if index >= self.code.len() {
            return Option::None;
        }

        return Option::Some(index as u64);        
    }

    fn compile_asm(&self, file: &mut AsmFile) -> bool {
        let mut ptr: u64 = 0;
        let csize = self.code.len() as u64;
        while ptr < csize {
            let index = self.idx(ptr);
            if let None = index {
                return true;
            }

            let index = index.unwrap() as usize;
            let value = self.code.get(index);
            if let Option::None = value {
                return true;
            }

            let value = value.unwrap().clone();

            file.addr(ptr);
            
            match value {
                LOpType::Push(x) => {
                    match x {
                        LValue::Number(y) => {
                            file.title("push u64");
                            file.code(format!("push {}", y as u64).as_str());
                        },
                        _ => {
                            println!("Not implemented! Push {:?}", x);
                        }
                    }
                },
                LOpType::Add => {
                    file.title("add");
                    file.code("pop rax");
                    file.code("pop rbx");
                    file.code("add rax, rbx");
                    file.code("push rax");
                },
                LOpType::Sub => {
                    file.title("sub");
                    file.code("pop rax");
                    file.code("pop rbx");
                    file.code("sub rbx, rax");
                    file.code("push rbx");
                },
                LOpType::Mul => {
                    file.title("mul");
                    file.code("pop rax");
                    file.code("pop rbx");
                    file.code("mul rbx");
                    file.code("push rax");
                },
                LOpType::Div => {
                    file.title("div");
                    file.code("xor rdx, rdx");
                    file.code("pop rbx");
                    file.code("pop rax");
                    file.code("div rbx");
                    file.code("push rax");
                },
                LOpType::Mod => {
                    file.title("div");
                    file.code("xor rdx, rdx");
                    file.code("pop rbx");
                    file.code("pop rax");
                    file.code("div rbx");
                    file.code("push rdx");
                },
                LOpType::Log => {
                    file.title("log");
                    file.code("pop rcx");
                    file.code("call log");
                },
                LOpType::Drop => {
                    file.title("drop");
                    file.code("pop rax");
                },
                LOpType::Dup => {
                    file.title("dup");
                    file.code("pop rax");
                    file.code("push rax");
                    file.code("push rax");
                },
                LOpType::Over => {
                    file.title("over");
                    file.code("pop rax");
                    file.code("pop rbx");
                    file.code("push rbx");
                    file.code("push rax");
                    file.code("push rbx");
                },
                LOpType::Swap => {
                    file.title("swap");
                    file.code("pop rax");
                    file.code("pop rbx");
                    file.code("push rax");
                    file.code("push rbx");
                },
                LOpType::Greater => {
                    file.title(">");
                    file.code("xor rcx, rcx");
                    file.code("mov rdx, 1");
                    file.code("pop rbx");
                    file.code("pop rax");
                    file.code("cmp rax, rbx");
                    file.code("cmovg rcx, rdx");
                    file.code("push rcx");
                },
                LOpType::Less => {
                    file.title("<");
                    file.code("xor rcx, rcx");
                    file.code("mov rdx, 1");
                    file.code("pop rbx");
                    file.code("pop rax");
                    file.code("cmp rax, rbx");
                    file.code("cmovl rcx, rdx");
                    file.code("push rcx");
                },
                LOpType::GreaterEqual => {
                    file.title(">=");
                    file.code("xor rcx, rcx");
                    file.code("mov rdx, 1");
                    file.code("pop rbx");
                    file.code("pop rax");
                    file.code("cmp rax, rbx");
                    file.code("cmovge rcx, rdx");
                    file.code("push rcx");
                },
                LOpType::LessEqual => {
                    file.title("<=");
                    file.code("xor rcx, rcx");
                    file.code("mov rdx, 1");
                    file.code("pop rbx");
                    file.code("pop rax");
                    file.code("cmp rax, rbx");
                    file.code("cmovle rcx, rdx");
                    file.code("push rcx");
                },
                LOpType::Equal => {
                    file.title("=");
                    file.code("xor rcx, rcx");
                    file.code("mov rdx, 1");
                    file.code("pop rbx");
                    file.code("pop rax");
                    file.code("cmp rax, rbx");
                    file.code("cmove rcx, rdx");
                    file.code("push rcx");
                },
                LOpType::NotEqual => {
                    file.title("!=");
                    file.code("xor rcx, rcx");
                    file.code("mov rdx, 1");
                    file.code("pop rbx");
                    file.code("pop rax");
                    file.code("cmp rax, rbx");
                    file.code("cmovne rcx, rdx");
                    file.code("push rcx");
                },
                LOpType::If(block_ip) => {
                    file.title("if");
                    file.code("pop rax");
                    file.code("cmp rax, 0");
                    file.code(format!("je addr_{}", block_ip).as_str());
                },
                LOpType::Else(block_ip) => {
                    file.title("else");
                    file.code(format!("jmp addr_{}", block_ip).as_str());
                },
                LOpType::Do(block_ip) => {
                    file.title("do");
                    file.code("pop rax");
                    file.code("cmp rax, 0");
                    file.code(format!("je addr_{}", block_ip).as_str());
                },
                LOpType::While => {
                    file.title("while");
                },
                LOpType::End(block_ip) => {
                    file.title("end");
                    file.code(format!("jmp addr_{}", block_ip).as_str());
                },
                _ => {
                    println!("Not implemented! {:?}", value);
                    return false;
                }
            }

            ptr += 1;
        }

        file.addr(csize);

        return true;
    }

    pub fn compile(&self) -> bool {
        let mut asmfile = pre_compile(self.name.as_str());

        if !self.compile_asm(&mut asmfile) {
            println!("Failed to create ASM file!");
            return false;
        }

        if !post_compile(asmfile) {
            println!("Failed to compile Ktnack program!");
            return false;
        }

        return true;
    }
}
