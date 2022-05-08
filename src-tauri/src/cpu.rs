use crate::chip8_font::CHIP8_FONT_SET;

pub struct CPU {
    // pub for debuging
    pub ram: [u8; 4096],
    // pub for debuging
    pub vx: [u8; 16],
    stack: [u16; 16],
    pc: usize,
    i: u16,
    sp: u8,
}

impl CPU {
    pub fn new() -> CPU {
        // Load font into ram
        let mut ram_w_font = [0; 4096];
        for (i, &byte) in CHIP8_FONT_SET.iter().enumerate() {
            let curr_addr = 0x50 + i;
            ram_w_font[curr_addr] = byte;
        }

        CPU {
            ram: ram_w_font,
            vx: [0; 16],
            stack: [0; 16],
            pc: 0x200,
            i: 0,
            sp: 0,
        }
    }

    pub fn emulate_cycle(&mut self) {
        let opcode = self.read_opcode();

        self.pc += 2;

        println!("{:0x?}", &opcode);
    }

    fn read_opcode(&self) -> u16 {
        let op_byte1 = self.ram[self.pc] as u16;
        let op_byte2 = self.ram[self.pc + 1] as u16;

        // Combine two bytes to u16 opcode.
        op_byte1 << 8 | op_byte2
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        for (i, &byte) in data.iter().enumerate() {
            let curr_addr = 0x200 + i;
            if curr_addr < 4096 {
                self.ram[curr_addr] = byte;
            } else {
                break;
            }
        }
    }
}
