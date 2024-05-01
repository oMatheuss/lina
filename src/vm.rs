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
    Rem,

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

    Call,
    Return,
}

#[derive(Debug)]
pub struct CodeError(String);

impl From<String> for CodeError {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Display for CodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CodeError: {}", self.0)
    }
}

impl TryFrom<u8> for OpCode {
    type Error = CodeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value < OpCode::Halt as u8 || value > OpCode::Return as u8 {
            Err(format!("{value} não é um opcode conhecido").into())
        } else {
            Ok(unsafe { std::mem::transmute(value) })
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum LinaValue {
    Int32(i32),
    Float32(f32),
    String(String),
    Boolean(bool),
    Address(usize),
}

pub struct TypeError(String);

impl From<&str> for TypeError {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for TypeError {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TypeError: {}", self.0)
    }
}

impl TryInto<i32> for LinaValue {
    type Error = TypeError;

    fn try_into(self) -> Result<i32, Self::Error> {
        match self {
            Self::Int32(number) => Ok(number),
            Self::Float32(number) => Ok(number as i32),
            other => Err(format!("{other} não pode ser convertido em i32").into()),
        }
    }
}

impl From<i32> for LinaValue {
    fn from(value: i32) -> Self {
        LinaValue::Int32(value)
    }
}

impl TryInto<f32> for LinaValue {
    type Error = TypeError;

    fn try_into(self) -> Result<f32, Self::Error> {
        match self {
            Self::Float32(number) => Ok(number),
            Self::Int32(number) => Ok(number as f32),
            other => Err(format!("{other} não pode ser convertido em f32").into()),
        }
    }
}

impl From<f32> for LinaValue {
    fn from(value: f32) -> Self {
        LinaValue::Float32(value)
    }
}

impl Into<String> for LinaValue {
    fn into(self) -> String {
        self.to_string()
    }
}

impl From<String> for LinaValue {
    fn from(value: String) -> Self {
        LinaValue::String(value)
    }
}

impl TryInto<bool> for LinaValue {
    type Error = TypeError;

    fn try_into(self) -> Result<bool, Self::Error> {
        match self {
            Self::Boolean(boolean) => Ok(boolean),
            other => Err(format!("{other} não pode ser convertido em bool").into()),
        }
    }
}

impl From<bool> for LinaValue {
    fn from(value: bool) -> Self {
        LinaValue::Boolean(value)
    }
}

impl TryInto<usize> for LinaValue {
    type Error = TypeError;

    fn try_into(self) -> Result<usize, Self::Error> {
        match self {
            Self::Address(address) => Ok(address),
            Self::Int32(number) => {
                if number >= 0 {
                    Ok(number as usize)
                } else {
                    Err(format!("{number} é negativo e não pode ser convertido em usize").into())
                }
            }
            Self::Float32(number) => {
                if number >= 0.0 {
                    Ok(number as usize)
                } else {
                    Err(format!("{number} é negativo e não pode ser convertido em usize").into())
                }
            }
            other => Err(format!("{other} não pode ser convertido em usize").into()),
        }
    }
}

impl From<usize> for LinaValue {
    fn from(value: usize) -> Self {
        LinaValue::Address(value)
    }
}

impl Display for LinaValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LinaValue::Int32(value) => value.fmt(f),
            LinaValue::Float32(value) => value.fmt(f),
            LinaValue::String(value) => value.fmt(f),
            LinaValue::Boolean(value) => value.fmt(f),
            LinaValue::Address(value) => write!(f, "{value:#02x}"),
        }
    }
}

pub enum RuntimeError {
    CodeError(CodeError),
    TypeError(TypeError),
}

impl From<TypeError> for RuntimeError {
    fn from(value: TypeError) -> Self {
        Self::TypeError(value)
    }
}

impl From<CodeError> for RuntimeError {
    fn from(value: CodeError) -> Self {
        Self::CodeError(value)
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::CodeError(err) => write!(f, "{err}"),
            RuntimeError::TypeError(err) => write!(f, "{err}"),
        }
    }
}

type VmResult = Result<(), RuntimeError>;

#[derive(Debug)]
pub struct LinaVm<'a> {
    bytecode: &'a [u8], // bytecode to be executed

    pc: usize, // program counter

    stack: Vec<LinaValue>,      // operand stack
    constants: &'a [LinaValue], // constant pool

    globals: Vec<LinaValue>,
}

