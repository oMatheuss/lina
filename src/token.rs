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
    Numero(i32),
    Operador(String),
    Incremento,
    AtribuicaoIncremento,
    Decremento,
    AtribuicaoDecremento,
    Fim,
	FimDoArquivo
}
