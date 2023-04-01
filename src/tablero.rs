use crate::pieza::Pieza;

/*
Recibe informaciòn del archivo txt y devuelve vector con piezas encontradas.
Cada una de ellas tendrà asociado su color, posiciòn y tipo.
*/
pub fn obtener_piezas(txt: String) -> Result<Vec<Pieza>, String> {
    let filas: Vec<&str> = txt.lines().collect();
    let mut piezas: Vec<Pieza> = Vec::new();
    let max_filas: usize = 8;
    let max_columnas: usize = 15;

    for (i, fila) in filas.iter().enumerate() {
        // Recorre filas
        if i > max_filas {
            return Err(String::from("Error: Tamaño del tablero excede el indicado"));
        }
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
            }
            if c != ' ' {
                j += 1;
            }
        }
        if j > max_columnas {
            return Err(String::from("Error: Tamaño del tablero excede el indicado"));
        }
    }
    Ok(piezas)
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
    let error = "Error: caracter ingresado no es valido".to_string();
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
