use std::any::{Any, TypeId};

pub fn is_number(n: &dyn Any) -> bool {
    TypeId::of::<usize>() == n.type_id()
}

#[test]
fn test_is_usize() {
    let number: usize = 2;
    let string = "Prueba".to_string();
    assert_eq!(is_number(&number), true);
    assert_eq!(is_number(&string), false);
}
