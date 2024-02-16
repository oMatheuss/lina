use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Token {
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
    Valor(Valor),
    Operador(Operador),
    Fim,
    FimDoArquivo,
}

impl Display for Token {
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
            Token::FimDoArquivo => write!(f, "eof"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Valor {
    Numero(f32),
    Texto(String),
    Booleano(bool),
    Vetor(Vec<Valor>),
    // TODO: implement objects
    // Object(Map<String, Value>),
    Nulo,
}

impl Valor {
    pub fn variant_eq(&self, b: &Valor) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(b)
    }
}

impl Display for Valor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Valor::Numero(v) => write!(f, "{v}"),
            Valor::Texto(v) => write!(f, "{v}"),
            Valor::Booleano(v) => write!(f, "{v}"),
            Valor::Vetor(items) => {
                write!(f, "[")?;
                for item in items {
                    write!(f, "{item}, ")?;
                }
                write!(f, "]")
            }
            Valor::Nulo => write!(f, "nulo"),
        }
    }
}

impl From<f32> for Valor {
    fn from(value: f32) -> Self {
        Self::Numero(value)
    }
}

impl From<String> for Valor {
    fn from(value: String) -> Self {
        Self::Texto(value)
    }
}

impl From<bool> for Valor {
    fn from(value: bool) -> Self {
        Self::Booleano(value)
    }
}

#[derive(Debug, PartialEq)]
pub enum Operador {
    MaiorQue,
    MenorQue,
    MaiorIgualQue,
    MenorIgualQue,
    Igual,
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
