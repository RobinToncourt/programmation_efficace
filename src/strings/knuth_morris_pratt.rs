// Complexité de O(m + n), somme des longeurs du mot et du pattern.
pub fn knuth_morris_pratt(word: &str, pattern: &str) -> Option<usize> {
    if pattern.is_empty() {
        return Some(0);
    }

    let word_letters: Vec<char> = word.chars().collect();
    let pattern_letters: Vec<char> = pattern.chars().collect();

    let mut r: Vec<isize> = vec![0; pattern.len()];
    r[0] = -1;
    let mut j: isize = -1;

    for i in 1..pattern.len() {
        println!("{r:?}");

        while j >= 0 && pattern_letters[i - 1] != pattern_letters[j as usize] {
            j = r[j as usize];
        }
        j += 1;
        r[i] = j;
    }

    println!("{r:?}");

    j = 0;
    for (i, c) in word_letters.iter().enumerate() {
        println!("j: {j}");

        while j >= 0 && *c != pattern_letters[j as usize] {
            j = r[j as usize];
        }
        j += 1;
        if j == pattern.len() as isize {
            return Some(i + 1 - pattern.len())
        }
    }

    None
}

enum PatternState {
    Continue,
    Invalid,
    Match,
}

struct PatternMatching<'a, T> {
    list_index: usize,
    pattern_letters: &'a [T],
    pattern_index: usize,
}

use std::cmp::Eq;
impl<'a, T> PatternMatching<'a, T>
where
    T: Eq,
{
    fn new(list_index: usize, pattern_letters: &'a [T]) -> Self {
        Self {
            list_index,
            pattern_letters,
            pattern_index: 1,
        }
    }

    fn check(&mut self, item: &T) -> PatternState {
        if self.pattern_letters[self.pattern_index].eq(item) {
            self.pattern_index += 1;
            if self.pattern_index == self.pattern_letters.len() {
                PatternState::Match
            } else {
                PatternState::Continue
            }
        } else {
            PatternState::Invalid
        }
    }
}

fn one_loop(word: &str, pattern: &str) -> Option<usize> {
    if pattern.is_empty() {
        return Some(0);
    }

    let word_letters: Vec<char> = word.chars().collect();
    let pattern_letters: Vec<char> = pattern.chars().collect();

    let mut pml: Vec<PatternMatching<char>> = Vec::new();

    for (i, c) in word_letters.iter().enumerate() {
        let mut new_pml: Vec<PatternMatching<char>> = Vec::new();
        for mut pm in pml.into_iter() {
            match pm.check(c) {
                PatternState::Continue => {new_pml.push(pm)},
                PatternState::Match => return Some(pm.list_index),
                PatternState::Invalid => {},
            }
        }
        pml = new_pml;

        if i <= word_letters.len().saturating_sub(pattern_letters.len()) && *c == pattern_letters[0] {
            let pattern_matching = PatternMatching::new(i, &pattern_letters);
            pml.push(pattern_matching);
        }
    }

    None
}

fn one_loop_generic<T>(list: &[T], pattern: &[T]) -> Option<usize>
where
    T: Eq,
{
    if pattern.is_empty() {
        return Some(0);
    }

    let mut pml: Vec<PatternMatching<T>> = Vec::new();

    for (i, item) in list.iter().enumerate() {
        let mut new_pml: Vec<PatternMatching<T>> = Vec::new();
        for mut pm in pml.into_iter() {
            match pm.check(item) {
                PatternState::Continue => {new_pml.push(pm)},
                PatternState::Match => return Some(pm.list_index),
                PatternState::Invalid => {},
            }
        }
        pml = new_pml;

        if i <= list.len().saturating_sub(pattern.len()) && item.eq(&pattern[0]) {
            let pattern_matching = PatternMatching::new(i, pattern);
            pml.push(pattern_matching);
        }
    }

    None
}

#[allow(clippy::manual_find)]
fn naive(word: &str, pattern: &str) -> Option<usize> {
    if word.len() < pattern.len() {
        return None;
    }

    let word_letters: Vec<char> = word.chars().collect();
    let pattern_letters: Vec<char> = pattern.chars().collect();

    for i in 0..=word_letters.len() - pattern.len() {
        if word_letters[i..i + pattern.len()] == pattern_letters {
            return Some(i);
        }
    }

    None
}

fn built_in_find(word: &str, pattern: &str) -> Option<usize> {
    word.find(pattern)
}

#[cfg(test)]
mod knuth_morris_pratt_test {
    use super::*;

    fn compare_tester(function: &dyn Fn(&str, &str) -> Option<usize>) {
        assert_eq!(Some(6), function("lalopalalali", "lala"));
        assert_eq!(None, function("bon", "bonjour"));
        assert_eq!(Some(0), function("bonjour", "bon"));
        assert_eq!(Some(3), function("bonjour", "jour"));
        assert_eq!(None, function("bonjour", "bonsoir"));
        assert_eq!(None, function("", "bonjour"));
        assert_eq!(Some(0), function("bonjour", ""));
        assert_eq!(Some(0), function("", ""));
        assert_eq!(Some(6), function("lalalalalali", "lalali"));
    }

    #[test]
    fn test_knuth_morris_pratt() {
        compare_tester(&knuth_morris_pratt);
    }

    #[test]
    fn test_one_loop() {
        compare_tester(&one_loop);
    }

    #[test]
    fn test_naive() {
        compare_tester(&naive);
    }

    #[test]
    fn test_built_in_find() {
        compare_tester(&built_in_find);
    }

    fn ts(s: &str) -> Vec<char> {
        s.chars().collect::<Vec<char>>()
    }

    #[test]
    fn test_one_loop_generic() {
        assert_eq!(Some(6), one_loop_generic(&ts("lalopalalali"), &ts("lala")));
        assert_eq!(None, one_loop_generic(&ts("bon"), &ts("bonjour")));
        assert_eq!(Some(0), one_loop_generic(&ts("bonjour"), &ts("bon")));
        assert_eq!(Some(3), one_loop_generic(&ts("bonjour"), &ts("jour")));
        assert_eq!(None, one_loop_generic(&ts("bonjour"), &ts("bonsoir")));
        assert_eq!(None, one_loop_generic(&ts(""), &ts("bonjour")));
        assert_eq!(Some(0), one_loop_generic(&ts("bonjour"), &ts("")));
        assert_eq!(Some(0), one_loop_generic(&ts(""), &ts("")));
        assert_eq!(Some(6), one_loop_generic(&ts("lalalalalali"), &ts("lalali")));
    }
}
