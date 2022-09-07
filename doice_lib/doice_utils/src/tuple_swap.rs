/// Swaps the elements of a 2-size tuple
pub fn tup_swap<A, B>(input: (A, B)) -> (B, A) {
    (input.1, input.0)
}
