use std::process::exit;

use crate::formato;

#[derive(Debug, PartialEq)]
pub struct Coordenadas {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
pub enum TipoPieza {
    Rey,
    Dama,
    Alfil,
    Caballo,
    Torre,
    Peon,
}

#[derive(Debug, PartialEq)]
pub enum Color {
    Blanco,
    Negro,
}

#[derive(Debug)]
pub struct Pieza {
    pub coordenadas: Coordenadas,
    pub tipo: TipoPieza,
    pub color: Color,
}

impl Coordenadas {
    pub fn posicionar(x: usize, y: usize) -> Self {
        if formato::is_number(&x) && formato::is_number(&y) {
            Coordenadas { x, y }
        } else {
            println!("Formato de coordenadas invàlido");
            exit(0)
        }
    }
}

impl Pieza {
    // Creo la pieza con sus caracteristicas: color, tipo y coordenadas
    pub fn crear(letra: &char, x: usize, y: usize) -> Result<Self, String> {
        // Aisgno posiciòn del tablero
        let coordenadas: Coordenadas = Coordenadas::posicionar(x, y);

        // Asigno tipo de pieza segùn la letra
        let tipo: TipoPieza = match letra {
            'r' | 'R' => TipoPieza::Rey,
            'd' | 'D' => TipoPieza::Dama,
            'a' | 'A' => TipoPieza::Alfil,
            'c' | 'C' => TipoPieza::Caballo,
            't' | 'T' => TipoPieza::Torre,
            'p' | 'P' => TipoPieza::Peon,
            _ => {
                return Err(String::from("Error: caracter ingresado no es valido"));
            }
        };

        // Asigno color -> Identifico jugador
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

    pub fn analizar_jugadas(piezas: Vec<Pieza>) -> Vec<bool> {
        let pieza_blanca: &Pieza;
        let pieza_negra: &Pieza;
        if piezas[0].color == Color::Blanco {
            pieza_blanca = &piezas[0];
            pieza_negra = &piezas[1];
        } else {
            pieza_blanca = &piezas[1];
            pieza_negra = &piezas[0];
        }
        println!("Pieza blanca: {:?}\n", pieza_blanca);
        println!("Pieza negra: {:?}\n", pieza_negra);

        let puede_atacar_pieza_blanca: bool = Self::puede_atacar(pieza_blanca, pieza_negra);
        let puede_atacar_pieza_negra: bool = Self::puede_atacar(pieza_negra, pieza_blanca);

        let resultados: Vec<bool> = vec![puede_atacar_pieza_blanca, puede_atacar_pieza_negra];
        println!("Resultados: {:?}\n", resultados);

        resultados
    }

    /*
    Pre: Ambas piezas tienen datos vàlidos.
    Recibe pieza de ambos jugadores y analiza si el primero puede atacar al segundo.
    Para esto, primero filtra por tipo para identificar movimiento y luego por color para identificar sentido.
    */
    fn puede_atacar(pieza_actual: &Pieza, pieza_contrincante: &Pieza) -> bool {
        let puede_atacar: bool = match pieza_actual.tipo {
            TipoPieza::Rey => Self::puede_atacar_rey(pieza_actual, pieza_contrincante),
            TipoPieza::Dama => Self::puede_atacar_dama(pieza_actual, pieza_contrincante),
            TipoPieza::Alfil => Self::puede_atacar_alfil(pieza_actual, pieza_contrincante),
            TipoPieza::Caballo => Self::puede_atacar_caballo(pieza_actual, pieza_contrincante),
            TipoPieza::Torre => Self::puede_atacar_torre(pieza_actual, pieza_contrincante),
            TipoPieza::Peon => Self::puede_atacar_peon(pieza_actual, pieza_contrincante),
        };
        puede_atacar
    }

    fn puede_atacar_rey(pieza_actual: &Pieza, pieza_contrincante: &Pieza) -> bool {
        true
    }

    fn puede_atacar_dama(pieza_actual: &Pieza, pieza_contrincante: &Pieza) -> bool {
        true
    }

    fn puede_atacar_alfil(pieza_actual: &Pieza, pieza_contrincante: &Pieza) -> bool {
        true
    }

    fn puede_atacar_caballo(pieza_actual: &Pieza, pieza_contrincante: &Pieza) -> bool {
        true
    }

    fn puede_atacar_torre(pieza_actual: &Pieza, pieza_contrincante: &Pieza) -> bool {
        true
    }

    fn puede_atacar_peon(pieza_actual: &Pieza, pieza_contrincante: &Pieza) -> bool {
        true
    }
}

#[test]
fn test_coordenadas_validas() {
    assert_eq!(Coordenadas::posicionar(2, 5), Coordenadas { x: 2, y: 5 });
}
