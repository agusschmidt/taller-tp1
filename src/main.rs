use std::env;
use std::process::exit;

use tp1::ajedrez;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Error: Debe ingresar el nombre del archivo para generar el tablero");
        exit(0);
    } else if args.len() > 2 {
        println!("Error: El programa solo admite un argumento");
        exit(0);
    }

    let txt_arg = &args[1];
    match ajedrez::iniciar(txt_arg) {
        Ok(resultado) => {
            println!("{}", resultado);
            exit(0);
        }
        Err(error) => {
            println!("{}", error);
            exit(0);
        }
    }
}
