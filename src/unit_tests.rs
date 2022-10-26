use crate::vector::Vector;

#[test]
fn vector_addition() {
    let a = Vector::from_vec(vec![1.0, 5.6, 5.5, 6.6]);
    let b = Vector::from_vec(vec![5.7, 6.6, 9.9, 8.0]);
    
    let intended_result = Vector::from_vec(vec![6.7, 12.2, 15.4, 14.6]);
    let result = a + b;
    
    assert_eq!(intended_result, result);
}