use std::collections::HashMap;

use crate::syntax::{Block, Expression, Program, SyntaxTree, Type};
use crate::token::{Literal, Operador};
use crate::vm::{LinaValue, OpCode};

pub struct ByteCode {
    pub bytecode: Vec<u8>,
    pub constants: Vec<LinaValue>,
}

type VarTable<'a> = HashMap<&'a str, usize>;

#[derive(Debug)]
pub struct Compiler<'a> {
    bytecode: Vec<u8>,
    constants: Vec<LinaValue>,
    scopes: Vec<VarTable<'a>>,
    vi: usize,
}

impl<'a> Compiler<'a> {
    pub fn new() -> Self {
        Self {
            bytecode: Vec::new(),
            constants: Vec::new(),
            scopes: vec![HashMap::new()],
            vi: 0,
        }
    }

    fn op_const(&mut self, addr: usize) {
        self.bytecode.push(OpCode::Const as u8);
        self.bytecode.extend(usize::to_ne_bytes(addr));
    }

    fn op_store(&mut self, addr: usize) {
        self.bytecode.push(OpCode::Store as u8);
        self.bytecode.extend(usize::to_ne_bytes(addr));
    }

    fn op_load(&mut self, addr: usize) {
        self.bytecode.push(OpCode::Load as u8);
        self.bytecode.extend(usize::to_ne_bytes(addr));
    }

    fn push_offset(&mut self, offset: isize) {
        // include itself on push
        const SIZE: isize = isize::BITS as isize / 8;
        let total = offset + (SIZE * offset.signum());
        self.bytecode.extend(isize::to_ne_bytes(total));
    }

    fn insert_offset(&mut self, offset: isize, pos: usize) {
        let value = isize::to_ne_bytes(offset);
        self.bytecode[pos..pos + std::mem::size_of::<isize>()].copy_from_slice(&value);
    }

    fn op(&mut self, op: OpCode) {
        self.bytecode.push(op as u8);
    }

    fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn exit_scope(&mut self) {
        let scope = self.scopes.pop().unwrap();
        self.vi -= scope.len();
    }

