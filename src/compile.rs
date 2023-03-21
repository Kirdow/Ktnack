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

    fn get_op_type(&self, ptr: u64) -> Option<LOpType> {
        let index = self.idx(ptr);
        if let None = index {
            return None;
        }

        let index = index.unwrap() as usize;
        let value = self.code.get(index);
        if let None = value {
            return None;
        }

        return Some(value.unwrap().clone());
    }

    fn compile_asm(&self, file: &mut AsmFile) -> bool {
        let mut ptr: u64 = 0;
        let csize = self.code.len() as u64;
        let mut strs: Vec<String> = Vec::new();
        while ptr < csize {
            let value = self.get_op_type(ptr);
            if let None = value {
                return true;
            }

            let value = value.unwrap();

            file.addr(ptr);
            
            match value {
                LOpType::Push(x) => {
                    match x {
                        LValue::Number(y) => {
                            file.title("push u64");
                            file.code(format!("push {}", y as u64).as_str());
                        },
                        LValue::Text(text) => {
                            file.title(format!("push str lit {} \"{}\":{}", strs.len(), text.as_str(), text.len()).as_str());
                            file.code(format!("lea rax, [rel str_{}]", strs.len()).as_str());
                            file.code("push rax");
                            file.code(format!("push {}", text.len()).as_str());
                            strs.push(text);
                        },
                        _ => {
                            println!("Not implemented! Push {:?}", x);
                        }
                    }
                },
                LOpType::Add => {
                    file.title("add");
                    file.code("pop rax");
                    file.code("add [rsp], rax");
                },
                LOpType::Sub => {
                    file.title("sub");
                    file.code("pop rax");
                    file.code("sub [rsp], rax");
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
                LOpType::Shl => {
                    file.title("shift left");
                    file.code("pop rcx");
                    file.code("shl qword [rsp], cl");
                },
                LOpType::Shr => {
                    file.title("shift right");
                    file.code("pop rcx");
                    file.code("shr qword [rsp], cl");
                },
                LOpType::Bor => {
                    file.title("bitwise or");
                    file.code("pop rax");
                    file.code("or [rsp], rax");
                },
                LOpType::Band => {
                    file.title("bitwise and");
                    file.code("pop rax");
                    file.code("and [rsp], rax");
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
                    file.code("mov rax, [rsp]");
                    file.code("push rax");
                },
                LOpType::Over => {
                    file.title("over");
                    file.code("mov rax, [rsp+8]");
                    file.code("push rax");
                },
                LOpType::Swap => {
                    file.title("swap");
                    file.code("pop rax");
                    file.code("xchg rax, [rsp]");
                    file.code("push rax");
                },
                LOpType::Greater => {
                    file.title(">");
                    file.code("pop rbx");
                    file.code("pop rax");
                    file.code("xor rcx, rcx");
                    file.code("cmp rax, rbx");
                    file.code("setg cl");
                    file.code("movzx rcx, cl");
                    file.code("push rcx");
                },
                LOpType::Less => {
                    file.title("<");
                    file.code("pop rbx");
                    file.code("pop rax");
                    file.code("xor rcx, rcx");
                    file.code("cmp rax, rbx");
                    file.code("setl cl");
                    file.code("movzx rcx, cl");
                    file.code("push rcx");
                },
                LOpType::GreaterEqual => {
                    file.title(">=");
                    file.code("pop rbx");
                    file.code("pop rax");
                    file.code("xor rcx, rcx");
                    file.code("cmp rax, rbx");
                    file.code("setge cl");
                    file.code("movzx rcx, cl");
                    file.code("push rcx");
                },
                LOpType::LessEqual => {
                    file.title("<=");
                    file.code("pop rbx");
                    file.code("pop rax");
                    file.code("xor rcx, rcx");
                    file.code("cmp rax, rbx");
                    file.code("setle cl");
                    file.code("movzx rcx, cl");
                    file.code("push rcx");
                },
                LOpType::Equal => {
                    file.title("=");
                    file.code("pop rbx");
                    file.code("pop rax");
                    file.code("xor rcx, rcx");
                    file.code("cmp rax, rbx");
                    file.code("sete cl");
                    file.code("movzx rcx, cl");
                    file.code("push rcx");
                },
                LOpType::NotEqual => {
                    file.title("!=");
                    file.code("pop rbx");
                    file.code("pop rax");
                    file.code("xor rcx, rcx");
                    file.code("cmp rax, rbx");
                    file.code("setne cl");
                    file.code("movzx rcx, cl");
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
                LOpType::Mem => {
                    file.title("mem u64");
                    file.code("lea rax, [rel membuf]");
                    file.code("push rax");
                },
                LOpType::Load => {
                    file.title("load");
                    file.code("xor rcx, rcx");
                    file.code("pop rax");
                    file.code("mov cl, [rax]");
                    file.code("push rcx");
                },
                LOpType::Store => {
                    /*
                        value address
                     */
                    file.title("store");
                    file.code("pop rax");
                    file.code("pop rcx");
                    file.code("mov [rax], cl");
                },
                LOpType::Puts(nl) => {
                    /*
                        address count
                     */
                    file.title("puts");
                    file.lbl(1);
                    file.code("xor rcx, rcx");
                    file.code("mov rsi, [rsp+8]");
                    file.code("lea rbx, [rsi]");
                    file.code("mov cl, [rbx]");
                    file.code("call puts");
                    file.code("sub qword [rsp], 1");
                    file.code("add qword [rsp+8], 1");
                    file.code("mov rbx, [rsp]");
                    file.code("test rbx, rbx");
                    file.code("jg .L1");
                    if nl {
                        file.code("mov cl, 10");
                        file.code("call puts");
                    }
                    file.code("add rsp, 16");
                }
                _ => {
                    println!("Not implemented! {:?}", value);
                    return false;
                }
            }

            ptr += 1;
        }

        file.addr(csize);
        file.write("segment .data\n");

        for (idx, text) in strs.iter().enumerate() {
            file.title(format!("str lit {} \"{}\":{}", idx, text, text.len()).as_str());
            
            let data = text.bytes().map(|x| x.to_string()).collect::<Vec<String>>().join(", ");
            file.write(format!("str_{}:\n    db {}\n", idx, data).as_str());
        }

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
