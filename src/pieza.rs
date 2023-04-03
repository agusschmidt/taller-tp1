use crate::color::Color;
use crate::coordenada::Coordenadas;
use crate::tipo_pieza::TipoPieza;

/// Representa la Pieza: color, tipo y posicion en el tablero
#[derive(Debug, PartialEq)]
pub struct Pieza {
    pub coordenadas: Coordenadas,
    pub tipo: TipoPieza,
    pub color: Color,
}

impl Pieza {
    /// Creo la pieza con sus caracteristicas: color, tipo y coordenadas
    pub fn crear(letra: &char, x: usize, y: usize) -> Result<Self, String> {
        let coordenadas: Coordenadas = match Coordenadas::posicionar(x, y) {
            Ok(pos) => pos,
            Err(err) => return Err(err),
        };

        let tipo: TipoPieza = match letra {
            'r' | 'R' => TipoPieza::Rey,
            'd' | 'D' => TipoPieza::Dama,
            'a' | 'A' => TipoPieza::Alfil,
            'c' | 'C' => TipoPieza::Caballo,
            't' | 'T' => TipoPieza::Torre,
            'p' | 'P' => TipoPieza::Peon,
            _ => {
                return Err(String::from("Error: Caracter ingresado no es valido"));
            }
        };

        let color: Color = if letra.is_uppercase() {
            Color::Negro
        } else {
            Color::Blanco
        };

        Ok(Pieza {
            color,
            tipo,
            coordenadas,
        })
    }

    /// Analiza la juagada para la pieza de cada color. Retorna un vector con ambos resultados, en la posicion 0 irà el resultado para color blanco y en la pos 1 para el color negro
    pub fn analizar_jugadas(piezas: Vec<Pieza>) -> Result<Vec<bool>, String> {
        let pieza_blanca: &Pieza;
        let pieza_negra: &Pieza;
        if piezas[0].color == Color::Blanco {
            pieza_blanca = &piezas[0];
            pieza_negra = &piezas[1];
        } else {
            pieza_blanca = &piezas[1];
            pieza_negra = &piezas[0];
        }

        let puede_atacar_pieza_blanca: bool = match Self::puede_atacar(pieza_blanca, pieza_negra) {
            Ok(res) => res,
            Err(err) => {
                return Err(err);
            }
        };

        let puede_atacar_pieza_negra: bool = match Self::puede_atacar(pieza_negra, pieza_blanca) {
            Ok(res) => res,
            Err(err) => {
                return Err(err);
            }
        };

        let resultados: Vec<bool> = vec![puede_atacar_pieza_blanca, puede_atacar_pieza_negra];
        Ok(resultados)
    }

    ///Pre: Cada una de las piezas tiene datos vàlidos. Recibe pieza de ambos jugadores y analiza si el primero puede atacar al segundo. Para esto, primero filtra por tipo para identificar movimiento y luego por color para identificar sentido en caso de ser necesario.
    pub fn puede_atacar(pieza_actual: &Pieza, pieza_contrincante: &Pieza) -> Result<bool, String> {
        if pieza_actual.color == pieza_contrincante.color {
            return Err(String::from(
                "Error: No es vàlido capturar piezas del mismo color",
            ));
        }
        let distancia_x =
            (pieza_actual.coordenadas.x as i8 - pieza_contrincante.coordenadas.x as i8).abs();
        let distancia_y =
            (pieza_actual.coordenadas.y as i8 - pieza_contrincante.coordenadas.y as i8).abs();

        let puede_atacar: bool = match pieza_actual.tipo {
            TipoPieza::Rey => Self::puede_atacar_rey(distancia_x, distancia_y),
            TipoPieza::Alfil => Self::puede_atacar_alfil(distancia_x, distancia_y),
            TipoPieza::Caballo => Self::puede_atacar_caballo(distancia_x, distancia_y),
            TipoPieza::Dama => Self::puede_atacar_dama(distancia_x, distancia_y),
            TipoPieza::Torre => Self::puede_atacar_torre(distancia_x, distancia_y),
            TipoPieza::Peon => Self::puede_atacar_peon(pieza_actual, pieza_contrincante),
        };
        Ok(puede_atacar)
    }

    ///Movimiento del rey: un casillero a la vez, en cualquier direccion y sentido
    fn puede_atacar_rey(distancia_x: i8, distancia_y: i8) -> bool {
        ((distancia_x == 0 || distancia_x == 1) && distancia_y == 1)
            || (distancia_x == 1 && distancia_y == 0)
    }

    ///Movimiento de la dama: cuantos casilleros desee, en cualquier direcciòn y sentido
    fn puede_atacar_dama(distancia_x: i8, distancia_y: i8) -> bool {
        let ataque_diag_habilitado: bool = Self::puede_atacar_diagonal(distancia_x, distancia_y);
        let ataque_recto_habilitado: bool = Self::puede_atacar_vh(distancia_x, distancia_y);

        ataque_diag_habilitado || ataque_recto_habilitado
    }

