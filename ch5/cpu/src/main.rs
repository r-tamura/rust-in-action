
#[derive(Debug)]
enum Opcode {
    Add(u8, u8),
    Halt,
}

struct CPU {
    registers: [u8; 16],
    pc: usize,
    memory: [u8; 0x1000],
}

impl CPU {

    fn new() -> Self {
        CPU {
            registers: [0; 16],
            pc: 0,
            memory: [0; 0x1000],
        }
    }
    fn read_opcode(&self) -> u16 {
        let opbyte_h = self.memory[self.pc] as u16;
        let opbyte_l = self.memory[self.pc + 1] as u16;
        (opbyte_h << 8) | opbyte_l
    }

    fn decode(&self, opcode: u16) -> Opcode {
        let c = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let n = (opcode & 0x000F) as u8;

        let c = match (c, x, y, n) {
            (0x0, 0x0, 0x0, 0x0) => Opcode::Halt,
            (0x8, _, _, 0x4) => Opcode::Add(x, y),
            _ => todo!("opcode {:04x}", opcode),
        };

        c
    }

    fn run(&mut self) {
        loop {
            let opcode_b = self.read_opcode();
            self.pc += 2;
            // 4bitずつに分割
            let opcode = self.decode(opcode_b);
            match opcode {
                Opcode::Halt => return,
                Opcode::Add(x, y) => self.add_xy(x, y),
                _ => todo!("opcode {:04x}", opcode_b),
            }
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }
}

fn main() {
    let mut cpu = CPU::new();

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10;
    cpu.registers[3] = 10;

    let mem = &mut cpu.memory;
    mem[0] = 0x80; mem[1] = 0x14;
    mem[2] = 0x80; mem[3] = 0x24;
    mem[4] = 0x80; mem[5] = 0x34;

    cpu.run();

    assert_eq!(cpu.registers[0], 35);
    println!("5 + 10 + 10 + 10 = {}", cpu.registers[0]);
}
