use rope::modint::*;

#[test]
fn test_make() {
    let i = ModInt::<21>::new(10);
    assert_eq!(i, 10.into());
}

#[test]
fn test_add_assign() {
    let mut m: ModInt<21> = ModInt::new(10);
    m += 30;
    assert_eq!(m, 19.into());
}
