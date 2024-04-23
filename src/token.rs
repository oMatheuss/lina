use std::fmt::Display;

#[derive(Debug, Clone, Default)]
pub struct TokenPos {
    pub col: usize,
    pub row: usize,
}

#[derive(Debug, Clone)]
pub struct TokenDef<'a> {
    pub kind: Token<'a>,
    pub position: TokenPos,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token<'a> {
    Programa,

    Seja,

    Inteiro,
    Real,
    Texto,
    Booleano,

    Se,
    Entao,

    Enquanto,
    Para,
    Ate,
    Repetir,

    Funcao,
    Retorne,

    Identificador(&'a str),
    Literal(Literal<'a>),
    Operador(Operador),
    Delimitador(Delimitador),

    Fim,
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Programa => write!(f, "programa"),

            Token::Seja => write!(f, "seja"),

            Token::Inteiro => write!(f, ""),
            Token::Real => write!(f, ""),
            Token::Texto => write!(f, ""),
            Token::Booleano => write!(f, ""),
            
            Token::Se => write!(f, "se"),
            Token::Entao => write!(f, "então"),

            Token::Enquanto => write!(f, "enquanto"),
            Token::Para => write!(f, "para"),
            Token::Ate => write!(f, "ate"),
            Token::Repetir => write!(f, "repetir"),
            
            Token::Funcao => write!(f, "função"),
            Token::Retorne => write!(f, "retorne"),

            Token::Identificador(idt) => write!(f, "idetificador: {idt}"),
            Token::Literal(val) => write!(f, "valor: {val}"),
            Token::Operador(ope) => write!(f, "operador: {ope}"),
            Token::Delimitador(del) => write!(f, "delimitador: {del}"),

            Token::Fim => write!(f, "fim"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operador {
    // Operadores Booleanos
    MaiorQue,
    MenorQue,
    MaiorIgualQue,
    MenorIgualQue,
    Igual,
    Diferente,
    E,
    Ou,

    // Operadores Aritméticos
    Adic,
    Subt,
    Mult,
    Div,
    Resto,
    Exp,

    // Atribuições
    Atrib,
    AdicAtrib,
    SubtAtrib,
    MultAtrib,
    DivAtrib,
    RestoAtrib,
    ExpAtrib,
}

pub enum OpAssoc {
    R,
    L,
}
pub struct OpInfo(pub u8, pub OpAssoc);

impl Operador {
    pub fn precedence(&self) -> OpInfo {
        match self {
            Operador::Exp => OpInfo(8, OpAssoc::R),
            Operador::Mult | Operador::Div | Operador::Resto => OpInfo(7, OpAssoc::L),
            Operador::Adic | Operador::Subt => OpInfo(6, OpAssoc::L),

            Operador::MenorQue
            | Operador::MaiorQue
            | Operador::MenorIgualQue
            | Operador::MaiorIgualQue => OpInfo(5, OpAssoc::L),

            Operador::Igual | Operador::Diferente => OpInfo(4, OpAssoc::L),
            Operador::E => OpInfo(3, OpAssoc::L),
            Operador::Ou => OpInfo(2, OpAssoc::L),

            Operador::Atrib
            | Operador::AdicAtrib
            | Operador::SubtAtrib
            | Operador::MultAtrib
            | Operador::DivAtrib
            | Operador::RestoAtrib
            | Operador::ExpAtrib => OpInfo(1, OpAssoc::R),
        }
    }
}

impl Display for Operador {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operador::MaiorQue => write!(f, ">"),
            Operador::MenorQue => write!(f, "<"),
            Operador::MaiorIgualQue => write!(f, ">="),
            Operador::MenorIgualQue => write!(f, "<="),
            Operador::Igual => write!(f, "="),
            Operador::Diferente => write!(f, "<>"),
            Operador::E => write!(f, "e"),
            Operador::Ou => write!(f, "ou"),

            Operador::Adic => write!(f, "+"),
            Operador::Subt => write!(f, "-"),
            Operador::Mult => write!(f, "*"),
            Operador::Div => write!(f, "/"),
            Operador::Resto => write!(f, "%"),
            Operador::Exp => write!(f, "^"),

            Operador::Atrib => write!(f, ":="),
            Operador::AdicAtrib => write!(f, "+="),
            Operador::SubtAtrib => write!(f, "-="),
            Operador::MultAtrib => write!(f, "*="),
            Operador::DivAtrib => write!(f, "/="),
            Operador::RestoAtrib => write!(f, "%="),
            Operador::ExpAtrib => write!(f, "^="),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal<'a> {
    Numero(f32),
    Texto(&'a str),
    Booleano(bool),
    Nulo,
}

impl Display for Literal<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Numero(number) => write!(f, "{}", number),
            Literal::Texto(string) => write!(f, "{}", string),
            Literal::Booleano(boolean) => write!(f, "{}", boolean),
            Literal::Nulo => write!(f, "nulo"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Delimitador {
    AParen,
    FParen,
    AChave,
    FChave,
    AColch,
    FColch,
}

impl Display for Delimitador {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Delimitador::AParen => write!(f, "("),
            Delimitador::FParen => write!(f, ")"),
            Delimitador::AChave => write!(f, "{{"),
            Delimitador::FChave => write!(f, "}}"),
            Delimitador::AColch => write!(f, "["),
            Delimitador::FColch => write!(f, "]"),
        }
    }
}
