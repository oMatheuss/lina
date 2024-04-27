use std::collections::HashMap;

use crate::syntax::{Block, Expression, Program, SyntaxTree};
use crate::token::{Literal, Operador};
use crate::vm::{self, LinaValue, OpCode};

type VarTable<'a> = HashMap<&'a str, usize>;

#[derive(Debug)]
struct Compiler<'a> {
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
            vi: 0
        }
    }

    fn push_nconst(&mut self, value: f32) -> usize {
        self.constants.push(LinaValue::Number(value));
        self.constants.len() - 1
    }

    fn op_const(&mut self, addr: usize) {
        self.bytecode.push(OpCode::Const as u8);
        self.bytecode.extend(usize::to_ne_bytes(addr));
    }

    fn op_global_store(&mut self, addr: usize) {
        self.bytecode.push(OpCode::GStore as u8);
        self.bytecode.extend(usize::to_ne_bytes(addr));
    }

    fn op_global_load(&mut self, addr: usize) {
        self.bytecode.push(OpCode::GLoad as u8);
        self.bytecode.extend(usize::to_ne_bytes(addr));
    }

    fn push_offset(&mut self, offset: isize) {
        self.bytecode.extend(isize::to_ne_bytes(offset));
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
            if let Some(i) = scope.get(name) {
                return *i
            }
        }

        panic!("ERRO: variable '{name}' não definida");
    }

    pub fn compile(&mut self, program: &'a Program<'a>) {
        self.compile_block(&program.block);
        self.bytecode.push(OpCode::Halt as u8);
    }

    fn compile_block(&mut self, block: &'a Block) {
        self.enter_scope();
        for instr in block.iter() {
            self.compile_instruction(instr);
        }
        self.exit_scope();
    }

    pub fn compile_instruction(&mut self, instr: &'a SyntaxTree) {
        match instr {
            SyntaxTree::Assign { ident, exprs } => {
                match *ident {
                    "saida" => {
                        self.compile_expr(exprs);
                        self.op(OpCode::Write);
                    },
                    _ => {
                        let addr = self.set_var(ident);
                        self.compile_expr(exprs);
                        self.op_global_store(addr);
                    }
                }
            },
            SyntaxTree::SeStmt { expr, block } => {
                self.compile_expr(expr);
                self.op(OpCode::JmpF); // jump if expression is false

                let jmp_offset_pos = self.bytecode.len(); // offset pos
                self.push_offset(0); // placeholder for jump offset

                let start = self.bytecode.len(); // start of block
                self.compile_block(block);
                let end = self.bytecode.len(); // end of block

                let jmp_offset = (end - start) as isize; // length of block
                self.insert_offset(jmp_offset, jmp_offset_pos); // jump over the block
            },
            SyntaxTree::EnquantoStmt { expr, block } => {
                let expr_start = self.bytecode.len(); // start while expression
                self.compile_expr(expr);
                self.op(OpCode::JmpF);

                let jmpf_offset_pos = self.bytecode.len();
                self.push_offset(0); // placeholder for the jump out

                let block_start = self.bytecode.len();
                self.compile_block(block);
                self.op(OpCode::Jmp);

                let jmp_offset_pos = self.bytecode.len();
                self.push_offset(0);
                let end = self.bytecode.len(); //  end while expression
                
                let jmp_offset = (end - expr_start) as isize; // jmp will go back to expr evaluation
                self.insert_offset(-jmp_offset, jmp_offset_pos);

                let end = self.bytecode.len();
                let jmp_offset = (end - block_start) as isize; // this will skip the block and jmp
                self.insert_offset(jmp_offset, jmpf_offset_pos);
            },
            SyntaxTree::ParaStmt { ident, expr, block } => todo!(),
            SyntaxTree::Expr(expr) => {
                self.compile_expr(expr);
            }
        }
    }
    
    pub fn compile_expr(&mut self, exprs: &Expression) {
        match exprs {
            Expression::Literal(literal) => {
                let addr = self.constants.len();

                match *literal {
                    Literal::Numero(number) => {
                        self.push_nconst(number);
                    },
                    Literal::Texto(text) => {
                        self.constants.push(LinaValue::String(String::from(text)));
                    },
                    Literal::Booleano(boolean) => {
                        self.constants.push(LinaValue::Boolean(boolean));
                    },
                    Literal::Nulo => todo!(),
                };

                self.op_const(addr);
            },
            Expression::Identifier(identifier) => {
                let addr = self.get_var(identifier);
                self.op_global_load(addr);
            },
            Expression::BinOp { ope, lhs, rhs } => {
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
                    
                    Operador::E => todo!(),
                    Operador::Ou => todo!(),
                    
                    Operador::Adic | Operador::AdicAtrib => self.op(OpCode::Add),
                    Operador::Subt | Operador::SubtAtrib  => self.op(OpCode::Sub),
                    Operador::Mult | Operador::MultAtrib => self.op(OpCode::Mul),
                    Operador::Div | Operador::DivAtrib => self.op(OpCode::Div),

                    Operador::Resto | Operador::RestoAtrib => todo!(),
                    Operador::Exp | Operador::ExpAtrib => todo!(),

                    Operador::Atrib => {},
                };

                let is_atrib = *ope == Operador::Atrib
                    || *ope == Operador::AdicAtrib
                    || *ope == Operador::SubtAtrib
                    || *ope == Operador::MultAtrib
                    || *ope == Operador::DivAtrib
                    || *ope == Operador::RestoAtrib
                    || *ope == Operador::ExpAtrib;
                
                if is_atrib {
                    let Expression::Identifier(identifier) = *lhs.to_owned() else { unreachable!() };
                    let addr = self.get_var(identifier);
                    self.op_global_store(addr);
                    //self.op_global_load(addr);
                }
            },
        }
    }
}

pub fn execute_program(program: Program) {
    let mut compiler = Compiler::new();
    compiler.compile(&program);

    let mut vm = vm::LinaVm::new(&compiler.bytecode, &compiler.constants);

    vm.run();
}

#[test]
fn test() {
    use crate::{lexer, parser, vm};

    let code = r#"
    seja x := 0
    seja y := 1
    saida := "Calculo de Fibonacci!"
    saida := x
    enquanto x < 10000000 repetir
        seja z := x + y
        x := y
        y := z
        saida := x
    fim
    "#;

    let tokens = lexer::lex(code).unwrap();
    let syntax = parser::parse(tokens).unwrap();

    let mut compiler = Compiler::new();
    compiler.compile(&syntax);

    let mut vm = vm::LinaVm::new(&compiler.bytecode, &compiler.constants);

    vm.run();
}