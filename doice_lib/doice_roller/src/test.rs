use super::*;

#[test]
fn parenth_test() {
    let src = "(2*d2-3)*stat()";
    dbg!(Roll::from_str(src).unwrap());
}
