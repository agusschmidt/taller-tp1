use std::{env, io};
use std::fs::File;
use std::io::Read;

/*enum Pieza {
    Rey,
    Dama,
    Alfil,
    Caballo,
    Torre,
    Peon,
}

fn puede_capturar(pieza: Pieza) -> u8 {
    match pieza {
        Pieza::Rey => 1,
        Pieza::Dama => 5,
        Pieza::Alfil => 10,
        Pieza::Caballo => 25,
        Pieza::Torre => 25,
        Pieza::Peon => 25,
    }
}*/

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 { 
        println!("Error: Debe ingresar el nombre del archivo para generar el tablero");
        return;
    }
    else if args.len() > 2 { 
        println!("Error: El programa solo admite un argumento");
        return;
    }

    let txt_arg = &args[1];
    match leer_archivo(txt_arg) {
        Ok(contenido) => {
            let tablero = generar_tablero(contenido);
            imprimir_tablero(&tablero);
        },
        Err(error) => {
            eprintln!("Error al leer el archivo: {}", error);
        }
    }
    
}

fn leer_archivo(nombre_txt: &str) -> Result<String, io::Error> {
    match File::open(nombre_txt) {
        Ok(mut archivo) => {
            let mut contenido = String::new();
            archivo.read_to_string(&mut contenido)?;
            Ok(contenido)
        },
        Err(error) => {
            Err(error)
        }
    }
}

fn generar_tablero(txt: String) -> [[char; 8]; 8]{
    let filas: Vec<&str> = txt.lines().collect();
    let mut tablero = [[' '; 8]; 8];

    for (i, fila) in filas.iter().enumerate() {
        let mut j = 0;
        for c in fila.chars() {
            if c != ' ' {
                tablero[i][j] = c;
                j += 1;
            } 
        }
    }

    tablero
}

fn imprimir_tablero(tablero: &[[char; 8]; 8]){
    for fila in tablero {
        for columna in fila {
            print!("{}", columna);
        }
        println!();
    }
}
