
#[derive(Debug)]
enum Opcode {
    Add(u8, u8),
    Halt,
    Call(u16),
    Ret,
}

struct CPU {
    registers: [u8; 16],
    pc: usize,
    memory: [u8; 0x1000],
    stack: [u16; 16],
    sp: usize,
}

impl CPU {

    fn new() -> Self {
        CPU {
            registers: [0; 16],
            pc: 0,
            memory: [0; 0x1000],
            stack: [0; 16],
            sp: 0,
        }
    }
    fn fetch(&mut self) -> u16 {
        let opbyte_h = self.memory[self.pc] as u16;
        let opbyte_l = self.memory[self.pc + 1] as u16;
        self.pc += 2;
        (opbyte_h << 8) | opbyte_l
    }

    fn decode(&self, opcode: u16) -> Opcode {
        let c = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let n = (opcode & 0x000F) as u8;

        let c = match (c, x, y, n) {
            (0x0, 0x0, 0x0, 0x0) => Opcode::Halt,
            (0x0, 0x0, 0xE, 0xE) => Opcode::Ret,
            (0x2, _, _, _) => Opcode::Call(opcode & 0x0FFF),
            (0x8, _, _, 0x4) => Opcode::Add(x, y),
            _ => todo!("opcode {:04x}", opcode),
        };

        c
    }

    fn run(&mut self) {
        loop {
            let opcode_b = self.fetch();
            // 4bitずつに分割
            let opcode = self.decode(opcode_b);
            match opcode {
                Opcode::Halt => return,
                Opcode::Add(x, y) => self.add_xy(x, y),
                Opcode::Call(addr) => self.call(addr),
                Opcode::Ret => self.ret(),
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

    fn call(&mut self, addr: u16) {
        if self.sp >= self.stack.len() {
            panic!("stack overflow");
        }

        self.stack[self.sp] = self.pc as u16;
        self.sp += 1;
        self.pc = addr as usize;
    }

    fn ret(&mut self) {
        if self.sp == 0 {
            panic!("stack underflow");
        }

        self.sp -= 1;
        self.pc = self.stack[self.sp] as usize;
    }
}

fn main() {
    let mut cpu = CPU::new();

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    let mem = &mut cpu.memory;
    mem[0x000] = 0x21; mem[0x001] = 0x00;
    mem[0x002] = 0x21; mem[0x003] = 0x00;
    mem[0x004] = 0x00; mem[0x005] = 0x00;

    mem[0x100] = 0x80; mem[0x101] = 0x14;
    mem[0x102] = 0x80; mem[0x103] = 0x14;
    mem[0x104] = 0x00; mem[0x105] = 0xEE;

    cpu.run();

    assert_eq!(cpu.registers[0], 45);
    println!("5 + (10 * 2) + 10 + (10 * 2) = {}", cpu.registers[0]);
}
