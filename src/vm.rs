use std::fmt::Display;
use std::io::{Read, Write};

use crate::compiler::ByteCode;

#[repr(u8)]
#[derive(Debug)]
pub enum OpCode {
    Halt = 0x0,

    Const,
    Dup,
    Pop,

    CastI,
    CastF,
    CastS,

    Add,
    Sub,
    Mul,
    Div,
    Rem,

    Or,
    And,

    // TODO
    // Xor,
    // Shl,
    // Neg,
    Jmp,
    JmpT,
    JmpF,

    Eq,
    NE,
    LT,
    GT,
    LE,
    GE,

    ReadL,
    ReadI,
    ReadF,

    Write,

    Load,
    Store,

    Call,
    Return,
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpCode::Halt => write!(f, "HALT"),
            OpCode::Const => write!(f, "CONST"),
            OpCode::Dup => write!(f, "DUP"),
            OpCode::Pop => write!(f, "POP"),
            OpCode::CastI => write!(f, "CASTI"),
            OpCode::CastF => write!(f, "CASTF"),
            OpCode::CastS => write!(f, "CASTS"),
            OpCode::Add => write!(f, "ADD"),
            OpCode::Sub => write!(f, "SUB"),
            OpCode::Mul => write!(f, "MUL"),
            OpCode::Div => write!(f, "DIV"),
            OpCode::Rem => write!(f, "REM"),
            OpCode::Or => write!(f, "OR"),
            OpCode::And => write!(f, "AND"),
            OpCode::Jmp => write!(f, "JMP"),
            OpCode::JmpT => write!(f, "JMPT"),
            OpCode::JmpF => write!(f, "JMPF"),
            OpCode::Eq => write!(f, "EQ"),
            OpCode::NE => write!(f, "NE"),
            OpCode::LT => write!(f, "LT"),
            OpCode::GT => write!(f, "GT"),
            OpCode::LE => write!(f, "LE"),
            OpCode::GE => write!(f, "GE"),
            OpCode::ReadL => write!(f, "READL"),
            OpCode::ReadI => write!(f, "READI"),
            OpCode::ReadF => write!(f, "READF"),
            OpCode::Write => write!(f, "WRITE"),
            OpCode::Load => write!(f, "LOAD"),
            OpCode::Store => write!(f, "STORE"),
            OpCode::Call => write!(f, "CALL"),
            OpCode::Return => write!(f, "RETURN"),
        }
    }
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
    Address(usize),
    Boolean(bool),
}

