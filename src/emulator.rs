const FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, 
    0x20, 0x60, 0x20, 0x20, 0x70, 
    0xF0, 0x10, 0xF0, 0x80, 0xF0, 
    0xF0, 0x10, 0xF0, 0x10, 0xF0, 
    0x90, 0x90, 0xF0, 0x10, 0x10, 
    0xF0, 0x80, 0xF0, 0x10, 0xF0, 
    0xF0, 0x80, 0xF0, 0x90, 0xF0, 
    0xF0, 0x10, 0x20, 0x40, 0x40, 
    0xF0, 0x90, 0xF0, 0x90, 0xF0, 
    0xF0, 0x90, 0xF0, 0x10, 0xF0, 
    0xF0, 0x90, 0xF0, 0x90, 0x90, 
    0xE0, 0x90, 0xE0, 0x90, 0xE0, 
    0xF0, 0x80, 0x80, 0x80, 0xF0, 
    0xE0, 0x90, 0x90, 0x90, 0xE0, 
    0xF0, 0x80, 0xF0, 0x80, 0xF0, 
    0xF0, 0x80, 0xF0, 0x80, 0x80, 
];

#[derive(Clone)]
pub struct Emulator {
   
    pub memory: [u8; 4096],


    pub v: [u8; 16],
    
    pub i: u16,
    
    pub pc: u16,
    
    pub stack: [u16; 16],
    
    pub sp: u8,
    
    pub delay_timer: u8,
    pub sound_timer: u8,
    
    pub keypad: [bool; 16],
    
    pub display: [bool; 64 * 32],
    
    pub paused: bool,
    
    pub waiting_for_key: Option<u8>,
    
    pub checkpoint: Option<Box<Emulator>>,
    
    pub ghost_display: [u8; 64 * 32],
}

impl Emulator {
    pub fn new() -> Self {
        let mut emulator = Self {
            memory: [0; 4096],
            v: [0; 16],
            i: 0,
            pc: 0x200, 
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            keypad: [false; 16],
            display: [false; 64 * 32],
            paused: false,
            waiting_for_key: None,
            checkpoint: None,
            ghost_display: [0; 64 * 32],
        };

        emulator.memory[0..80].copy_from_slice(&FONT_SET);

        emulator
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        let start = 0x200;
        let end = start + data.len().min(0xE00); 
        self.memory[start..end].copy_from_slice(&data[..end - start]);
    }

    pub fn fetch(&self) -> u16 {
        let pc = self.pc as usize;
        (self.memory[pc] as u16) << 8 | self.memory[pc + 1] as u16
    }

    pub fn cycle(&mut self) {
        if self.waiting_for_key.is_some() {
            return;
        }

        let opcode = self.fetch();

        crate::opcodes::execute(self, opcode);
    }

    pub fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    pub fn update_ghost(&mut self) {
        for i in 0..self.ghost_display.len() {
            if self.display[i] {
                self.ghost_display[i] = 255; // Full brightness
            } else if self.ghost_display[i] > 0 {
                self.ghost_display[i] = self.ghost_display[i].saturating_sub(25); // Decay
            }
        }
    }

    pub fn save_checkpoint(&mut self) {
        let mut clone = self.clone();
        clone.checkpoint = None;
        self.checkpoint = Some(Box::new(clone));
    }

    pub fn load_checkpoint(&mut self) {
        if let Some(checkpoint) = &self.checkpoint {
            let saved_checkpoint = self.checkpoint.clone();
            *self = (**checkpoint).clone();
            self.checkpoint = saved_checkpoint;
        }
    }

    pub fn reset(&mut self) {
        let rom_data: Vec<u8> = self.memory[0x200..].to_vec();
        *self = Self::new();
        self.memory[0x200..].copy_from_slice(&rom_data);
    }
}


