// Complexité de O(m + n), somme des longeurs du mot et du pattern.
fn knuth_morris_pratt(word: &str, pattern: &str) -> Option<usize> {
    todo!()
}

fn knuth_morris_pratt_personnal(word: &str, pattern: &str) -> Option<usize> {
    todo!()
}

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
        assert_eq!(None, function("bon", "bonjour"));
        assert_eq!(Some(0), function("bonjour", "bon"));
        assert_eq!(Some(3), function("bonjour", "jour"));
        assert_eq!(None, function("bonjour", "bonsoir"));
        assert_eq!(None, function("", "bonjour"));
        assert_eq!(Some(0), function("bonjour", ""));
    }

    #[test]
    fn test_knuth_morris_pratt() {
        compare_tester(&knuth_morris_pratt);
    }

    #[test]
    fn test_knuth_morris_pratt_personnal() {
        compare_tester(&knuth_morris_pratt_personnal);
    }

    #[test]
    fn test_naive() {
        compare_tester(&naive);
    }

    #[test]
    fn test_built_in_find() {
        compare_tester(&built_in_find);
    }
}
