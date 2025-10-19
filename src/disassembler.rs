pub fn disassemble(opcode: u16) -> String {
    let x = (opcode & 0x0F00) >> 8;
    let y = (opcode & 0x00F0) >> 4;
    let n = opcode & 0x000F;
    let nn = opcode & 0x00FF;
    let nnn = opcode & 0x0FFF;

    match opcode & 0xF000 {
        0x0000 => match opcode {
            0x00E0 => "CLS".to_string(),
            0x00EE => "RET".to_string(),
            _ => format!("SYS  {:03X}", nnn),
        },
        0x1000 => format!("JP   {:03X}", nnn),
        0x2000 => format!("CALL {:03X}", nnn),
        0x3000 => format!("SE   V{:X}, {:02X}", x, nn),
        0x4000 => format!("SNE  V{:X}, {:02X}", x, nn),
        0x5000 => format!("SE   V{:X}, V{:X}", x, y),
        0x6000 => format!("LD   V{:X}, {:02X}", x, nn),
        0x7000 => format!("ADD  V{:X}, {:02X}", x, nn),
        0x8000 => match n {
            0x0 => format!("LD   V{:X}, V{:X}", x, y),
            0x1 => format!("OR   V{:X}, V{:X}", x, y),
            0x2 => format!("AND  V{:X}, V{:X}", x, y),
            0x3 => format!("XOR  V{:X}, V{:X}", x, y),
            0x4 => format!("ADD  V{:X}, V{:X}", x, y),
            0x5 => format!("SUB  V{:X}, V{:X}", x, y),
            0x6 => format!("SHR  V{:X}", x),
            0x7 => format!("SUBN V{:X}, V{:X}", x, y),
            0xE => format!("SHL  V{:X}", x),
            _ => format!("DATA {:04X}", opcode),
        },
        0x9000 => format!("SNE  V{:X}, V{:X}", x, y),
        0xA000 => format!("LD   I, {:03X}", nnn),
        0xB000 => format!("JP   V0, {:03X}", nnn),
        0xC000 => format!("RND  V{:X}, {:02X}", x, nn),
        0xD000 => format!("DRW  V{:X}, V{:X}, {:X}", x, y, n),
        0xE000 => match nn {
            0x9E => format!("SKP  V{:X}", x),
            0xA1 => format!("SKNP V{:X}", x),
            _ => format!("DATA {:04X}", opcode),
        },
        0xF000 => match nn {
            0x07 => format!("LD   V{:X}, DT", x),
            0x0A => format!("LD   V{:X}, K", x),
            0x15 => format!("LD   DT, V{:X}", x),
            0x18 => format!("LD   ST, V{:X}", x),
            0x1E => format!("ADD  I, V{:X}", x),
            0x29 => format!("LD   F, V{:X}", x),
            0x33 => format!("LD   B, V{:X}", x),
            0x55 => format!("LD   [I], V{:X}", x),
            0x65 => format!("LD   V{:X}, [I]", x),
            _ => format!("DATA {:04X}", opcode),
        },
        _ => format!("DATA {:04X}", opcode),
    }
}


