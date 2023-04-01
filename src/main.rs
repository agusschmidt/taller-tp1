mod archivo;
mod tablero;

mod pieza;
use crate::pieza::Pieza;

mod formato;

use std::env;
use std::process::exit;

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
    match archivo::leer_archivo(txt_arg) {
        Ok(contenido) => match tablero::obtener_piezas(contenido) {
            Ok(piezas) => {
                let piezas: Vec<Pieza> = piezas;
                if piezas.len() != 2 {
                    eprintln!(
                        "Error: La cantidad de piezas dentro del tablero debe ser unicamente 2"
                    );
                    exit(0);
                }
                Pieza::analizar_jugadas(piezas);
            }
            Err(error) => {
                eprintln!("{}", error);
                exit(0);
            }
        },
        Err(error) => {
            eprintln!("Error: No se pudo leer el archivo: {}", error);
            exit(0);
        }
    }
}
