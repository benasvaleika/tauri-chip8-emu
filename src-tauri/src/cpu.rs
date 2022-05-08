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

        self.execute_opcode(opcode);
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

    fn execute_opcode(&self, opcode: u16) {
        let inst = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let var = (opcode & 0x000F) as u8;

        let nnn = (opcode & 0x0FFF) as usize;
        let nn = (opcode & 0x00FF) as u8;
        let n = (opcode & 0x000F) as usize;

        match (inst, x, y, var) {
            _ => println!("opcode {:04x} not implemented", opcode),
        }
    }
}
