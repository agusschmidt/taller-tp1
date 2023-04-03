use crate::archivo;
use crate::pieza::Pieza;

/// Recibe el nombre del archivo quee contiene el tablero. 
/// Busca las piezas, las crea y luego analiza las jugadas para devolver el resultado o error en caso de haber alguno.
pub fn iniciar(txt_arg: &String) -> Result<String, String> {
    match archivo::leer_archivo(txt_arg) {
        Ok(contenido) => match self::obtener_piezas(contenido) {
            Ok(piezas) => {
                let piezas: Vec<Pieza> = piezas;
                if piezas.len() != 2 {
                    return Err(String::from(
                        "Error: La cantidad de piezas dentro del tablero debe ser unicamente 2",
                    ));
                }
                match Pieza::analizar_jugadas(piezas) {
                    Ok(res) => {
                        let resultado = self::definir_resultado(res);
                        Ok(resultado.to_string())
                    }
                    Err(error) => {
                        return Err(error);
                    }
                }
            }
            Err(error) => {
                return Err(error);
            }
        },
        Err(_error) => {
            return Err(String::from("Error: No se pudo leer el archivo"));
        }
    }
}

///Recibe informaciòn del archivo txt y devuelve vector con piezas encontradas.
///Cada una de ellas tendrà asociado su color, posiciòn y tipo.
pub fn obtener_piezas(txt: String) -> Result<Vec<Pieza>, String> {
    let filas: Vec<&str> = txt.lines().collect();
    let mut piezas: Vec<Pieza> = Vec::new();
    let max_filas: usize = 8;
    let max_columnas: usize = 8;

    if filas.len() != max_filas {
        return Err(String::from(
            "Error: Tamaño del tablero excede el indicado",
        ));
    }

    for (i, fila) in filas.iter().enumerate() {
        // Recorre filas
        let mut j = 0;
        for c in fila.chars() {
            // Recorre columnas
            if c != ' ' && c != '_' {
                match Pieza::crear(&c, j, i) {
                    Ok(pieza) => {
                        piezas.push(pieza);
                        j += 1;
                    }
                    Err(err) => {
                        return Err(err);
                    }
                };
            } else if c != ' ' {
                j += 1;
            }

            if j > max_columnas {
                return Err(String::from(
                    "Error: Tamaño del tablero excede el indicado",
                ));
            }
        }
    }
    // Devuelvo vector con las piezas creadas
    Ok(piezas)
}

/// Define resultado final en base al resultado de cada jugador
pub fn definir_resultado(resultados: Vec<bool>) -> char {
    if resultados[0] && resultados[1] {
        'E'
    } else if !resultados[0] && !resultados[1] {
        'P'
    } else if resultados[0] && !resultados[1] {
        'B'
    } else {
        'N'
    }
}

#[test]
fn test_obtener_piezas_tablero_distinto_tamaño() {
    let error = "Error: Tamaño del tablero excede el indicado".to_string();

    let contenido = "_ _ _ _ _ _ _ _ _ _ _
                            _ _ _ _ _ _ _ _ _ _ _
                            _ _ _ _ _ _ P _ _ _ _
                            _ _ _ _ _ _ _ _ _ _ _
                            _ _ _ _ _ _ _ _ _ _ _
                            _ _ _ _ _ _ _ _ _ _ _
                            _ _ a _ _ _ _ _ _ _ _
                            _ _ _ _ _ _ _ _ _ _ _
                            _ _ _ _ _ _ _ _ _ _ _ "
        .to_string();
    assert_eq!(obtener_piezas(contenido).unwrap_err(), error);
}

#[test]
fn test_obtener_piezas_con_pieza_numerica_muestra_error() {
    let error = "Error: Caracter ingresado no es valido".to_string();
    let contenido = "_ _ _ _ _ _ _ _
    _ _ _ _ _ _ _ _
    _ _ _ _ _ _ 8 _
    _ _ _ _ _ _ _ _
    _ _ _ _ _ _ _ _
    _ _ _ _ _ _ _ _
    _ _ a _ _ _ _ _
    _ _ _ _ _ _ _ _"
        .to_string();
    assert_eq!(obtener_piezas(contenido).unwrap_err(), error);
}
