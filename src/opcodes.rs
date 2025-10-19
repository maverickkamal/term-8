use crate::emulator::Emulator;
use rand::Rng;

pub fn execute(emu: &mut Emulator, opcode: u16) {
    let x = ((opcode & 0x0F00) >> 8) as usize;
    let y = ((opcode & 0x00F0) >> 4) as usize;
    let n = (opcode & 0x000F) as u8;
    let nn = (opcode & 0x00FF) as u8;
    let nnn = opcode & 0x0FFF;

    match opcode & 0xF000 {
        0x0000 => match opcode {
            0x00E0 => {
                emu.display = [false; 64 * 32];
                emu.pc += 2;
            }
            0x00EE => {
                emu.sp -= 1;
                emu.pc = emu.stack[emu.sp as usize];
                emu.pc += 2;
            }
            _ => {
                emu.pc += 2;
            }
        },
        0x1000 => {
            emu.pc = nnn;
        }
        0x2000 => {
            emu.stack[emu.sp as usize] = emu.pc;
            emu.sp += 1;
            emu.pc = nnn;
        }
        0x3000 => {
            if emu.v[x] == nn {
                emu.pc += 4;
            } else {
                emu.pc += 2;
            }
        }
        0x4000 => {
            if emu.v[x] != nn {
                emu.pc += 4;
            } else {
                emu.pc += 2;
            }
        }
        0x5000 => {
            if emu.v[x] == emu.v[y] {
                emu.pc += 4;
            } else {
                emu.pc += 2;
            }
        }
        0x6000 => {
            emu.v[x] = nn;
            emu.pc += 2;
        }
        0x7000 => {
            emu.v[x] = emu.v[x].wrapping_add(nn);
            emu.pc += 2;
        }
        0x8000 => {
            match n {
                0x0 => {
                    emu.v[x] = emu.v[y];
                    emu.pc += 2;
                }
                0x1 => {
                    emu.v[x] |= emu.v[y];
                    emu.pc += 2;
                }
                0x2 => {
                    emu.v[x] &= emu.v[y];
                    emu.pc += 2;
                }
                0x3 => {
                    emu.v[x] ^= emu.v[y];
                    emu.pc += 2;
                }
                0x4 => {
                    let (result, overflow) = emu.v[x].overflowing_add(emu.v[y]);
                    emu.v[x] = result;
                    emu.v[0xF] = if overflow { 1 } else { 0 };
                    emu.pc += 2;
                }
                0x5 => {
                    let (result, overflow) = emu.v[x].overflowing_sub(emu.v[y]);
                    emu.v[x] = result;
                    emu.v[0xF] = if overflow { 0 } else { 1 };
                    emu.pc += 2;
                }
                0x6 => {
                    emu.v[0xF] = emu.v[x] & 0x1;
                    emu.v[x] >>= 1;
                    emu.pc += 2;
                }
                0x7 => {
                    let (result, overflow) = emu.v[y].overflowing_sub(emu.v[x]);
                    emu.v[x] = result;
                    emu.v[0xF] = if overflow { 0 } else { 1 };
                    emu.pc += 2;
                }
                0xE => {
                    emu.v[0xF] = (emu.v[x] & 0x80) >> 7;
                    emu.v[x] <<= 1;
                    emu.pc += 2;
                }
                _ => {
                    emu.pc += 2;
                }
            }
        }
        0x9000 => {
            if emu.v[x] != emu.v[y] {
                emu.pc += 4;
            } else {
                emu.pc += 2;
            }
        }
        0xA000 => {
            emu.i = nnn;
            emu.pc += 2;
        }
        0xB000 => {
            emu.pc = nnn + emu.v[0] as u16;
        }
        0xC000 => {
            let random: u8 = rand::thread_rng().gen();
            emu.v[x] = random & nn;
            emu.pc += 2;
        }
        0xD000 => {
            let x_coord = emu.v[x] as usize % 64;
            let y_coord = emu.v[y] as usize % 32;
            emu.v[0xF] = 0;

            for row in 0..n as usize {
                if y_coord + row >= 32 {
                    break;
                }

                let sprite_byte = emu.memory[emu.i as usize + row];

                for col in 0..8 {
                    if x_coord + col >= 64 {
                        break;
                    }

                    let pixel = (sprite_byte >> (7 - col)) & 1;
                    if pixel == 1 {
                        let idx = (y_coord + row) * 64 + (x_coord + col);
                        if emu.display[idx] {
                            emu.v[0xF] = 1; 
                        }
                        emu.display[idx] ^= true;
                    }
                }
            }

            emu.pc += 2;
        }
        0xE000 => match nn {
            0x9E => {
                if emu.keypad[emu.v[x] as usize] {
                    emu.pc += 4;
                } else {
                    emu.pc += 2;
                }
            }
            0xA1 => {
                if !emu.keypad[emu.v[x] as usize] {
                    emu.pc += 4;
                } else {
                    emu.pc += 2;
                }
            }
            _ => {
                emu.pc += 2;
            }
        },
        0xF000 => match nn {
            0x07 => {
                emu.v[x] = emu.delay_timer;
                emu.pc += 2;
            }
            0x0A => {
                let mut key_pressed = None;
                for (i, &pressed) in emu.keypad.iter().enumerate() {
                    if pressed {
                        key_pressed = Some(i as u8);
                        break;
                    }
                }

                if let Some(key) = key_pressed {
                    emu.v[x] = key;
                    emu.waiting_for_key = None;
                    emu.pc += 2;
                } else {
                    emu.waiting_for_key = Some(x as u8);
                }
            }
            0x15 => {
                emu.delay_timer = emu.v[x];
                emu.pc += 2;
            }
            0x18 => {
                emu.sound_timer = emu.v[x];
                emu.pc += 2;
            }
            0x1E => {
                emu.i = emu.i.wrapping_add(emu.v[x] as u16);
                emu.pc += 2;
            }
            0x29 => {
                emu.i = (emu.v[x] as u16 & 0x0F) * 5;
                emu.pc += 2;
            }
            0x33 => {
                let value = emu.v[x];
                emu.memory[emu.i as usize] = value / 100;
                emu.memory[emu.i as usize + 1] = (value / 10) % 10;
                emu.memory[emu.i as usize + 2] = value % 10;
                emu.pc += 2;
            }
            0x55 => {
                for i in 0..=x {
                    emu.memory[emu.i as usize + i] = emu.v[i];
                }
                emu.pc += 2;
            }
            0x65 => {
                for i in 0..=x {
                    emu.v[i] = emu.memory[emu.i as usize + i];
                }
                emu.pc += 2;
            }
            _ => {
                emu.pc += 2;
            }
        },
        _ => {
            emu.pc += 2;
        }
    }
}


