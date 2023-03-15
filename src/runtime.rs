use crate::ltypes::*;
use crate::src::load_and_lex_code;
use crate::stack::stack_runtime;
use crate::compile::*;

pub struct Runtime {
    pub code: Vec<LOpType>,
    pub stack: Vec<LValue>,
    pub ptr: u64,
}

impl Runtime {
    pub fn new(path: &str) -> Self {
        Self {
            code: load_and_lex_code(path),
            stack: Vec::new(),
            ptr: 0,
        }
    }
}

impl std::fmt::Display for Runtime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Runtime(code:{:?},stack:{:?},ptr:{})", self.code, self.stack, self.ptr)
    }
}

impl Runtime {
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
        let mut asmfile = pre_compile("ktnckc");

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
