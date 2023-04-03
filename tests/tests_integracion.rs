use tp1::ajedrez;

#[test]
fn test_gana_color_negro() {
    let archivo = "tablero_ganador_negro.txt".to_string();
    let resultado = ajedrez::iniciar(&archivo).unwrap();
    assert_eq!(resultado, "N");
}

#[test]
fn test_gana_color_blanco() {
    let archivo = "tablero_ganador_blanco.txt".to_string();
    let resultado = ajedrez::iniciar(&archivo).unwrap();
    assert_eq!(resultado, "B");
}

#[test]
fn test_empate() {
    let archivo = "tablero_empate.txt".to_string();
    let resultado = ajedrez::iniciar(&archivo).unwrap();
    assert_eq!(resultado, "E");
}

#[test]
fn test_ambos_pierden() {
    let archivo = "tablero_ambos_pierden.txt".to_string();
    let resultado = ajedrez::iniciar(&archivo).unwrap();
    assert_eq!(resultado, "P");
}

#[test]
fn test_error_cantidad_piezas() {
    let archivo = "tablero_tres_piezas.txt".to_string();
    let resultado = ajedrez::iniciar(&archivo).unwrap_err();
    print!("{}", resultado);
    assert_eq!(
        resultado,
        "Error: La cantidad de piezas dentro del tablero debe ser unicamente 2"
    );
}

#[test]
fn test_error_archivo_inexistente() {
    let archivo = "inexistente.txt".to_string();
    let resultado = ajedrez::iniciar(&archivo).unwrap_err();
    assert_eq!(resultado, "Error: No se pudo leer el archivo");
}

#[test]
fn test_error_piezas_mismo_color() {
    let archivo = "tablero_piezas_mismo_color.txt".to_string();
    let resultado = ajedrez::iniciar(&archivo).unwrap_err();
    assert_eq!(
        resultado,
        "Error: No es vàlido capturar piezas del mismo color"
    );
}

#[test]
fn tes_tablero_mayor_tamaño() {
    let archivo = "tablero_mayor_tamaño.txt".to_string();
    let resultado = ajedrez::iniciar(&archivo).unwrap_err();
    assert_eq!(resultado, "Error: Tamaño del tablero excede el indicado");
}
