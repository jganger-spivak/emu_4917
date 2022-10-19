fn main() {
    println!("Hello, world!");
    let mut mem:[i8; 16] = [0; 16];
    let mut halt = false;
    let mut ip:usize = 0;
    let mut r0:i8 = 0;
    let mut r1:i8 = 0;
    println!("Memory: {:?}", mem);
    while !halt {
        execute(&mut halt, &mut ip, &mut r0, &mut r1, &mut mem);
    }
}

fn execute(halt:&mut bool, ip:&mut usize, r0:&mut i8, r1:&mut i8, mem:&mut [i8;16]) {
    let x = mem[*ip+1];
    if mem[*ip] == 0 {
        println!("HALT! ");
        *halt = true;
    }
    if mem[*ip] == 1 {
        *r0 += *r1;
    }
    if mem[*ip] == 2 {
        *r0 -= *r1;
    }
    if mem[*ip] == 3 {
        *r0 += 1;
    }
    if mem[*ip] == 4 {
        *r1 += 1;
    }
    if mem[*ip] == 5 {
        *r0 -= 1;
    }
    if mem[*ip] == 6 {
        *r1 -= 1;
    }
    if mem[*ip] == 7 {
        println!("BEEP! ");
    }
    if mem[*ip] == 8 {
        println!("{}", x);
    }
    if mem[*ip] == 9 {
        *r0 = x;
    }
    if mem[*ip] == 10 {
        *r1 = x;
    }
    if mem[*ip] == 11 {
        mem[*ip+1] = *r0;
    }
    if mem[*ip] == 12 {
        mem[*ip+1] = *r1;
    }
    if mem[*ip] == 13 {
        
    }
    if mem[*ip] == 14 {
        
    }
    if mem[*ip] == 15 {
        
    }
}