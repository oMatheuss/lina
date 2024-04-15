use std::collections::HashMap;

use crate::syntax::{Block, Expression, Program, SyntaxTree};
use crate::token::{Literal, Operador};
use crate::vm::{OpCode, LinoValue};

#[derive(Debug)]
struct Compiler<'a> {
    bytecode: Vec<u8>,
    constants: Vec<LinoValue>,
    variables: HashMap<&'a str, usize>,
    vi: usize,
}

impl<'a> Compiler<'a> {
    pub fn new() -> Self {
        Self {
            bytecode: Vec::new(),
            constants: Vec::new(),
            variables: HashMap::new(),
            vi: 0
        }
    }

    pub fn compile(&mut self, program: Program<'a>) {
        self.compile_block(&program.block);
        self.bytecode.push(OpCode::Halt as u8);
    }

    fn compile_block(&mut self, block: &Block<'a>) {
        for instr in block.iter() {
            match instr {
                SyntaxTree::Assign { ident, exprs } => {
                    self.variables.insert(ident, self.vi);
                    self.compile_expr(exprs);
                    self.bytecode.push(OpCode::Store as u8);
                    self.bytecode.push(self.vi as u8);
                    self.vi += 1;
                },
                SyntaxTree::SeStmt { expr, block } => {
                    self.compile_expr(expr);
                    self.bytecode.push(OpCode::JumpIfFalse as u8);

                    self.bytecode.push(0u8);
                    let jmp_start = self.bytecode.len();

                    self.compile_block(block);

                    let jmp_end = self.bytecode.len();
                    let jmp_offset = jmp_end - jmp_start;
                    assert!(jmp_offset <= 127);
                    self.bytecode[jmp_start - 1] = jmp_offset as u8;
                },
                SyntaxTree::EnquantoStmt { expr, block } => {
                    let expr_start = self.bytecode.len();
                    self.compile_expr(expr);
                    self.bytecode.push(OpCode::JumpIfFalse as u8);

                    self.bytecode.push(0u8);
                    let jmp_start = self.bytecode.len();

                    self.compile_block(block);

                    self.bytecode.push(OpCode::Jump as u8);
                    let expr_end = self.bytecode.len();
                    let jmp_offset = expr_end - expr_start + 1; // +1 do parametro offset
                    assert!(jmp_offset <= 127);
                    self.bytecode.push(-(jmp_offset as i8) as u8);

                    let jmp_end = self.bytecode.len();
                    let jmp_offset = jmp_end - jmp_start;
                    assert!(jmp_offset <= 127);
                    self.bytecode[jmp_start - 1] = jmp_offset as u8;
                },
                SyntaxTree::ParaStmt { ident, expr, block } => todo!(),
                SyntaxTree::Expr(expr) => {
                    self.compile_expr(expr);
                }
            }
        }
    }
    
    pub fn compile_expr(&mut self, exprs: &Expression) {
        match exprs {
            Expression::Literal(literal) => {
                let address = self.constants.len();

                match *literal {
                    Literal::Numero(number) => {
                        self.constants.push(LinoValue::Number(number));
                    },
                    Literal::Texto(text) => {
                        self.constants.push(LinoValue::String(String::from(text)));
                    },
                    Literal::Booleano(boolean) => {
                        self.constants.push(LinoValue::Bool(boolean));
                    },
                    Literal::Vetor(_) => todo!(),
                    Literal::Nulo => todo!(),
                };

                self.bytecode.push(OpCode::Const as u8);
                self.bytecode.push(address as u8);
            },
            Expression::Identifier(identifier) => {
                let address = self.variables[identifier];
                self.bytecode.push(OpCode::Load as u8);
                self.bytecode.push(address as u8);
            },
            Expression::BinOp { ope, lhs, rhs } => {
                if *ope != Operador::Atrib {
                    self.compile_expr(lhs);
                }
                self.compile_expr(rhs);

                match ope {
                    Operador::MaiorQue => {
                        self.bytecode.push(OpCode::GreaterThan as u8);
                    },
                    Operador::MenorQue => {
                        self.bytecode.push(OpCode::LessThan as u8);
                    },
                    Operador::MaiorIgualQue => todo!(),
                    Operador::MenorIgualQue => todo!(),
                    Operador::Igual => {
                        self.bytecode.push(OpCode::Equal as u8);
                    },
                    Operador::Diferente => {
                        self.bytecode.push(OpCode::NotEqual as u8);
                    },
                    Operador::E => todo!(),
                    Operador::Ou => todo!(),
                    Operador::Adic | Operador::AdicAtrib => {
                        self.bytecode.push(OpCode::Add as u8);
                    },
                    Operador::Subt | Operador::SubtAtrib  => {
                        self.bytecode.push(OpCode::Sub as u8);
                    },
                    Operador::Mult | Operador::MultAtrib => {
                        self.bytecode.push(OpCode::Mul as u8);
                    },
                    Operador::Div | Operador::DivAtrib => {
                        self.bytecode.push(OpCode::Div as u8);
                    },
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
                    let address = self.variables[identifier];
                    self.bytecode.push(OpCode::Store as u8);
                    self.bytecode.push(address as u8);
                    self.bytecode.push(OpCode::Load as u8);
                    self.bytecode.push(address as u8);
                }
            },
        }
    }
}

#[test]
fn test() {
    use crate::{lexer, parser, vm};

    let code = r#"
    seja x := 0
    seja y := 1
    enquanto x < 3 repetir
        seja z := x + y
        seja x := y
        seja y := z
    fim
    "#;

    let tokens = lexer::lex(code).unwrap();
    let syntax = parser::parse(tokens).unwrap();

    let mut compiler = Compiler::new();
    compiler.compile(syntax);

    println!("{:?}", compiler.bytecode);

    let mut vm = vm::LinoVm::new(&compiler.bytecode, compiler.constants);

    //vm.run();
    //vm.debug();
}