//! Compiler that transforms Transient-C into TransientIR. (TIR)
//! Currently under development.


/*
    0x01: MOV byte from source1 into destination
    0x02: ADD source1 and source2 and store result in destination
    0x03: SUB source2 from source1 and store result in destination
    0x04: MUL source1 and source2 and store result in destination
    0x05: DIV source1 by source2 and store result in destination (truncated)
    0x06: DIV source1 by source2 and store result in destination (rounded)
    0x07: REM divides source1 by source2 and stores the remainder in destination
    0x08: CGT compare if source1 is greater than source2, and if so, store 1 in destination
    0x09: CLT compare if source1 is less than source2, and if so, store 1 in destination
    0x0A: JMP stops current execution and jumps to code in source1
    0x0B: JIE stops current execution and jumps to code in source1 ONLY IF source2 is non-zero
    0x0C: JNE stops current execution and jumps to code in source1 ONLY IF source2 is zero
    0x0D: PUT prints data at source1 to the screen (int)
    0x0E: PUT prints data at source1 to the screen (char)
    0x0F: IMZ gets the image size that was loaded to ROM and stores it in destination
    0x10: EQU compare if source1 and source2 are equal, and if so, store 1 in destination
    0xFF: HLT halts execution and stops processor
*/


use std::env::args;
use std::io::{Read, Write};
use std::fs::File;
use std::collections::HashMap;

enum Operation {
    Mov(usize, usize, usize),
    Add(usize, usize, usize, usize),
    Sub(usize, usize, usize, usize),
    Mul(usize, usize, usize, usize),
    DivT(usize, usize, usize, usize),
    DivR(usize, usize, usize, usize),
    Rem(usize, usize, usize, usize),
    Cgt(usize, usize, usize, usize),
    Clt(usize, usize, usize, usize),
    Jmp(usize),
    Jie(usize, usize, usize),
    Jne(usize, usize, usize),
    PutI(usize, usize),
    PutC(usize, usize),
    Imz(usize, usize),
    Equ(usize, usize, usize, usize,),
    Hlt(),
}



fn preprocess_source_code(source_code: Vec<String>) -> Vec<Operation> {
    // Pass 1
    // Count IR LoC
    let mut lines_of_ir = 0u64;
    for line in &source_code {
        // Check if it's actual IR
        if !line.is_empty() && !line.starts_with("#") && !line.starts_with("//") && !line.starts_with("set") {
            lines_of_ir += 1;
        }
    }
    let ir_size_bytes = lines_of_ir * 8;

    // Pass 2
    // Build hashmap of variables
    let memory_map: HashMap<String, (usize, u64)> = HashMap::new();
    for line in &source_code {
        if !line.starts_with("set") {
            continue;
        }
        let line_tokens: Vec<String> = line.split(" ").map(|x| {x.to_owned()}).collect();
        assert!(
            line_tokens.len() != 3,
            "Invalid set syntax"
        );
        assert!(
            line_tokens[1].starts_with("$"),
            "Invalid variable"
        );
        

    }


    todo!();
}
    
fn main() {
    // Verify input parameters
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        println!("Stop: Incorrect amount of arguments!");
        return;
    }

    // Open file for reading
    let mut input_file = match File::open(&args[1]) {
        Ok(x) => x,
        Err(_) => {
            panic!("Stop: Failed to open file");
        }
    };

    // Read bytes into buffer
    let mut source_code: String = String::new();
    if let Err(_) = input_file.read_to_string(&mut source_code) {
        panic!("Stop: Failed to read file contents");
    }
    println!("Info: File read");

    let source_code: Vec<String> = source_code.split("\n").map(|x| {x.to_owned()}).collect();

    let abstract_syntax_tree = preprocess_source_code(source_code);
}
