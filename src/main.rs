#![allow(dead_code)]
mod strings;
mod utils;
use strings::correcteur::create_trie_from_word_list;

fn main() {
    let words = vec!["as", "port", "pore", "pre", "pres", "pret"];

    let trie = create_trie_from_word_list(&words);
    println!("{trie:#?}");
}
