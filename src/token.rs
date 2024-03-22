use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct TokenPos {
    pub col: usize,
    pub row: usize,
}

pub struct TokenDef<'a> {
    pub kind: Token<'a>,
    pub position: TokenPos,
}

#[derive(Debug, PartialEq)]
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
    Identificador(String),
    Valor(Valor<'a>),
    Operador(Operador),
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
            Token::Valor(val) => write!(f, "valor: {val}"),
            Token::Operador(ope) => write!(f, "operador: {ope}"),
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
pub enum Valor<'a> {
    Numero(f32),
    Texto(&'a str),
    Booleano(bool),
    Vetor(&'a [Valor<'a>]),
    Nulo,
}

impl Display for Valor<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Valor::Numero(..) => write!(f, "numero"),
            Valor::Texto(..) => write!(f, "texto"),
            Valor::Booleano(..) => write!(f, "booleano"),
            Valor::Vetor(..) => write!(f, "vetor"),
            Valor::Nulo => write!(f, "nulo"),
        }
    }
}