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
    Seja,
    Faca,
    Entao,
    Imprima,
    Enquanto,
    Se,
    Funcao,
    Para,
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
            Token::Seja => write!(f, "seja"),
            Token::Faca => write!(f, "faça"),
            Token::Entao => write!(f, "então"),
            Token::Imprima => write!(f, "imprima"),
            Token::Enquanto => write!(f, "enquanto"),
            Token::Se => write!(f, "se"),
            Token::Funcao => write!(f, "função"),
            Token::Para => write!(f, "para"),
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
    MaiorQue,
    MenorQue,
    MaiorIgualQue,
    MenorIgualQue,
    Igual,
    CondicionalE,
    CondicionalOu,

    Atribuicao,
    Adicao,
    Subtracao,
    Multiplicacao,
    Divisao,
    Resto,

    SomaAtribuicao,
    SubtracaoAtribuicao,
    MultiplicacaoAtribuicao,
    DivisaoAtribuicao,
    RestoAtribuicao,
    AutoAdicao,
    AutoSubtracao,
}

impl Display for Operador {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operador::MaiorQue => write!(f, ">"),
            Operador::MenorQue => write!(f, "<"),
            Operador::MaiorIgualQue => write!(f, ">="),
            Operador::MenorIgualQue => write!(f, "<="),
            Operador::Igual => write!(f, "=="),
            Operador::CondicionalE => write!(f, "&&"),
            Operador::CondicionalOu => write!(f, "||"),

            Operador::Atribuicao => write!(f, "="),
            Operador::Adicao => write!(f, "+"),
            Operador::Subtracao => write!(f, "-"),
            Operador::Multiplicacao => write!(f, "*"),
            Operador::Divisao => write!(f, "/"),
            Operador::Resto => write!(f, "%"),

            Operador::SomaAtribuicao => write!(f, "+="),
            Operador::SubtracaoAtribuicao => write!(f, "-="),
            Operador::MultiplicacaoAtribuicao => write!(f, "*="),
            Operador::DivisaoAtribuicao => write!(f, "/="),
            Operador::RestoAtribuicao => write!(f, "%="),
            Operador::AutoAdicao => write!(f, "++"),
            Operador::AutoSubtracao => write!(f, "--"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal<'a> {
    Numero(f32),
    Texto(&'a str),
    Booleano(bool),
    Vetor(&'a [Literal<'a>]),
    Nulo,
}

impl Display for Literal<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Numero(..) => write!(f, "numero"),
            Literal::Texto(..) => write!(f, "texto"),
            Literal::Booleano(..) => write!(f, "booleano"),
            Literal::Vetor(..) => write!(f, "vetor"),
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