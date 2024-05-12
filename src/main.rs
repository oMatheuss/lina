use std::env;
use std::fs::File;
use std::io::Read;

use lina::run_code;

fn main() {
    let mut code = String::new();

    let args = env::args().collect::<Vec<_>>();
    let file_name = args.get(1).expect("nenhum arquivo especificado");

    let mut file = File::open(file_name).expect("Arquivo não encontrado");

    file.read_to_string(&mut code)
        .expect("Erro ao ler o arquivo");

    let mut stdout = std::io::stdout();

    run_code(&code, &mut stdout).expect("programa ser executado");
}