impl<'a> LinaVm<'a> {
    pub fn new(bytecode: &'a [u8], constants: &'a [LinaValue]) -> Self {
        Self {
            bytecode,
            pc: 0,
            stack: Vec::with_capacity(512),
            constants,
            globals: Vec::new(),
        }
    }

    fn push(&mut self, value: LinaValue) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> LinaValue {
        self.stack.pop().expect("stack should not be empty")
    }

    fn next_byte(&mut self) -> u8 {
        assert!(self.pc < self.bytecode.len());
        let byte = self.bytecode[self.pc];
        self.pc += 1;
        byte
    }

    fn next_address(&mut self) -> usize {
        let bytes = core::array::from_fn(|_i| self.next_byte());
        usize::from_ne_bytes(bytes)
    }

    fn next_offset(&mut self) -> isize {
        let bytes = core::array::from_fn(|_i| self.next_byte());
        isize::from_ne_bytes(bytes)
    }

    fn store(&mut self, value: LinaValue, address: usize) {
        while self.globals.len() < address + 1 {
            self.globals.push(0.0.into());
        }

        self.globals[address] = value;
    }

    fn load(&mut self, address: usize) -> LinaValue {
        self.globals[address].clone()
    }

    fn binary_op(&mut self, op: OpCode) -> VmResult {
        let rhs = self.pop();
        let lhs = self.pop();

        let result: LinaValue = match op {
            OpCode::Add => match lhs {
                LinaValue::Int32(lhs) => {
                    let rhs: i32 = rhs.try_into()?;
                    (lhs + rhs).into()
                }
                LinaValue::Float32(lhs) => {
                    let rhs: f32 = rhs.try_into()?;
                    (lhs + rhs).into()
                }
                LinaValue::Address(lhs) => {
                    let rhs: usize = rhs.try_into()?;
                    (lhs + rhs).into()
                }
                LinaValue::String(lhs) => {
                    let rhs: String = rhs.into();
                    (lhs + &rhs).into()
                }
                _ => Err(TypeError(format!(
                    "operacao + não implementada para {:?}",
                    lhs
                )))?,
            },
            OpCode::Sub => match lhs {
                LinaValue::Int32(lhs) => {
                    let rhs: i32 = rhs.try_into()?;
                    (lhs - rhs).into()
                }
                LinaValue::Float32(lhs) => {
                    let rhs: f32 = rhs.try_into()?;
                    (lhs - rhs).into()
                }
                LinaValue::Address(lhs) => {
                    let rhs: usize = rhs.try_into()?;
                    (lhs - rhs).into()
                }
                _ => Err(TypeError(format!(
                    "operacao - não implementada para {:?}",
                    lhs
                )))?,
            },
            OpCode::Mul => match lhs {
                LinaValue::Int32(lhs) => {
                    let rhs: i32 = rhs.try_into()?;
                    (lhs * rhs).into()
                }
                LinaValue::Float32(lhs) => {
                    let rhs: f32 = rhs.try_into()?;
                    (lhs * rhs).into()
                }
                LinaValue::Address(lhs) => {
                    let rhs: usize = rhs.try_into()?;
                    (lhs * rhs).into()
                }
                _ => Err(TypeError(format!(
                    "operacao * não implementada para {:?}",
                    lhs
                )))?,
            },
            OpCode::Div => match lhs {
                LinaValue::Int32(lhs) => {
                    let rhs: i32 = rhs.try_into()?;
                    (lhs / rhs).into()
                }
                LinaValue::Float32(lhs) => {
                    let rhs: f32 = rhs.try_into()?;
                    (lhs / rhs).into()
                }
                LinaValue::Address(lhs) => {
                    let rhs: usize = rhs.try_into()?;
                    (lhs / rhs).into()
                }
                _ => Err(TypeError(format!(
                    "operacao / não implementada para {:?}",
                    lhs
                )))?,
            },
            OpCode::Rem => match lhs {
                LinaValue::Int32(lhs) => {
                    let rhs: i32 = rhs.try_into()?;
                    (lhs % rhs).into()
                }
                LinaValue::Address(lhs) => {
                    let rhs: usize = rhs.try_into()?;
                    (lhs % rhs).into()
                }
                _ => Err(TypeError(format!(
                    "operacao % não implementada para {:?}",
                    lhs
                )))?,
            },
            OpCode::Eq => (lhs == rhs).into(),
            OpCode::NE => (lhs != rhs).into(),
            OpCode::LT => match lhs {
                LinaValue::Int32(lhs) => {
                    let rhs: i32 = rhs.try_into()?;
                    (lhs < rhs).into()
                }
                LinaValue::Float32(lhs) => {
                    let rhs: f32 = rhs.try_into()?;
                    (lhs < rhs).into()
                }
                LinaValue::Address(lhs) => {
                    let rhs: usize = rhs.try_into()?;
                    (lhs < rhs).into()
                }
                _ => Err(TypeError(format!(
                    "operacao < não implementada para {:?}",
                    lhs
                )))?,
            },
            OpCode::GT => match lhs {
                LinaValue::Int32(lhs) => {
                    let rhs: i32 = rhs.try_into()?;
                    (lhs > rhs).into()
                }
                LinaValue::Float32(lhs) => {
                    let rhs: f32 = rhs.try_into()?;
                    (lhs > rhs).into()
                }
                LinaValue::Address(lhs) => {
                    let rhs: usize = rhs.try_into()?;
                    (lhs > rhs).into()
                }
                _ => Err(TypeError(format!(
                    "operacao > não implementada para {:?}",
                    lhs
                )))?,
            },
            OpCode::LE => match lhs {
                LinaValue::Int32(lhs) => {
                    let rhs: i32 = rhs.try_into()?;
                    (lhs <= rhs).into()
                }
                LinaValue::Float32(lhs) => {
                    let rhs: f32 = rhs.try_into()?;
                    (lhs <= rhs).into()
                }
                LinaValue::Address(lhs) => {
                    let rhs: usize = rhs.try_into()?;
                    (lhs <= rhs).into()
                }
                _ => Err(TypeError(format!(
                    "operacao <= não implementada para {:?}",
                    lhs
                )))?,
            },
            OpCode::GE => match lhs {
                LinaValue::Int32(lhs) => {
                    let rhs: i32 = rhs.try_into()?;
                    (lhs >= rhs).into()
                }
                LinaValue::Float32(lhs) => {
                    let rhs: f32 = rhs.try_into()?;
                    (lhs >= rhs).into()
                }
                LinaValue::Address(lhs) => {
                    let rhs: usize = rhs.try_into()?;
                    (lhs >= rhs).into()
                }
                _ => Err(TypeError(format!(
                    "operacao >= não implementada para {:?}",
                    lhs
                )))?,
            },
            _ => Err(CodeError(format!("{} não é um operador binário", op as u8)))?,
        };

        self.push(result);

        Ok(())
    }

