// Complexité de O(m + n), somme des longeurs du mot et du pattern.
fn knuth_morris_pratt(word: &str, pattern: &str) -> Option<usize> {
    if pattern.is_empty() {
        return Some(0);
    }

    let word_letters: Vec<char> = word.chars().collect();
    let pattern_letters: Vec<char> = pattern.chars().collect();

    let mut r: Vec<isize> = vec![0; pattern.len()];
    r[0] = -1;
    let mut j: isize = -1;

    for i in 1..pattern.len() {
        while j >= 0 && pattern_letters[i - 1] != pattern_letters[j as usize] {
            j = r[j as usize];
        }
        j += 1;
        r[i] = j;
    }
    j = 0;
    for (i, c) in word_letters.iter().enumerate() {
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

fn one_loop(word: &str, pattern: &str) -> Option<usize> {
    if word.len() < pattern.len() {
        return None;
    } else if pattern.is_empty() {
        return Some(0);
    }

    let word_letters: Vec<char> = word.chars().collect();
    let pattern_letters: Vec<char> = pattern.chars().collect();

    let mut result: usize = 0;
    let mut pattern_i: usize = 0;
    let mut first_match: bool = true;
    for (i, c) in word_letters.iter().enumerate() {
        if *c == pattern_letters[pattern_i] {
            if first_match {
                result = i;
                first_match = false;
            }

            pattern_i += 1;
            if pattern_i == pattern_letters.len() {
                return Some(result);
            }
        } else {
            result = i;
            pattern_i = 0;
            first_match = true;
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
        assert_eq!(Some(2), function("lalalali", "lalali"));
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
}
