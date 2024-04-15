#[repr(u8)]
pub enum OpCode {
    Halt = 0x0,

    Const,

    Add,
    Sub,
    Mul,
    Div,

    Jump,
    JumpIfTrue,
    JumpIfFalse,
    
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,

    Load,
    Store,

    Call,
    Return,
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum LinoValue {
    Number(f32),
    String(String),
    Bool(bool),
}

impl LinoValue {
    fn as_number(self) -> f32 {
        match self {
            Self::Number(number) => number,
            _ => panic!("variavel não é um número")
        }
    }

    fn as_string(self) -> String {
        match self {
            Self::String(string) => string,
            _ => panic!("variavel não é uma string")
        }
    }

    fn as_bool(self) -> bool {
        match self {
            Self::Bool(boolean) => boolean,
            _ => panic!("variavel não é um booleano")
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
const LINO_VALUE_DEFAULT: Option<LinoValue> = None;

#[derive(Debug)]
pub struct LinoVm<'a> {
    constants: Vec<LinoValue>,
    stack: [Option<LinoValue>; STACK_LIMIT],
    memory: Vec<Option<LinoValue>>,
    bytecode: &'a [u8],
    ip: usize, // instruction pointer
    sp: usize, // stack pointer
}

impl<'a> LinoVm<'a> {
    pub fn new(bytecode: &'a [u8], constants: Vec<LinoValue>) -> Self {
        Self {
            bytecode,
            ip: 0,
            sp: 0,
            constants,
            stack: [LINO_VALUE_DEFAULT; STACK_LIMIT],
            memory: Vec::new(),
        }
    }

    fn push(&mut self, value: LinoValue) {
        assert!(self.sp <= STACK_LIMIT, "stack overflow");
        self.stack[self.sp] = Some(value);
        self.sp += 1;
    }

    fn pop(&mut self) -> LinoValue {
        assert!(self.sp > 0, "stack underflow");
        self.sp -= 1;
        std::mem::replace(&mut self.stack[self.sp], None).unwrap()
    }

    fn store(&mut self, address: usize, value: LinoValue) {
        if self.memory.len() == address {
            self.memory.push(Some(value));
        } else if self.memory.len() > address {
            self.memory[address] = Some(value);
        } else {
            panic!("invalid memory acess {address}");
        }
    }

    fn load(&mut self, address: usize) -> LinoValue {
        self.memory[address].as_ref().unwrap().clone()
    }

    fn read_byte(&mut self) -> u8 {
        let byte = self.bytecode[self.ip];
        self.ip += 1;
        byte
    }

    pub fn run(&mut self) {
        loop {
            let opcode: OpCode = self.bytecode[self.ip].into();
            self.ip += 1;
            
            match opcode {
                OpCode::Halt => return,
                OpCode::Const => {
                    let index = self.read_byte();
                    let constant = &self.constants[index as usize];
                    self.push(constant.clone());
                },
                OpCode::Add => binary_op!(self, +),
                OpCode::Sub => binary_op!(self, -),
                OpCode::Mul => binary_op!(self, *),
                OpCode::Div => binary_op!(self, /),

                OpCode::Equal => {
                    let a = self.pop();
                    let b = self.pop();
                    self.push(LinoValue::Bool(a == b));
                },
                OpCode::NotEqual => {
                    let a = self.pop();
                    let b = self.pop();
                    self.push(LinoValue::Bool(a != b));
                },

                OpCode::LessThan => {
                    let a = self.pop().as_number();
                    let b = self.pop().as_number();
                    self.push(LinoValue::Bool(b < a));
                },
                OpCode::GreaterThan => {
                    let a = self.pop().as_number();
                    let b = self.pop().as_number();
                    self.push(LinoValue::Bool(b > a));
                },

                // Controle de fluxo
                OpCode::Jump => {
                    let offset = self.read_byte() as i8;
                    self.ip = (self.ip as isize + offset as isize) as usize;
                },
                OpCode::JumpIfTrue => {
                    let condition = self.pop();
                    let offset = self.read_byte() as i8;

                    if condition.as_bool() {
                        self.ip = (self.ip as isize + offset as isize) as usize;
                    }
                },
                OpCode::JumpIfFalse => {
                    let condition = self.pop();
                    let offset = self.read_byte() as i8;

                    if !condition.as_bool() {
                        self.ip = (self.ip as isize + offset as isize) as usize;
                    }
                },

                OpCode::Call => {
                    let function_address = self.read_byte() as usize;
                    self.push(LinoValue::Number(self.ip as f32));
                    self.ip = function_address;
                },
                OpCode::Return => {
                    let return_address = self.pop();
                    self.ip = return_address.as_number() as usize;
                },

                OpCode::Load => {
                    let address = self.read_byte() as usize;
                    let value = self.load(address);
                    self.push(value);
                },
                OpCode::Store => {
                    let address = self.read_byte() as usize;
                    let value = self.pop();
                    self.store(address, value);
                },
            }
        }
    }

    pub fn debug(&self) {
        println!("stack: {:?}", self.stack.iter().filter(|v| v.is_some()).collect::<Vec<_>>());
        println!("memory: {:?}", self.memory);
    }
}

#[test]
fn test() {
    let constants = vec![
        LinoValue::Number(15.0),
        LinoValue::Number(10.0),
        LinoValue::Number(2.0)
    ];
    let bytecode = vec![
        OpCode::Const as u8, 0x1,   // 10
        OpCode::Const as u8, 0x0,   // 15
        OpCode::GreaterThan as u8,  // >

        OpCode::JumpIfFalse as u8, 0x7, // if

        // true part
        OpCode::Const as u8, 0x0,   // 15
        OpCode::Const as u8, 0x1,   // 10
        OpCode::Sub as u8,
        OpCode::Jump as u8, 0x5,
        
        // false part
        OpCode::Const as u8, 0x0,   // 15 
        OpCode::Const as u8, 0x1,   // 10
        OpCode::Add as u8,

        OpCode::Halt as u8,
    ];
    let mut vm = LinoVm::new(&bytecode, constants);

    vm.run();
}