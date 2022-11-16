pub fn find_parenth(src: &str, needle: char) -> Option<usize> {
    let mut parenth = 0;
    src.char_indices()
        .filter(|(_, c)| {
            if *c == ')' {
                parenth -= 1;
                parenth = parenth.max(0);
            }
            let par_out = parenth;
            if *c == '(' {
                parenth += 1;
            }
            par_out == 0
        })
        .find(|(_, c)| *c == needle)
        .map(|(i, _)| i)
}

pub fn split_once_parenth(src: &str, at: char) -> Option<(&str, &str)> {
    let index = find_parenth(src, at)?;
    let (first, second) = src.split_at(index);
    Some((first, &second[1..second.len()]))
}
