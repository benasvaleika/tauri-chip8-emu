pub struct CPU {
    ram: [u8; 4096],
    vx: [u8; 16],
    stack: [u16; 16],
    pc: u16,
    i: u16,
    sp: u8,
}

impl CPU {
    pub fn new() {
        // TODO: Load Font into ram

        CPU {
            ram: [0; 4096],
            vx: [0; 16],
            stack: [0; 16],
            pc: 0x200,
            i: 0,
            sp: 0,
        }
    }

    fn read_opcode(&self) -> u16 {
        let op_byte1 = self.ram[pc] as u16;
        let op_byte2 = self.ram[pc] as u16;

        // Combine two bytes to u16 opcode.
        op_byte1 << 8 | op_byte2
    }

    fn load_rom(&mut self, data: &[u8]) {
        for (i, &byte) in &data.iter().enumerate() {
            let curr_addr = 0x200 + i;
            if (curr_addr < 4096) {
                self.ram[curr_addr] = byte;
            } else {
                break;
            }
        }
    }
}