    pub fn run(&mut self) -> VmResult {
        loop {
            let opcode: OpCode = self.next_byte().try_into()?;

            match opcode {
                OpCode::Halt => return Ok(()),

                OpCode::Const => {
                    let index = self.next_address();
                    let constant = &self.constants[index];
                    self.push(constant.clone());
                }
                OpCode::Dup => {
                    let top = self.pop();
                    self.push(top.clone());
                    self.push(top);
                }

                OpCode::Add
                | OpCode::Sub
                | OpCode::Mul
                | OpCode::Div
                | OpCode::Rem
                | OpCode::Eq
                | OpCode::NE
                | OpCode::LT
                | OpCode::GT
                | OpCode::LE
                | OpCode::GE => self.binary_op(opcode)?,

                // Controle de fluxo
                OpCode::Jmp => {
                    let offset = self.next_offset();
                    self.pc = (self.pc as isize + offset) as usize;
                }
                OpCode::JmpT => {
                    let condition: bool = self.pop().try_into()?;
                    let offset = self.next_offset();

                    if condition {
                        self.pc = (self.pc as isize + offset) as usize;
                    }
                }
                OpCode::JmpF => {
                    let condition: bool = self.pop().try_into()?;
                    let offset = self.next_offset();

                    if !condition {
                        self.pc = (self.pc as isize + offset) as usize;
                    }
                }

                OpCode::Call => todo!(),
                OpCode::Return => todo!(),

                OpCode::Load => {
                    let address = self.next_address();
                    let value = self.load(address);
                    self.push(value);
                }
                OpCode::Store => {
                    let value = self.pop();
                    let address = self.next_address();
                    self.store(value, address);
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
