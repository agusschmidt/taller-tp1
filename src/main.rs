mod pieza;
use crate::pieza::Pieza;

use std::fs::File;
use std::io::Read;
use std::process::exit;
use std::{env, io};

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
    match leer_archivo(txt_arg) {
        Ok(contenido) => {
            let piezas: Vec<Pieza> = obtener_piezas(contenido);
            if piezas.len() != 2 {
                eprintln!("Error: La cantidad de piezas dentro del tablero debe ser unicamente 2");
                exit(0);
            }
            Pieza::analizar_jugadas(piezas);
        }
        Err(error) => {
            eprintln!("Error: No se pudo leer el archivo: {}", error);
            exit(0);
        }
    }
}

/*
Abre el archivo txt. Retorna contenido o error en caso de no poder abrirlo.
*/
fn leer_archivo(nombre_txt: &str) -> Result<String, io::Error> {
    match File::open(nombre_txt) {
        Ok(mut archivo) => {
            let mut contenido = String::new();
            archivo.read_to_string(&mut contenido)?;
            Ok(contenido)
        }
        Err(error) => Err(error),
    }
}

/*
Recibe informaciòn del archivo txt y devuelve vector con piezas encontradas.
Cada una de ellas tendrà asociado su color, posiciòn y tipo.
*/
fn obtener_piezas(txt: String) -> Vec<Pieza> {
    let filas: Vec<&str> = txt.lines().collect();
    let mut piezas: Vec<Pieza> = Vec::new();

    for (i, fila) in filas.iter().enumerate() { // Recorre filas
        let mut j = 0;
        for c in fila.chars() { // Recorre columnas
            if c != ' ' && c != '_' {
                let pieza = Pieza::crear(&c, j, i);
                piezas.push(pieza);
                j += 1;
            }
            if c != ' '{
                j += 1;
            }
        }
    }
    piezas
}

