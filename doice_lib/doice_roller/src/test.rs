use super::*;

#[test]
fn parenth_test() {
    let src = "(2*d2-3)*stat()";
    dbg!(Roll::from_str(src).unwrap());
}

#[test]
fn sum_test() {
    let src = "sum(atk(2d12, 16, +15, 20,) ,3)";
    dbg!(Roll::from_str(src).unwrap());
}

#[test]
fn multi_parenth_test() {
    let src = "((dick(3, 10)+5)*dick(3, 20))*(2*d2-3)";
    dbg!(Roll::from_str(src).unwrap());
}
