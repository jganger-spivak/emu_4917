use std::fs;
use std::env;

use log::{info, trace, warn};

const INT_MAX:u8 = 15;
const MEM_MAX:usize = 16;
const DEBUG_MODE:bool = false;
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut mem:[u8; MEM_MAX] = [0; MEM_MAX];
    let mut halt = false;
    let mut ip:usize = 0;
    let mut r0:u8 = 0;
    let mut r1:u8 = 0;
    
    
    
    load_from_file(&mut mem, args.get(1).unwrap_or(&"program.txt".to_string()));
    
    println!("Memory: {:?}", mem);
    
    while !halt {
        if DEBUG_MODE {
            println!("IP: {}, executing instruction {}", ip, mem[ip]);
        }
        execute(&mut halt, &mut ip, &mut r0, &mut r1, &mut mem);
        if DEBUG_MODE {
            println!("R0: {} R1: {}", r0, r1);
            println!("Memory: {:?}", mem);
        }
    }
    println!("R0: {} R1: {}", r0, r1);
    println!("Memory: {:?}", mem);
}

fn load_from_file(mem: &mut[u8], filename: &String) {
    let result = fs::read(filename);
    let code;
    match result {
        Ok(data) => { code = String::from_utf8_lossy(&data).parse::<String>().unwrap();}
        Err(err) => { panic!("Couldn't read file: {}", err.to_string())}
    }
    let mut line_num = 0;
    for line in code.split("\n") {
        let mut line_code:[u8; 4] = [0; 4];
        let mut i:usize = 0;
        for val in line.split(",") {
            line_code[i] = val.parse().unwrap();
            i += 1;
        }
        println!("line_code: {:?}", line_code);
        load(mem, line_num, line_code);
        line_num += 1;
    }
    println!("{:?}", code);
}

fn load(mem:&mut [u8], line:usize, line_code:[u8; 4]) {
    let offset = line * 4;
    mem[0+offset] = line_code[0];
    mem[1+offset] = line_code[1];
    mem[2+offset] = line_code[2];
    mem[3+offset] = line_code[3];
}

fn add_wrap(a:u8, b:u8) -> u8 {
    if a + b > INT_MAX { //overflow
        return 0;
    } else {
        return a + b;
    }
}

fn sub_wrap(a:u8, b:u8) -> u8 {
    if (a as i8 - b as i8) < 0 { //underflow
        return 15;
    } else {
        return a - b;
    }
}

fn execute(halt:&mut bool, ip:&mut usize, r0:&mut u8, r1:&mut u8, mem:&mut [u8;MEM_MAX]) {
    let ip_plus_one = add_wrap(*ip as u8, 1) as usize;
    let ip_plus_two = add_wrap(*ip as u8, 2) as usize;
    let x = mem[ip_plus_one];
    if DEBUG_MODE {
        println!("X: {}", x);
    }
    match mem[*ip] {
        0 => { //HLT
            println!("HALT! ");
            *halt = true;
        }
        1 => { //ADD R0, R1
            *r0 = add_wrap(*r0, *r1);
            *ip = ip_plus_one;
        }
        2 => { //SUB R0, R1
            *r0 = sub_wrap(*r0, *r1);
            *ip = ip_plus_one;
        }
        3 => { //INC R0
            *r0 = add_wrap(*r0, 1);
            *ip = ip_plus_one;
        }
        4 => { //INC R1
            *r1 = add_wrap(*r1, 1);
            *ip = ip_plus_one;
        }
        5 => { //DEC R0
            *r0 = sub_wrap(*r0, 1);
            *ip = ip_plus_one;
        }
        6 => { //DEC R1
            *r1 = sub_wrap(*r1, 1);
            *ip = ip_plus_one;
        }
        7 => { //BEL
            println!("BEEP! ");
            *ip = ip_plus_one;
        }
        8 => { //PRN X
            println!("{}", x);
            *ip = ip_plus_two;
        }
        9 => { //LDA R0, X
            *r0 = x;
            *ip = ip_plus_two;
        }
        10 => { //LDA R1, X
            *r1 = x;
            *ip = ip_plus_two;
        }
        11 => { //STO R0, X
            mem[ip_plus_one] = *r0;
            *ip = ip_plus_two;
        }
        12 => { //STO R1, X
            mem[ip_plus_one] = *r1;
            *ip = ip_plus_two;
        }
        13 => { //JMP X
            *ip = x as usize;
            return;
        }
        14 => { //JMZ X
            if *r0 == 0 {
                *ip = x as usize;
            } else {
                *ip = ip_plus_two
            }
            return;
        }
        15 => { //JNZ X
            if *r0 != 0 {
                *ip = x as usize;
            } else {
                *ip = ip_plus_two
            }
            return;
        }
        16_u8..=u8::MAX => { warn!("Non-implemented opcodes are being executed at IP: {}, check your jumps", *ip); *ip = ip_plus_one; }
    }
}