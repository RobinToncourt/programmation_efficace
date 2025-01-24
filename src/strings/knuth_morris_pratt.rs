// Complexité de O(m + n), somme des longeurs du mot et du pattern.
fn knuth_morris_pratt(word: &str, pattern: &str) -> Option<usize> {
    todo!()
}

fn naive(word: &str, pattern: &str) -> Option<usize> {
    if word.len() < pattern.len() {
        return None;
    }

    let word_letters: Vec<char> = word.chars().collect();
    let pattern_letters: Vec<char> = pattern.chars().collect();
    println!("{word_letters:?}");

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

    #[test]
    fn test_knuth_morris_pratt() {
        assert_eq!(None, knuth_morris_pratt("bon", "bonjour"));
        assert_eq!(Some(0), knuth_morris_pratt("bonjour", "bon"));
        assert_eq!(Some(3), knuth_morris_pratt("bonjour", "jour"));
        assert_eq!(None, knuth_morris_pratt("bonjour", "bonsoir"));
        assert_eq!(None, knuth_morris_pratt("", "bonjour"));
        assert_eq!(None, knuth_morris_pratt("bonjour", ""));
    }

    #[test]
    fn test_naive() {
        assert_eq!(None, naive("bon", "bonjour"));
        assert_eq!(Some(0), naive("bonjour", "bon"));
        assert_eq!(Some(3), naive("bonjour", "jour"));
        assert_eq!(None, naive("bonjour", "bonsoir"));
        assert_eq!(None, naive("", "bonjour"));
        assert_eq!(Some(0), naive("bonjour", ""));
    }

    #[test]
    fn test_built_in_find() {
        assert_eq!(None, built_in_find("bon", "bonjour"));
        assert_eq!(Some(0), built_in_find("bonjour", "bon"));
        assert_eq!(Some(3), built_in_find("bonjour", "jour"));
        assert_eq!(None, built_in_find("bonjour", "bonsoir"));
        assert_eq!(None, built_in_find("", "bonjour"));
        assert_eq!(Some(0), built_in_find("bonjour", ""));
    }
}