    ///Movimiento del alfil: cuantos casilleros desee, de forma diagonal y en cualquier sentido
    fn puede_atacar_alfil(distancia_x: i8, distancia_y: i8) -> bool {
        Self::puede_atacar_diagonal(distancia_x, distancia_y)
    }

    ///Movimiento del caballo: avanza dos casilleros en vertical y uno horizontal, o viceversa en cualquier sentido
    fn puede_atacar_caballo(distancia_x: i8, distancia_y: i8) -> bool {
        distancia_x == 1 && distancia_y == 2 || distancia_x == 2 && distancia_y == 1
    }

    ///Movimiento de la torre: Avanza cuantos casilleros desee en linea recta y cualquier sentido
    fn puede_atacar_torre(distancia_x: i8, distancia_y: i8) -> bool {
        Self::puede_atacar_vh(distancia_x, distancia_y)
    }

    ///Movimiento depende del color pues no puede retroceder.
    ///Avanzan en linea recta y 1 casillero a la vez.
    ///Caso color blanco: Se posicionan abajo por lo que avanzan hacia arriba (hacia los negativos de la matriz).
    ///Caso color negro: Opuesto al blanco.
    fn puede_atacar_peon(pieza_actual: &Pieza, pieza_contrincante: &Pieza) -> bool {
        let sentido_movimiento = if pieza_actual.color == Color::Blanco {
            1
        } else {
            -1
        };
        let distancia_x =
            (pieza_actual.coordenadas.x as i8 - pieza_contrincante.coordenadas.x as i8).abs();
        let distancia_y = pieza_actual.coordenadas.y as i8 - pieza_contrincante.coordenadas.y as i8;

        distancia_y == sentido_movimiento && distancia_x == 1
    }

    ///Analiza movimiento en caso de que sea diagonal
    fn puede_atacar_diagonal(distancia_x: i8, distancia_y: i8) -> bool {
        distancia_x == distancia_y
    }

    ///Analiza movimiento en caso de que sea vertical u horizontal
    fn puede_atacar_vh(distancia_x: i8, distancia_y: i8) -> bool {
        distancia_x == 0 || distancia_y == 0
    }
}

#[test]
fn test_crear_pieza_blanca_caracter_invalido() {
    let error = "Error: Caracter ingresado no es valido".to_string();
    let caracter = 'q';
    assert_eq!(Pieza::crear(&caracter, 2, 5).unwrap_err(), error);
}

#[test]
fn test_crear_pieza_negra_caracter_invalido() {
    let error = "Error: Caracter ingresado no es valido".to_string();
    let caracter = 'L';
    assert_eq!(Pieza::crear(&caracter, 2, 5).unwrap_err(), error);
}

#[test]
fn test_crear_pieza_coordenadas_invalidas() {
    let error = "Error: Coordenadas invalidas".to_string();
    let caracter = 'r';
    assert_eq!(Pieza::crear(&caracter, 10, 5).unwrap_err(), error);
}

#[test]
fn test_crear_pieza_blanca() {
    let caracter = 'd';
    assert_eq!(
        Pieza::crear(&caracter, 2, 5).unwrap(),
        Pieza {
            color: Color::Blanco,
            tipo: TipoPieza::Dama,
            coordenadas: Coordenadas { x: (2), y: (5) }
        }
    );
}

#[test]
fn test_crear_pieza_negra() {
    let caracter = 'A';
    assert_eq!(
        Pieza::crear(&caracter, 2, 5).unwrap(),
        Pieza {
            color: Color::Negro,
            tipo: TipoPieza::Alfil,
            coordenadas: Coordenadas { x: (2), y: (5) }
        }
    );
}

#[test]
fn test_puede_atacar_rey() {
    assert_eq!(Pieza::puede_atacar_rey(1, 1), true);
    assert_eq!(Pieza::puede_atacar_rey(1, 2), false);
    assert_eq!(Pieza::puede_atacar_rey(9, 2), false);
    assert_eq!(Pieza::puede_atacar_rey(0, 2), false);
}

#[test]
fn test_puede_atacar_dama() {
    assert_eq!(Pieza::puede_atacar_dama(1, 1), true);
    assert_eq!(Pieza::puede_atacar_dama(5, 5), true);
    assert_eq!(Pieza::puede_atacar_dama(9, 2), false);
    assert_eq!(Pieza::puede_atacar_dama(0, 2), true);
    assert_eq!(Pieza::puede_atacar_dama(6, 0), true);
}

#[test]
fn test_puede_atacar_alfil() {
    assert_eq!(Pieza::puede_atacar_alfil(3, 1), false);
    assert_eq!(Pieza::puede_atacar_alfil(5, 5), true);
    assert_eq!(Pieza::puede_atacar_alfil(15, 4), false);
    assert_eq!(Pieza::puede_atacar_alfil(0, 4), false);
}

