use std::fmt::Display;

#[repr(u8)]
#[derive(Debug)]
pub enum OpCode {
    Halt = 0x0,

    Const,
    Dup,

    Add,
    Sub,
    Mul,
    Div,

    Jmp,
    JmpT,
    JmpF,

    Eq,
    NE,
    LT,
    GT,
    LE,
    GE,

    Read,
    Write,

    Load,
    Store,

    GLoad,
    GStore,

    Call,
    Return,
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum LinaValue {
    Number(f32),
    String(String),
    Boolean(bool),
    Address(usize),
}

impl LinaValue {
    fn as_number(self) -> f32 {
        match self {
            Self::Number(number) => number,
            _ => panic!("variavel não é um número"),
        }
    }

    fn as_string(self) -> String {
        match self {
            Self::String(string) => string,
            _ => panic!("variavel não é uma string"),
        }
    }

    fn as_bool(self) -> bool {
        match self {
            Self::Boolean(boolean) => boolean,
            _ => panic!("variavel não é um booleano"),
        }
    }

    fn as_address(self) -> usize {
        match self {
            Self::Address(address) => address,
            _ => panic!("variavel não é um endereço"),
        }
    }
}

impl From<f32> for LinaValue {
    fn from(value: f32) -> Self {
        LinaValue::Number(value)
    }
}

impl From<String> for LinaValue {
    fn from(value: String) -> Self {
        LinaValue::String(value)
    }
}

impl From<bool> for LinaValue {
    fn from(value: bool) -> Self {
        LinaValue::Boolean(value)
    }
}

impl Display for LinaValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LinaValue::Number(value) => write!(f, "{value}"),
            LinaValue::String(value) => write!(f, "{value}"),
            LinaValue::Boolean(value) => write!(f, "{value}"),
            LinaValue::Address(value) => write!(f, "{value:#02x}"),
        }
    }
}

macro_rules! binary_op {
    ($x:ident, $op:tt) => {{
        let a = $x.pop().as_number();
        let b = $x.pop().as_number();
        $x.push((b $op a).into());
    }};
}

#[derive(Debug)]
struct Frame {
    locals: Vec<LinaValue>, // local variables
    stack: Vec<LinaValue>,  // operand stack
}

impl Frame {
    fn new(return_address: usize) -> Self {
        Self {
            locals: Vec::new(),
            stack: vec![LinaValue::Address(return_address)],
        }
    }
}

#[derive(Debug)]
pub struct LinaVm<'a> {
    bytecode: &'a [u8], // bytecode to be executed

    pc: usize, // program counter

    constants: &'a [LinaValue], // constant pool
    callstack: Vec<Frame>,      // call stack

    globals: Vec<LinaValue>,
}

impl<'a> LinaVm<'a> {
    pub fn new(bytecode: &'a [u8], constants: &'a [LinaValue]) -> Self {
        Self {
            bytecode,
            pc: 0,
            constants,
            callstack: vec![Frame::new(0)],
            globals: Vec::new(),
        }
    }

    fn push(&mut self, value: LinaValue) {
        self.callstack
            .last_mut()
            .expect("frame stack should not be empty")
            .stack
            .push(value);
    }

    fn pop(&mut self) -> LinaValue {
        self.callstack
            .last_mut()
            .expect("frame stack should not be empty")
            .stack
            .pop()
            .expect("stack should not be empty")
    }

    fn read_byte(&mut self) -> u8 {
        assert!(self.pc < self.bytecode.len());
        let byte = self.bytecode[self.pc];
        self.pc += 1;
        byte
    }

    fn read_address(&mut self) -> usize {
        let bytes = core::array::from_fn(|_i| self.read_byte());
        usize::from_ne_bytes(bytes)
    }

    fn read_offset(&mut self) -> isize {
        let bytes = core::array::from_fn(|_i| self.read_byte());
        isize::from_ne_bytes(bytes)
    }

    fn store(&mut self, value: LinaValue, address: usize) {
        self.callstack
            .last_mut()
            .expect("frame stack should not be empty")
            .locals[address] = value;
    }

    fn read(&mut self, address: usize) -> LinaValue {
        self.callstack
            .last_mut()
            .expect("frame stack should not be empty")
            .locals[address]
            .clone()
    }

    fn store_global(&mut self, value: LinaValue, address: usize) {
        while self.globals.len() < address + 1 {
            self.globals.push(0.0.into());
        }

        self.globals[address] = value;
    }

    fn read_global(&mut self, address: usize) -> LinaValue {
        self.globals[address].clone()
    }

    pub fn run(&mut self) {
        loop {
            let opcode: OpCode = self.read_byte().into();

            match opcode {
                OpCode::Halt => break,

                OpCode::Const => {
                    let index = self.read_address();
                    let constant = &self.constants[index];
                    self.push(constant.clone());
                }
                OpCode::Dup => {
                    let top = self.pop();
                    self.push(top.clone());
                    self.push(top);
                }

                OpCode::Add => binary_op!(self, +),
                OpCode::Sub => binary_op!(self, -),
                OpCode::Mul => binary_op!(self, *),
                OpCode::Div => binary_op!(self, /),

                OpCode::Eq => binary_op!(self, ==),
                OpCode::NE => binary_op!(self, !=),

                OpCode::LT => binary_op!(self, <),
                OpCode::GT => binary_op!(self, >),

                OpCode::LE => binary_op!(self, <=),
                OpCode::GE => binary_op!(self, >=),

                // Controle de fluxo
                OpCode::Jmp => {
                    let offset = self.read_offset();
                    self.pc = (self.pc as isize + offset) as usize;
                }
                OpCode::JmpT => {
                    let condition = self.pop();
                    let offset = self.read_offset();

                    if condition.as_bool() {
                        self.pc = (self.pc as isize + offset) as usize;
                    }
                }
                OpCode::JmpF => {
                    let condition = self.pop();
                    let offset = self.read_offset();

                    if !condition.as_bool() {
                        self.pc = (self.pc as isize + offset) as usize;
                    }
                }

                OpCode::Call => {
                    let function_address = self.read_address();
                    let frame = Frame::new(self.pc);
                    self.callstack.push(frame);
                    self.pc = function_address;
                }
                OpCode::Return => {
                    let return_address = self.pop().as_address();
                    self.callstack.pop();
                    self.pc = return_address;
                }

                OpCode::Load => {
                    let address = self.read_address();
                    let value = self.read(address);
                    self.push(value);
                }
                OpCode::Store => {
                    let value = self.pop();
                    let address = self.read_address();
                    self.store(value, address);
                }

                OpCode::GLoad => {
                    let address = self.read_address();
                    let value = self.read_global(address);
                    self.push(value);
                }
                OpCode::GStore => {
                    let value = self.pop();
                    let address = self.read_address();
                    self.store_global(value, address);
                }

                OpCode::Write => {
                    let value = self.pop();
                    println!("{value}");
                }
                OpCode::Read => todo!(),
            }
        }
    }
}
