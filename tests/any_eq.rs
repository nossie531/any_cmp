use any_cmp::prelude::*;

#[test]
fn test() {
    let x = &42 as &dyn AnyEq;
    let y = &42 as &dyn AnyEq;
    let z = &"42" as &dyn AnyEq;
    assert!(x == y);
    assert!(x != z);
}
