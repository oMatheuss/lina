use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum Valor {
    Numero(f32),
    Texto(String),
    Booleano(bool),
    Vetor(Vec<Valor>),
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
            Valor::Numero(..) => write!(f, "numero"),
            Valor::Texto(..) => write!(f, "texto"),
            Valor::Booleano(..) => write!(f, "booleano"),
            Valor::Vetor(..) => write!(f, "vetor"),
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