impl Default for LinaValue {
    fn default() -> Self {
        Self::Boolean(false)
    }
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
    IoError(std::io::Error),
    FromUtf8Error(std::string::FromUtf8Error),
    ParseIntError(std::num::ParseIntError),
    ParseFloatError(std::num::ParseFloatError),
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

impl From<std::io::Error> for RuntimeError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<std::string::FromUtf8Error> for RuntimeError {
    fn from(value: std::string::FromUtf8Error) -> Self {
        Self::FromUtf8Error(value)
    }
}

impl From<std::num::ParseIntError> for RuntimeError {
    fn from(value: std::num::ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl From<std::num::ParseFloatError> for RuntimeError {
    fn from(value: std::num::ParseFloatError) -> Self {
        Self::ParseFloatError(value)
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::CodeError(err) => write!(f, "{err}"),
            RuntimeError::TypeError(err) => write!(f, "{err}"),
            RuntimeError::IoError(err) => write!(f, "{err}"),
            RuntimeError::FromUtf8Error(err) => write!(f, "{err}"),
            RuntimeError::ParseIntError(err) => write!(f, "{err}"),
            RuntimeError::ParseFloatError(err) => write!(f, "{err}"),
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum VmState {
    Idle,      // vm is ready to start
    Executing, // while this, execute the bytecode
    WillRead,  // next instruction is to read
    WillWrite, // next instruction is to write
}

impl Display for VmState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VmState::Idle => write!(f, "idle"),
            VmState::Executing => write!(f, "executing"),
            VmState::WillRead => write!(f, "will-read"),
            VmState::WillWrite => write!(f, "will-write"),
        }
    }
}

impl Default for VmState {
    fn default() -> Self {
        VmState::Idle
    }
}

type VmResult<T> = Result<T, RuntimeError>;

pub struct LinaVm<In, Out>
where
    In: Read,
    Out: Write,
{
    bytecode: Vec<u8>,         // bytecode to be executed
    constants: Vec<LinaValue>, // constant pool
    pc: usize,                 // program counter
    stack: Vec<LinaValue>,     // operand stack
    pub stdin: In,             // standard input
    pub stdout: Out,           // standard output
}

impl<In, Out> LinaVm<In, Out>
where
    In: Read,
    Out: Write,
{
    pub fn new(code: ByteCode, stdin: In, stdout: Out) -> Self {
        Self {
            bytecode: code.bytecode,
            constants: code.constants,
            pc: 0,
            stack: Vec::with_capacity(512),
            stdin,
            stdout,
        }
    }

    pub fn empty(stdin: In, stdout: Out) -> Self {
        Self {
            bytecode: Vec::default(),
            constants: Vec::default(),
            pc: 0,
            stack: Vec::with_capacity(512),
            stdin,
            stdout,
        }
    }

    pub fn start(&mut self, code: ByteCode) {
        self.bytecode = code.bytecode;
        self.constants = code.constants;
        self.pc = 0;
        self.stack.clear();
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.stack.clear();
    }

    fn push(&mut self, value: LinaValue) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> LinaValue {
        self.stack.pop().expect("stack should not be empty")
    }

    fn curr_byte(&mut self) -> u8 {
        self.bytecode[self.pc]
    }

    fn next_byte(&mut self) -> u8 {
        assert!(self.pc < self.bytecode.len());
        self.pc += 1;
        self.bytecode[self.pc]
    }

    fn next_addr(&mut self) -> usize {
        let bytes = core::array::from_fn(|_| self.next_byte());
        usize::from_ne_bytes(bytes)
    }

    fn next_offs(&mut self) -> isize {
        let bytes = core::array::from_fn(|_| self.next_byte());
        isize::from_ne_bytes(bytes)
    }

    fn store(&mut self, value: LinaValue, address: usize) {
        if address >= self.stack.len() {
            self.stack.resize_with(address + 1, Default::default);
        }
        self.stack[address] = value;
    }

    fn load(&mut self, address: usize) -> &LinaValue {
        &self.stack[address]
    }

    fn binop(&mut self, op: OpCode) -> VmResult<()> {
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
            OpCode::Or => match lhs {
                LinaValue::Int32(lhs) => {
                    let rhs: i32 = rhs.try_into()?;
                    (lhs | rhs).into()
                }
                LinaValue::Boolean(lhs) => {
                    let rhs: bool = rhs.try_into()?;
                    (lhs | rhs).into()
                }
                LinaValue::Address(lhs) => {
                    let rhs: usize = rhs.try_into()?;
                    (lhs | rhs).into()
                }
                _ => Err(TypeError(format!(
                    "operacao % não implementada para {:?}",
                    lhs
                )))?,
            },
            OpCode::And => match lhs {
                LinaValue::Int32(lhs) => {
                    let rhs: i32 = rhs.try_into()?;
                    (lhs & rhs).into()
                }
                LinaValue::Boolean(lhs) => {
                    let rhs: bool = rhs.try_into()?;
                    (lhs & rhs).into()
                }
                LinaValue::Address(lhs) => {
                    let rhs: usize = rhs.try_into()?;
                    (lhs & rhs).into()
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

    fn read(&mut self, stopc: &[u8]) -> Result<String, RuntimeError> {
        let mut buff = Vec::new();
        let mut byte = [0_u8];

        loop {
            let n = self.stdin.read(&mut byte)?;
            if n == 0 || stopc.contains(&byte[0]) {
                break;
            }
            buff.push(byte[0]);
        }

        let value = String::from_utf8(buff)?;
        Ok(value)
    }

    pub fn run_instr(&mut self) -> VmResult<VmState> {
        let opcode: OpCode = self.curr_byte().try_into()?;

        match opcode {
            OpCode::Halt => return Ok(VmState::Idle),

            OpCode::Const => {
                let index = self.next_addr();
                let constant = &self.constants[index];
                self.push(constant.clone());
            }
            OpCode::Dup => {
                let top = self.pop();
                self.push(top.clone());
                self.push(top);
            }
            OpCode::Pop => _ = self.pop(),

            OpCode::CastI => {
                let top = self.pop();
                let val: i32 = top.try_into()?;
                self.push(val.into());
            }
            OpCode::CastF => {
                let top = self.pop();
                let val: f32 = top.try_into()?;
                self.push(val.into());
            }
            OpCode::CastS => {
                let top = self.pop();
                let val: String = top.into();
                self.push(val.into());
            }

            OpCode::Add
            | OpCode::Sub
            | OpCode::Mul
            | OpCode::Div
            | OpCode::Rem
            | OpCode::Or
            | OpCode::And
            | OpCode::Eq
            | OpCode::NE
            | OpCode::LT
            | OpCode::GT
            | OpCode::LE
            | OpCode::GE => self.binop(opcode)?,

            // Controle de fluxo
            OpCode::Jmp => {
                let offset = self.next_offs();
                self.pc = (self.pc as isize + offset) as usize;
            }
            OpCode::JmpT => {
                let condition: bool = self.pop().try_into()?;
                let offset = self.next_offs();

                if condition {
                    self.pc = (self.pc as isize + offset) as usize;
                }
            }
            OpCode::JmpF => {
                let condition: bool = self.pop().try_into()?;
                let offset = self.next_offs();

                if !condition {
                    self.pc = (self.pc as isize + offset) as usize;
                }
            }

            OpCode::Call => todo!(),
            OpCode::Return => todo!(),

            OpCode::Load => {
                let address = self.next_addr();
                let value = self.load(address).clone();
                self.push(value);
            }
            OpCode::Store => {
                let value = self.pop();
                let address = self.next_addr();
                self.store(value, address);
            }

            OpCode::Write => {
                let value = self.pop();
                write!(self.stdout, "{value}")?;
            }
            OpCode::ReadL => {
                let line = self.read(&[b'\n'])?;
                self.push(LinaValue::String(line));
            }
            OpCode::ReadI => {
                let value = self.read(&[b'\n', b' '])?.parse::<i32>()?;
                self.push(LinaValue::Int32(value));
            }
            OpCode::ReadF => {
                let value = self.read(&[b'\n', b' '])?.parse::<f32>()?;
                self.push(LinaValue::Float32(value));
            }
        };

        let next: OpCode = self.next_byte().try_into()?;

        let state = match next {
            OpCode::Write => VmState::WillWrite,
            OpCode::ReadL | OpCode::ReadI | OpCode::ReadF => VmState::WillRead,
            _ => VmState::Executing,
        };

        Ok(state)
    }

    pub fn non_stop(&mut self) -> VmResult<()> {
        loop {
            let vm_state = self.run_instr()?;
            if let VmState::Idle = vm_state {
                break Ok(());
            }
        }
    }

    pub fn decompile(&mut self) -> VmResult<()> {
        loop {
            let opcode: OpCode = self.next_byte().try_into()?;

            match opcode {
                OpCode::Halt => {
                    writeln!(self.stdout, "{opcode}")?;
                    return Ok(());
                }
                OpCode::Const => {
                    let index = self.next_addr();
                    let value = &self.constants[index];
                    let fmt_value = match value {
                        LinaValue::Int32(value) => format!("{}i32", value),
                        LinaValue::Float32(value) => format!("{}f32", value),
                        LinaValue::String(value) => format!("\"{}\"", value.escape_default()),
                        LinaValue::Boolean(value) => format!("{}", value),
                        LinaValue::Address(value) => format!("{:#02x}", value),
                    };
                    writeln!(self.stdout, "{opcode}\t{index:#02x}\t{fmt_value}")?;
                }
                OpCode::Jmp => {
                    let index = self.next_offs();
                    writeln!(self.stdout, "{opcode}\t{index}")?;
                }
                OpCode::JmpT => {
                    let index = self.next_offs();
                    writeln!(self.stdout, "{opcode}\t{index}")?;
                }
                OpCode::JmpF => {
                    let index = self.next_offs();
                    writeln!(self.stdout, "{opcode}\t{index}")?;
                }
                OpCode::Load => {
                    let index = self.next_addr();
                    writeln!(self.stdout, "{opcode}\t{index:#02x}")?;
                }
                OpCode::Store => {
                    let index = self.next_addr();
                    writeln!(self.stdout, "{opcode}\t{index:#02x}")?;
                }
                OpCode::Call => todo!(),
                OpCode::Return => todo!(),
                _ => writeln!(self.stdout, "{opcode}")?,
            }
        }
    }
}
