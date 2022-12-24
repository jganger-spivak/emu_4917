use log::warn;

pub fn assemble(file: &str) -> Vec<u8> {
    let mut result = Vec::new();
    let lines = file.split("\n");
    let mut line_num = 0;
    for line in lines {
        line_num += 1;
        if line.contains("DB ") {
            let val:u8 = line.replace("DB ", "").parse().unwrap();
            result.push(val);
            continue;
        }
        if line.contains("HLT") {
            result.push(0);
            continue;
        }
        if line.contains("ADD") {
            result.push(1);
            continue;
        }
        if line.contains("SUB") {
            result.push(2);
            continue;
        }
        if line.contains("INC R0") {
            result.push(3);
            continue;
        }
        if line.contains("INC R1") {
            result.push(4);
            continue;
        }
        if line.contains("DEC R0") {
            result.push(5);
            continue;
        }
        if line.contains("DEC R1") {
            result.push(6);
            continue;
        }
        if line.contains("BEL") {
            result.push(7);
            continue;
        }
        if line.contains("PRN") {
            result.push(8);
            continue;
        }
        if line.contains("LR0 ") {
            let val:u8 = line.replace("LR0 ", "").parse().unwrap();
            result.push(9);
            result.push(val);
            continue;
        }
        if line.contains("LR1 ") {
            let val:u8 = line.replace("LR1 ", "").parse().unwrap();
            result.push(10);
            result.push(val);
            continue;
        }
        if line.contains("SR0 ") {
            let val:u8 = line.replace("SR0 ", "").parse().unwrap();
            result.push(11);
            result.push(val);
            continue;
        }
        if line.contains("SR1 ") {
            let val:u8 = line.replace("SR1 ", "").parse().unwrap();
            result.push(12);
            result.push(val);
            continue;
        }
        if line.contains("JMP ") {
            let val:u8 = line.replace("JMP ", "").parse().unwrap();
            result.push(13);
            result.push(val);
            continue;
        }
        if line.contains("JMZ ") {
            let val:u8 = line.replace("JMZ ", "").parse().unwrap();
            result.push(14);
            result.push(val);
            continue;
        }
        if line.contains("JNZ ") {
            let val:u8 = line.replace("JNZ ", "").parse().unwrap();
            result.push(15);
            result.push(val);
            continue;
        }
        warn!("Unrecognized operand at line {}", line_num);
    }
    return result;
}
