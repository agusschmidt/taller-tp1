use std::fs::File;
use std::io::{self, Read};

///Abre el archivo txt. Retorna contenido o error en caso de no poder abrirlo.
pub fn leer_archivo(nombre_txt: &str) -> Result<String, io::Error> {
    match File::open(nombre_txt) {
        Ok(mut archivo) => {
            let mut contenido = String::new();
            archivo.read_to_string(&mut contenido)?;
            Ok(contenido)
        }
        Err(error) => Err(error),
    }
}

#[test]
fn test_leer_archivo_inexistente() {
    let archivo = leer_archivo("nombre.txt");
    assert!(archivo.is_err());
}

#[test]
fn test_leer_archivo_existente() {
    let archivo = leer_archivo("tablero.txt");
    assert!(archivo.is_ok())
}