    fn get_current_scope(&mut self) -> &mut VarTable<'a> {
        self.scopes.last_mut().unwrap()
    }

    fn set_var(&mut self, name: &'a str) -> usize {
        let addr = self.vi;
        self.get_current_scope().insert(name, addr);
        self.vi += 1;
        addr
    }

    fn get_var(&mut self, name: &str) -> usize {
        for scope in self.scopes.iter().rev() {
            if let Some(addr) = scope.get(name) {
                return *addr;
            }
        }

        panic!("ERRO: variable '{name}' não definida");
    }

    pub fn compile(mut self, program: &'a Program<'a>) -> ByteCode {
        self.compile_block(&program.block);
        self.bytecode.push(OpCode::Halt as u8);

        ByteCode {
            bytecode: self.bytecode,
            constants: self.constants,
        }
    }

    fn compile_block(&mut self, block: &'a Block) {
        self.enter_scope();
        for instr in block.iter_stmts() {
            self.compile_instruction(instr);
        }
        self.exit_scope();
    }

    fn compile_instruction(&mut self, instr: &'a SyntaxTree) {
        match instr {
            SyntaxTree::Assign { idt, exp, .. } => {
                let addr = self.set_var(idt);
                self.compile_expr(exp);
                self.op_store(addr);
            }
            SyntaxTree::SeStmt { exp, blk } => {
                self.compile_expr(exp);
                self.op(OpCode::JmpF); // jump if expression is false

                let jmp_offset_pos = self.bytecode.len(); // offset pos
                self.push_offset(0); // placeholder for jump offset

                let start = self.bytecode.len(); // start of block
                self.compile_block(blk);
                let end = self.bytecode.len(); // end of block

                let jmp_offset = (end - start) as isize; // length of block
                self.insert_offset(jmp_offset, jmp_offset_pos); // jump over the block
            }
            SyntaxTree::EnquantoStmt { exp, blk } => {
                let start = self.bytecode.len(); // start while expression

                self.compile_expr(exp);
                self.op(OpCode::JmpF);

                let jmpf_offset_pos = self.bytecode.len();
                self.push_offset(0); // placeholder for the jump out

                let block_start = self.bytecode.len();
                self.compile_block(blk);
                self.op(OpCode::Jmp);

                let end = self.bytecode.len(); //  end while expression
                let jmp_offset = (end - start) as isize;
                self.push_offset(-jmp_offset); // jmp will go back to expr evaluation

                let end = self.bytecode.len();
                let jmp_offset = (end - block_start) as isize; // this will skip the block and jmp
                self.insert_offset(jmp_offset, jmpf_offset_pos);
            }
            SyntaxTree::ParaStmt {
                idt,
                sta,
                lmt,
                stp,
                blk,
            } => {
                let addr = self.set_var(idt);
                self.compile_literal(sta.as_ref().unwrap_or(&Literal::Inteiro(0)));
                self.op_store(addr);

                let start = self.bytecode.len();

                match lmt.get_type() {
                    Type::Integer | Type::Real => {
                        self.op_load(addr);
                        self.compile_expr(lmt);
                        self.op(OpCode::LE);
                        self.op(OpCode::JmpF);
                    }
                    Type::Boolean => {
                        self.compile_expr(lmt);
                        self.op(OpCode::JmpT);
                    }
                    _ => panic!("ERRO: tipo não permitido como limite"),
                };

                let jmp_offset_pos = self.bytecode.len();
                self.push_offset(0);

                let block_start = self.bytecode.len();
                self.compile_block(blk);

                self.op_load(addr);
                self.compile_literal(stp.as_ref().unwrap_or(&Literal::Inteiro(1)));
                self.op(OpCode::Add);
                self.op_store(addr);

                self.op(OpCode::Jmp);

                let end = self.bytecode.len();
                let jmp_offset = (end - start) as isize;
                self.push_offset(-jmp_offset);

                let end = self.bytecode.len();
                let jmp_offset = (end - block_start) as isize;
                self.insert_offset(jmp_offset, jmp_offset_pos)
            }
            SyntaxTree::Expr(expr) => {
                self.compile_expr(expr);
                if expr.get_type() != Type::Void {
                    self.op(OpCode::Pop);
                }
            }
        }
    }

    fn compile_literal(&mut self, literal: &Literal) {
        let value = match *literal {
            Literal::Decimal(number) => LinaValue::Float32(number),
            Literal::Inteiro(number) => LinaValue::Int32(number),
            Literal::Texto(text) => LinaValue::String(String::from(text)),
            Literal::Booleano(boolean) => LinaValue::Boolean(boolean),
        };

        let find = self.constants.iter().position(|v| *v == value);
        let addr = match find {
            Some(i) => i,
            None => {
                self.constants.push(value);
                self.constants.len() - 1
            }
        };

        self.op_const(addr);
    }

    fn compile_expr(&mut self, expr: &Expression) {
        match expr {
            Expression::Literal(literal) => self.compile_literal(literal),
            Expression::Identifier(idt, ..) => {
                let addr = self.get_var(idt);
                self.op_load(addr);
            }
            Expression::BinOp { ope, lhs, rhs, .. } => {
                // Atrib (:=) does not need a left hand side
                if *ope != Operador::Atrib {
                    self.compile_expr(lhs);
                }
                self.compile_expr(rhs);

                match ope {
                    Operador::MaiorQue => self.op(OpCode::GT),
                    Operador::MenorQue => self.op(OpCode::LT),
                    Operador::MaiorIgualQue => self.op(OpCode::GE),
                    Operador::MenorIgualQue => self.op(OpCode::LE),
                    Operador::Igual => self.op(OpCode::Eq),
                    Operador::Diferente => self.op(OpCode::NE),

                    Operador::E => self.op(OpCode::And),
                    Operador::Ou => self.op(OpCode::Or),

                    Operador::Adic | Operador::AdicAtrib => self.op(OpCode::Add),
                    Operador::Subt | Operador::SubtAtrib => self.op(OpCode::Sub),
                    Operador::Mult | Operador::MultAtrib => self.op(OpCode::Mul),
                    Operador::Div | Operador::DivAtrib => self.op(OpCode::Div),

                    Operador::Resto | Operador::RestoAtrib => self.op(OpCode::Rem),
                    Operador::Exp | Operador::ExpAtrib => todo!(),

                    Operador::Atrib => {}
                };

                if ope.is_atrib() {
                    let Expression::Identifier(idt, _typ) = *lhs.to_owned() else {
                        panic!("ERRO: lado esquerdo de uma atribuição deve ser um identificador");
                    };
                    self.op(OpCode::Dup);
                    let addr = self.get_var(idt);
                    self.op_store(addr);
                }
            }
            Expression::Cast(exp, typ) => {
                self.compile_expr(exp);
                match typ {
                    Type::Integer => self.op(OpCode::CastI),
                    Type::Real => self.op(OpCode::CastF),
                    Type::Text => self.op(OpCode::CastS),
                    _ => panic!("ERRO: nenhuma função de cast para o tipo: {typ}"),
                }
            }
            Expression::Function {
                idt: "saida", arg, ..
            } => {
                for exp in arg {
                    self.compile_expr(exp);
                    self.op(OpCode::Write);
                }
                self.compile_literal(&Literal::Texto("\n"));
                self.op(OpCode::Write);
            }
            Expression::Function {
                idt: "entrada",
                arg,
                ..
            } => {
                for exp in arg {
                    let Expression::Identifier(idt, typ) = exp else {
                        panic!("ERRO: argumento deve ser um identificador")
                    };

                    let addr = self.get_var(idt);
                    match typ {
                        Type::Integer => self.op(OpCode::ReadI),
                        Type::Real => self.op(OpCode::ReadF),
                        Type::Text => self.op(OpCode::ReadL),
                        _ => panic!("ERRO: leitura não suportada para o tipo: {typ}"),
                    }

                    self.op_store(addr);
                }
            }
            Expression::Function { .. } => todo!(),
        }
    }
}

pub fn compile<'a>(program: &'a Program<'a>) -> ByteCode {
    Compiler::new().compile(program)
}
