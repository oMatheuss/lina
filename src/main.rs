use std::env;
use std::fs::File;
use std::io::Read;

use lina::LinaExec;

fn main() {
    let mut code = String::new();

    let args = env::args().collect::<Vec<_>>();
    let file_name = args.get(1).expect("nenhum arquivo especificado");

    let mut file = File::open(file_name).expect("arquivo não encontrado");

    file.read_to_string(&mut code)
        .expect("erro ao ler o arquivo");

    let mut exec = LinaExec {
        path: &file_name,
        source: &code,
        stdout: std::io::stdout(),
    };

    _ = exec.run();
}