#[test]
fn test_puede_atacar_caballo() {
    assert_eq!(Pieza::puede_atacar_caballo(2, 1), true);
    assert_eq!(Pieza::puede_atacar_caballo(1, 2), true);
    assert_eq!(Pieza::puede_atacar_caballo(5, 5), false);
    assert_eq!(Pieza::puede_atacar_caballo(0, 4), false);
}

#[test]
fn test_puede_atacar_torre() {
    assert_eq!(Pieza::puede_atacar_torre(2, 1), false);
    assert_eq!(Pieza::puede_atacar_torre(1, 2), false);
    assert_eq!(Pieza::puede_atacar_torre(5, 0), true);
    assert_eq!(Pieza::puede_atacar_torre(0, 4), true);
}

#[test]
fn test_puede_atacar_peon_blanco_devuelve_true() {
    let atacante = Pieza {
        color: Color::Blanco,
        tipo: TipoPieza::Peon,
        coordenadas: Coordenadas { x: (2), y: (6) },
    };
    let atacado = Pieza {
        color: Color::Negro,
        tipo: TipoPieza::Torre,
        coordenadas: Coordenadas { x: (1), y: (5) },
    };
    assert_eq!(Pieza::puede_atacar_peon(&atacante, &atacado), true);
}

#[test]
fn test_puede_atacar_peon_negro_devuelve_true() {
    let atacante = Pieza {
        color: Color::Negro,
        tipo: TipoPieza::Peon,
        coordenadas: Coordenadas { x: (6), y: (1) },
    };
    let atacado = Pieza {
        color: Color::Blanco,
        tipo: TipoPieza::Torre,
        coordenadas: Coordenadas { x: (7), y: (2) },
    };
    assert_eq!(Pieza::puede_atacar_peon(&atacante, &atacado), true);
}

#[test]
fn test_puede_atacar_peon_blanco_devuelve_false() {
    let atacante = Pieza {
        color: Color::Blanco,
        tipo: TipoPieza::Peon,
        coordenadas: Coordenadas { x: (2), y: (6) },
    };
    let atacado = Pieza {
        color: Color::Negro,
        tipo: TipoPieza::Alfil,
        coordenadas: Coordenadas { x: (2), y: (5) },
    };
    assert_eq!(Pieza::puede_atacar_peon(&atacante, &atacado), false);
}

#[test]
fn test_ataque_piezas_mismo_color_devuelve_error() {
    let error = "Error: No es vàlido capturar piezas del mismo color".to_string();
    let atacante = Pieza {
        color: Color::Blanco,
        tipo: TipoPieza::Peon,
        coordenadas: Coordenadas { x: (2), y: (6) },
    };
    let atacado = Pieza {
        color: Color::Blanco,
        tipo: TipoPieza::Alfil,
        coordenadas: Coordenadas { x: (2), y: (5) },
    };
    assert_eq!(Pieza::puede_atacar(&atacante, &atacado).unwrap_err(), error);
}

#[test]
fn test_ataque_piezas() {
    let atacante = Pieza {
        color: Color::Blanco,
        tipo: TipoPieza::Peon,
        coordenadas: Coordenadas { x: (2), y: (6) },
    };
    let atacado = Pieza {
        color: Color::Negro,
        tipo: TipoPieza::Alfil,
        coordenadas: Coordenadas { x: (1), y: (5) },
    };
    assert_eq!(Pieza::puede_atacar(&atacante, &atacado).unwrap(), true);
}

#[test]
fn test_analizar_jugadas_devuelve_true_ambos_colores() {
    let atacante = Pieza {
        color: Color::Blanco,
        tipo: TipoPieza::Peon,
        coordenadas: Coordenadas { x: (2), y: (6) },
    };
    let atacado = Pieza {
        color: Color::Negro,
        tipo: TipoPieza::Alfil,
        coordenadas: Coordenadas { x: (1), y: (5) },
    };
    let piezas = vec![atacante, atacado];
    let resultados = vec![true, true];
    assert_eq!(Pieza::analizar_jugadas(piezas).unwrap(), resultados);
}

#[test]
fn test_analizar_jugadas_devuelve_true_blanca_false_negra() {
    let atacante = Pieza {
        color: Color::Blanco,
        tipo: TipoPieza::Peon,
        coordenadas: Coordenadas { x: (2), y: (6) },
    };
    let atacado = Pieza {
        color: Color::Negro,
        tipo: TipoPieza::Torre,
        coordenadas: Coordenadas { x: (1), y: (5) },
    };
    let piezas = vec![atacante, atacado];
    let resultados = vec![true, false];
    assert_eq!(Pieza::analizar_jugadas(piezas).unwrap(), resultados);
}
