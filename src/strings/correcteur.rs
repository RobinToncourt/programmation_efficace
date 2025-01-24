use std::collections::BTreeMap;

use crate::utils::get_mut_or_default_btree_map;

#[derive(Default, Debug)]
pub struct TrieNode {
    is_word: bool,
    links: BTreeMap<char, TrieNode>,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            is_word: false,
            links: BTreeMap::new(),
        }
    }

    fn add_word(&mut self, word: &str) {
        self.add_word_at(word, 0);
    }

    fn add_word_at(&mut self, word: &str, index: usize) {
        if index == word.len() {
            self.is_word = true;
        } else {
            let letter: char = word.chars().nth(index).unwrap();

            let trie_node_link: &mut TrieNode =
                get_mut_or_default_btree_map(&mut self.links, &letter);

            trie_node_link.add_word_at(word, index + 1);
        }
    }
}

pub fn create_trie_from_word_list(word_list: &[&str]) -> TrieNode {
    let mut trie_node = TrieNode::new();

    for word in word_list {
        trie_node.add_word(word);
    }

    trie_node
}

fn spell_check(trie: &TrieNode, word: &str) -> String {
    let mut distance: isize = 0;

    if word.is_empty() {
        return String::new();
    }

    loop {
        match search(Some(trie), distance, word, 0) {
            Some(result) => return result,
            None => distance += 1,
        }
    }
}

fn search(trie: Option<&TrieNode>, distance: isize, word: &str, i: usize) -> Option<String> {
    if i == word.len() {
        if trie.is_some() && trie.unwrap().is_word && distance == 0 {
            return Some(String::new());
        }

        return None;
    }

    let trie: &TrieNode = trie?;

    let letter: char = word.chars().nth(i).unwrap();
    let f: Option<String> = search(trie.links.get(&letter), distance, word, i + 1);

    if f.is_some() {
        let mut result = letter.to_string();
        result.push_str(&f.unwrap());
        return Some(result);
    }

    if distance == 0 {
        return None;
    }

    for (c, sub_trie_node) in &trie.links {
        let f: Option<String> = search(Some(sub_trie_node), distance - 1, word, i);
        if f.is_some() {
            let mut result = c.to_string();
            result.push_str(&f.unwrap());
            return Some(result);
        }

        let f: Option<String> = search(Some(sub_trie_node), distance - 1, word, i + 1);
        if f.is_some() {
            let mut result = c.to_string();
            result.push_str(&f.unwrap());
            return Some(result);
        }
    }

    search(Some(trie), distance - 1, word, i + 1)
}

#[cfg(test)]
mod correcteur_test {
    use super::*;

    #[test]
    fn test_spell_check() {
        let word_list = vec!["as", "port", "pore", "pre", "pres", "pret"];
        let trie: TrieNode = create_trie_from_word_list(&word_list);

        assert_eq!("".to_string(), spell_check(&trie, ""));
        assert_eq!("pres".to_string(), spell_check(&trie, "pres"));
        assert_eq!("pres".to_string(), spell_check(&trie, "prez"));
        assert_eq!("pore".to_string(), spell_check(&trie, "bjeurg"));
    }
}
