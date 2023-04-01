use std::process::exit;

#[derive(Debug)]
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

#[derive(Debug)]
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
        Coordenadas { x, y }
    }
}

impl Pieza {
    // Creo la pieza con sus caracteristicas: color, tipo y coordenadas
    pub fn crear(letra: &char, x: usize, y: usize) -> Self {
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
                println!("Error: {} no es una letra vàlida", letra);
                exit(0);
            }
        };

        // Asigno color -> Identifico jugador
        let color: Color;
        if letra.is_uppercase() {
            color = Color::Negro;
        } else if letra.is_lowercase() {
            color = Color::Blanco;
        } else {
            println!("Error: Caracter ingresado invàlido. No es posible detectar el color");
            exit(0);
        }

        Pieza {
            color,
            tipo,
            coordenadas,
        }
    }

    pub fn analizar_jugadas(piezas: Vec<Pieza>) -> char {
        print!("Piezas desde analizar jugada: {:?}", piezas);
        'P' //prueba
    }

}
