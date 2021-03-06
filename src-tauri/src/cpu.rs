use crate::chip8_font::CHIP8_FONT_SET;
use rand::Rng;

static load_quirk: bool = true;
static shift_quirk: bool = true;

pub struct CPU {
    // pub for debuging
    pub ram: [u8; 4096],
    // pub for debuging
    pub vx: [u8; 16],
    stack: [usize; 16],
    pc: usize,
    i: usize,
    sp: usize,
    pub display: [u8; 2048],
    pub display_changed: bool,
    pub keys: [bool; 16],
    delayt: u8,
    soundt: u8,
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
            display: [0; 2048],
            display_changed: false,
            keys: [false; 16],
            delayt: 0,
            soundt: 0,
        }
    }

    pub fn emulate_cycle(&mut self, keys: [bool; 16]) {
        self.keys = keys;

        let opcode = self.read_opcode();

        // println!("{:0x?}", &opcode);
        // println!("{:0x?}", self.pc);

        self.display_changed = false;
        self.execute_opcode(opcode);

        if self.delayt > 0 {
            self.delayt -= 1;
        }
        if self.soundt > 0 {
            self.soundt -= 1;
        }
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

    fn execute_opcode(&mut self, opcode: u16) {
        let inst = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let var = (opcode & 0x000F) as u8;

        let nnn = (opcode & 0x0FFF) as usize;
        let nn = (opcode & 0x00FF) as u8;
        let n = (opcode & 0x000F) as usize;

        match (inst, x, y, var) {
            (0x0, 0x0, 0xE, 0x0) => self.op_00E0(),
            (0x0, 0x0, 0xE, 0xE) => self.op_00EE(),
            (0x1, _, _, _) => self.op_1NNN(nnn),
            (0x2, _, _, _) => self.op_2NNN(nnn),
            (0x3, _, _, _) => self.op_3XNN(x, nn),
            (0x4, _, _, _) => self.op_4XNN(x, nn),
            (0x5, _, _, 0x0) => self.op_5XY0(x, y),
            (0x6, _, _, _) => self.op_6XNN(x, nn),
            (0x7, _, _, _) => self.op_7XNN(x, nn),
            (0x8, _, _, 0x0) => self.op_8XY0(x, y),
            (0x8, _, _, 0x1) => self.op_8XY1(x, y),
            (0x8, _, _, 0x2) => self.op_8XY2(x, y),
            (0x8, _, _, 0x3) => self.op_8XY3(x, y),
            (0x8, _, _, 0x4) => self.op_8XY4(x, y),
            (0x8, _, _, 0x5) => self.op_8XY5(x, y),
            (0x8, _, _, 0x6) => self.op_8XY6(x, y),
            (0x8, _, _, 0x7) => self.op_8XY7(x, y),
            (0x8, _, _, 0xE) => self.op_8XYE(x, y),
            (0x9, _, _, 0x0) => self.op_9XY0(x, y),
            (0xA, _, _, _) => self.op_ANNN(nnn),
            (0xB, _, _, _) => self.op_BNNN(nnn),
            (0xC, _, _, _) => self.op_CXNN(x, nn),
            (0xD, _, _, _) => self.op_DXYN(x, y, n),
            (0xE, _, 0x9, 0xE) => self.op_EX9E(x),
            (0xE, _, 0xA, 0x1) => self.op_EXA1(x),
            (0xF, _, 0x0, 0x7) => self.op_FX07(x),
            (0xF, _, 0x0, 0xA) => self.op_FX0A(x),
            (0xF, _, 0x1, 0x5) => self.op_FX15(x),
            (0xF, _, 0x1, 0x8) => self.op_FX18(x),
            (0xF, _, 0x1, 0xE) => self.op_FX1E(x),
            (0xF, _, 0x2, 0x9) => self.op_FX29(x),
            (0xF, _, 0x3, 0x3) => self.op_FX33(x),
            (0xF, _, 0x5, 0x5) => self.op_FX55(x),
            (0xF, _, 0x6, 0x5) => self.op_FX65(x),
            _ => println!("opcode {:04x} not implemented", opcode),
        }
    }

    // for debugging
    pub fn print_display(&self) {
        for y in 0..32 {
            println!("");
            for x in 0..64 {
                match self.display[y * 64 + x] {
                    0 => print!(" "),
                    1 => print!("#"),
                    _ => println!("Unexpected value in display"),
                }
            }
        }
    }

    // OPCODES

    // Clear the screen
    fn op_00E0(&mut self) {
        println!("00E0 Called");

        for i in 0..32 {
            for j in 0..64 {
                self.display[i * 64 + j] = 0;
            }
        }

        self.pc += 2;
    }

    // Return from subroutine
    fn op_00EE(&mut self) {
        println!("00EE Called");

        self.sp -= 1;
        self.pc = self.stack[self.sp];
    }

    // Jump to address NNN
    fn op_1NNN(&mut self, nnn: usize) {
        println!("1NNN Called");

        self.pc = nnn;
    }

    // Execute subroutine starting at address NNN
    fn op_2NNN(&mut self, nnn: usize) {
        println!("2NNN Called");

        if self.sp > self.stack.len() {
            panic!("Stack Overflow");
        } else {
            self.stack[self.sp] = self.pc + 2;
            self.sp += 1;
            self.pc = nnn;
        }
    }

    // Skip the following instruction if the value of VX equals NN
    fn op_3XNN(&mut self, x: u8, nn: u8) {
        println!("3XNN Called");

        if self.vx[x as usize] == nn {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    // Skip the following instruction if the value of VX is not equal to NN
    fn op_4XNN(&mut self, x: u8, nn: u8) {
        println!("4XNN Called");

        if self.vx[x as usize] != nn {
            self.pc += 4;
        } else {
            self.pc += 2
        }
    }

    // Skip the following instruction if the value of VX is equal to the value of VY
    fn op_5XY0(&mut self, x: u8, y: u8) {
        println!("5XY0 Called");

        if self.vx[x as usize] == self.vx[y as usize] {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    // Store number NN in VX
    fn op_6XNN(&mut self, x: u8, nn: u8) {
        println!("6XNN Called");

        self.vx[x as usize] = nn;
        self.pc += 2;
    }

    // Add the value NN to VX
    // Does not affect VF
    fn op_7XNN(&mut self, x: u8, nn: u8) {
        println!("7XNN Called");

        self.vx[x as usize] = self.vx[x as usize].wrapping_add(nn);

        self.pc += 2;
    }

    // Store the value of VY in VX
    fn op_8XY0(&mut self, x: u8, y: u8) {
        println!("8XY0 Called");

        self.vx[x as usize] = self.vx[y as usize];
        self.pc += 2;
    }

    // Set VX to VX OR VY
    fn op_8XY1(&mut self, x: u8, y: u8) {
        println!("8XY1 Called");

        self.vx[x as usize] |= self.vx[y as usize];
        self.pc += 2;
    }

    // Set VX to VX AND VY
    fn op_8XY2(&mut self, x: u8, y: u8) {
        println!("8XY2 Called");

        self.vx[x as usize] &= self.vx[y as usize];
        self.pc += 2;
    }

    // Set VX to VX XOR VY
    fn op_8XY3(&mut self, x: u8, y: u8) {
        println!("8XY3 Called");

        self.vx[x as usize] ^= self.vx[y as usize];
        self.pc += 2;
    }

    // Add the value of VY to VX, sets VF to 1 if a carry occurs,
    // sets VF to 00 if a carry does not occur
    fn op_8XY4(&mut self, x: u8, y: u8) {
        println!("8XY4 Called");

        let (val, overflow) = self.vx[x as usize].overflowing_add(self.vx[y as usize]);

        self.vx[x as usize] = val;

        if overflow {
            self.vx[0xF] = 1;
        } else {
            self.vx[0xF] = 0;
        }

        self.pc += 2;
    }

    // Substract the value of VY from VX, set VF to 0 if borrow occurs
    // set VF to 1 if the borrow doesn't occur
    fn op_8XY5(&mut self, x: u8, y: u8) {
        println!("8XY5 Called");

        let (val, borrow) = self.vx[x as usize].overflowing_sub(self.vx[y as usize]);

        self.vx[x as usize] = val;

        if borrow {
            self.vx[0xF] = 0;
        } else {
            self.vx[0xF] = 1;
        }

        self.pc += 2;
    }

    // 	Stores the least significant bit of VX in VF and then shifts VX to the right by 1 bit
    fn op_8XY6(&mut self, x: u8, mut y: u8) {
        println!("8XY6 Called");

        if shift_quirk {
            y = x;
        }

        self.vx[0xF] = self.vx[y as usize] & 0x01;
        self.vx[x as usize] = self.vx[y as usize] >> 1;

        self.pc += 2;
    }

    // Substract the value of VX from VY, store the value in VX.
    // set VF to 0 if the borrow occurs, to 1 otherwise.
    fn op_8XY7(&mut self, x: u8, y: u8) {
        println!("8XY7 Called");

        let (val, borrow) = self.vx[y as usize].overflowing_sub(self.vx[x as usize]);

        self.vx[x as usize] = val;

        if borrow {
            self.vx[0xF] = 0;
        } else {
            self.vx[0xF] = 1;
        }

        self.pc += 2;
    }

    // Stores the least significant bit of VX in VF and then shifts VX to the left by 1 bit
    fn op_8XYE(&mut self, x: u8,mut y: u8) {
        println!("8XYE Called");

        if shift_quirk {
            y = x
        }

        self.vx[0xF] = (self.vx[y as usize] >> 7) & 0x01;
        self.vx[x as usize] = self.vx[y as usize] << 1;

        self.pc += 2;
    }

    // Skip the following instruction if the value of register VX
    // is not equal to the value of register VY.
    fn op_9XY0(&mut self, x: u8, y: u8) {
        println!("9XY0 Called");

        if self.vx[x as usize] != self.vx[y as usize] {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    // Store memory address NNN in register I
    fn op_ANNN(&mut self, nnn: usize) {
        println!("ANNN Called");

        self.i = nnn;
        self.pc += 2;
    }

    // Jump to address NNN + V0
    fn op_BNNN(&mut self, nnn: usize) {
        println!("BNNN Called");

        self.pc = nnn + self.vx[0x0] as usize;
    }

    // Set VX to a random number with mask of NN
    fn op_CXNN(&mut self, x: u8, nn: u8) {
        println!("CXNN Called");

        let mut random: u8 = rand::thread_rng().gen();
        println!("{}", &random);
        self.vx[x as usize] = random & nn;
        self.pc += 2;
    }

    // Draw a sprite at positions VX, VY with N bytes of sprite data starting
    // at the address stored in I. Set VF to 01 if any set pixels are changed
    // to unset, and 00 otherwise.
    fn op_DXYN(&mut self, x: u8, y: u8, n: usize) {
        println!("DXYN Called");

        self.vx[0xF] = 0;
        self.display_changed = true;

        for byte in 0..n {
            let sprite_byte = self.ram[self.i + byte] as u8;
            for bit in 0..8 {
                let mut x = (self.vx[x as usize] as usize + bit) as usize % 64;
                let mut y = (self.vx[y as usize] as usize + byte) as usize % 32;
                let bit_active = (sprite_byte >> (7 - bit)) & 1;
                self.vx[0xF] |= bit_active & self.display[y * 64 + x];
                self.display[y * 64 + x] ^= bit_active;
            }
        }

        self.pc += 2;
    }

    // Skip the following instruction if they key corresponding to the hex
    // value is pressed
    fn op_EX9E(&mut self, x: u8) {
        println!("EX9E Called");

        if self.keys[self.vx[x as usize] as usize] {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    // Skip the following instruction if they key corresponding to the hex
    // value is not pressed
    fn op_EXA1(&mut self, x: u8) {
        println!("EXA1 Called");

        if !self.keys[self.vx[x as usize] as usize] {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    // Store current value of the delay timer in VX
    fn op_FX07(&mut self, x: u8) {
        println!("FX07 Called");

        self.vx[x as usize] = self.delayt;
        self.pc += 2;
    }

    // Wait for keypress and store the result in VX
    fn op_FX0A(&mut self, x: u8) {
        println!("FX0A Called");

        for i in 0..self.keys.len() {
            if self.keys[i] == true {
                self.vx[x as usize] = i as u8;
                self.pc += 2;
            }
        }
    }

    // Set the delay timer to the current value of register VX
    fn op_FX15(&mut self, x: u8) {
        println!("FX15 Called");

        self.delayt = self.vx[x as usize];
        self.pc += 2;
    }

    // Set the sound timet to the current value of register VX
    fn op_FX18(&mut self, x: u8) {
        println!("FX18 Called");

        self.soundt = self.vx[x as usize];
        self.pc += 2;
    }

    // Add the value stored in VX to I
    fn op_FX1E(&mut self, x: u8) {
        println!("FX1E Called");

        self.i += self.vx[x as usize] as usize;
        self.vx[0x0f] = if self.i > 0x0F00 {1} else {0};
        self.pc += 2;
    }

    // Set I to the memory address of the sprite data corresponding
    // to the hex digit stored in VX
    fn op_FX29(&mut self, x: u8) {
        println!("FX29 Called");

        self.i =  0x50 + (self.vx[x as usize] as usize) * 5;
        self.pc += 2;
    }

    // Store binary-coded decimal representation of the value stored in VX
    // at memory addresses I, I+1, I+2
    fn op_FX33(&mut self, x: u8) {
        println!("FX33 Called");

        self.ram[self.i] = self.vx[x as usize] / 100; // Hundreds
        self.ram[self.i + 1] = (self.vx[x as usize] % 100) / 10; // Tens
        self.ram[self.i + 2] = self.vx[x as usize] % 10; // Ones

        self.pc += 2;
    }

    // Store the values of V0 to VX inclusive in memory starting at address I.
    // I is set to I+x+1 after operation
    fn op_FX55(&mut self, x: u8) {
        println!("FX55 Called");
        
        for i in 0..x + 1 {
            self.ram[self.i + i as usize] = self.vx[i as usize];
        }

        if load_quirk {
            self.i += x as usize + 1
        }

        self.pc += 2;
    }

    // Fill V0 to VX inclusive with the values stored in memory starting at I.
    // I is set to I+x+1 after operation
    fn op_FX65(&mut self, x: u8) {
        println!("FX65 Called");

        for i in 0..x + 1 {
            self.vx[i as usize] = self.ram[self.i + i as usize];
        }

        if load_quirk {
            self.i += x as usize + 1
        }

        self.pc += 2;
    }
}
