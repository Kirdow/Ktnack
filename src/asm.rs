use std::io;
use std::io::{Write, ErrorKind};
use std::fs::File;
use std::process::Command;

pub struct AsmFile {
    name: String,
    filename: String,
    handle: File,
}

impl AsmFile {
    fn new(name: &str) -> Self {
        let filename = format!("{}.asm", name); 
        let output = File::create(filename.clone());
        let mut output = match output {
            Ok(file) => file,
            Err(error) => {
                panic!("Problem creating ASM file : {:?}", error);
            }
        };

        let mut result = Self {
            name: name.to_string(),
            filename: filename,
            handle: output,
        };

        result.write("BITS 64\n");
        result.write("global main\n");
        result.write("extern printf\n");
        result.write("segment .data\n");
        result.write("    fmt     db \"%ld\", 10, 0\n");
        result.write("    putc    db 0, 0\n");
        result.write("    putcf   db \"%s\", 0\n");
        result.write("segment .bss\n");
        result.write("    membuf  resb 640 * 1024\n");
        result.write("segment .text\n");
        result.write("log:\n");
        result.write("    sub     rsp, 32\n");
        result.write("    mov     rdx, rcx\n");
        result.write("    lea     rcx, [rel fmt]\n");
        result.write("    call    printf\n");
        result.write("    add     rsp, 32\n");
        result.write("    ret\n");
        result.write("puts:\n");
        result.write("    sub     rsp, 32\n");
        result.write("    mov     [rel putc], cl\n");
        result.write("    lea     rdx, [rel putc]\n");
        result.write("    lea     rcx, [rel putcf]\n");
        result.write("    call    printf\n");
        result.write("    add     rsp, 32\n");
        result.write("    ret\n");
        result.write("main:\n");

        result
    }

    pub fn close(&mut self) -> String {
        self.handle.flush().expect("Failed to flush ASM file!");
        
        self.name.clone()
    }

    pub fn write(&mut self, text: &str) {
        write!(self.handle, "{}", text).expect("Failed to write to file!");
    }

    pub fn code(&mut self, text: &str) {
        self.write(format!("    {}\n", text).as_str());
    }

    pub fn title(&mut self, text: &str) {
        self.write(format!("    ;; -- {} --\n", text).as_str());
    }

    pub fn addr(&mut self, ip: u64) {
        self.write(format!("addr_{}:\n", ip).as_str());
    }

    pub fn lbl(&mut self, i: u64) {
        self.write(format!(".L{}:\n", i).as_str());
    }
}

pub fn pre_compile(name: &str) -> AsmFile {
    println!("Generating ASM from Ktnack code...");
    return AsmFile::new(name);
}

pub fn post_compile(mut file: AsmFile) -> bool {
    file.write("addr_eof:\n");
    file.write("    ret\n");

    let name = file.name.to_string();
    let file = file.close();

    println!("name:{} file:{}", name.as_str(), file.as_str());

    println!("Building ASM...");
    let status = Command::new("nasm")
                                .args(&["-f", "win64", "-o", format!("{}.obj", name).as_str(), format!("{}.asm", name).as_str()])
                                .status()
                                .expect("Failed to assemble code!");

    if !status.success() {
        println!("Failed to build ASM!");
        return false;
    }

    println!("Linking program...");

    let status = Command::new("link")
                                .args(&[format!("{}.obj", name).as_str(), "/subsystem:console", "kernel32.lib", "msvcrt.lib", "legacy_stdio_definitions.lib", format!("/out:{}.exe", name).as_str()])
                                .status()
                                .expect("Failed to link code!");

    if !status.success() {
        println!("Failed to link program!");
        return false;
    }

    println!("Compilation successful!");
    println!("Binary located as: {}.exe", name);

    return true;    
}