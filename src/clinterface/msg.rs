use console::Term;

pub fn display_msg(msg: &'static str, term: &mut Term) {
    println!("{}\n[Press any key to continue]", msg);
    term.read_key().unwrap();
    term.clear_screen().unwrap();
}