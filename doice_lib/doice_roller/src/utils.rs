pub fn find_parenth(src: &str, needle: char) -> Option<usize> {
    let mut parenth = false;
    src.char_indices()
        .filter(|(_, c)| {
            if *c == ')' {
                parenth = false;
            }
            let par_out = parenth;
            if *c == '(' {
                parenth = true;
            }
            !par_out
        })
        .find(|(_, c)| *c == needle)
        .map(|(i, _)| i)
}

pub fn split_once_parenth(src: &str, at: char) -> Option<(&str, &str)> {
    let index = find_parenth(src, at)?;
    let (first, second) = src.split_at(index);
    Some((first, &second[1..second.len()]))
}
