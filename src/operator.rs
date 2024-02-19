use std::fmt::Display;

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

pub trait ExecOpe {
    fn exec() -> Result<(), ()>;
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
