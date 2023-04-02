use crate::formato;

#[derive(Debug, PartialEq)]
pub struct Coordenadas {
    pub x: usize,
    pub y: usize,
}

impl Coordenadas {
    pub fn posicionar(x: usize, y: usize) -> Result<Self, String> {
        if formato::is_number(&x) && x < 8 && formato::is_number(&y) && y < 8 {
            Ok(Coordenadas { x, y })
        } else {
            Err(String::from("Error: Coordenadas invalidas"))
        }
    }
}

#[test]
fn test_crear_coordenadas_validas() {
    assert_eq!(
        Coordenadas::posicionar(2, 5).unwrap(),
        Coordenadas { x: 2, y: 5 }
    );
}

#[test]
fn test_crear_coordenadas_invalidas() {
    let error = "Error: Coordenadas invalidas".to_string();
    assert_eq!(Coordenadas::posicionar(9, 8).unwrap_err(), error);
}
