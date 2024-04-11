use std::mem::transmute;

#[repr(u8)]
enum OpCode {
    Halt = 0x0,
    Const,
    Add,
    Sub,
    Mul,
    Div
}

#[derive(Clone, Copy, Debug)]
pub enum LinoValue {
    Number(f32),
}

impl LinoValue {
    fn as_number(self) -> f32 {
        match self {
            Self::Number(number) => number,
        }
    }
}

macro_rules! binary_op {
    ($x:ident, $op:tt) => {{
        let a = $x.pop().as_number();
        let b = $x.pop().as_number();
        $x.push(LinoValue::Number(b $op a));
    }};
}

const STACK_LIMIT: usize = 512;

pub struct LinoVm<'a> {
    constants: Vec<LinoValue>,
    stack: [LinoValue; STACK_LIMIT],
    bytecode: &'a [u8],
    ip: usize, // instruction pointer
    sp: usize, // stack pointer
}

impl<'a> LinoVm<'a> {
    pub fn new(bytecode: &'a [u8]) -> Self {
        Self {
            bytecode,
            ip: 0,
            sp: 0,
            constants: vec![LinoValue::Number(10.0), LinoValue::Number(20.0)],
            stack: [LinoValue::Number(0.0); STACK_LIMIT]
        }
    }

    fn push(&mut self, value: LinoValue) {
        if self.sp == STACK_LIMIT {
            panic!("stack overflow");
        }

        self.stack[self.sp] = value;
        self.sp += 1;
    }

    fn pop(&mut self) -> LinoValue {
        if self.sp == 0 {
            panic!("stack underflow");
        }

        self.sp -= 1;
        self.stack[self.sp]
    }

    fn read_byte(&mut self) -> u8 {
        let byte = self.bytecode[self.ip];
        self.ip += 1;
        byte
    }

    pub fn run(&mut self) -> LinoValue {
        loop {
            let opcode: OpCode = unsafe { transmute(self.bytecode[self.ip]) };
            self.ip += 1;

            match opcode {
                OpCode::Halt => return self.pop(),
                OpCode::Const => {
                    let index = self.read_byte();
                    let constant = self.constants[index as usize];
                    self.push(constant);
                },
                OpCode::Add => binary_op!(self, +),
                OpCode::Sub => binary_op!(self, -),
                OpCode::Mul => binary_op!(self, *),
                OpCode::Div => binary_op!(self, /),
            }
        }
    }
}

#[test]
fn test() {
    let bytecode = vec![
        OpCode::Const as u8, 0x0, 
        OpCode::Const as u8, 0x1,
        OpCode::Mul as u8,
        OpCode::Halt as u8
    ];
    let mut vm = LinoVm::new(&bytecode);

    let result = vm.run();

    print!("{:?}", result);
}