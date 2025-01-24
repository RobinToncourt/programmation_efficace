use std::collections::HashMap;

// Temps en O(nk log k) en moyenne et en
// O(n2k log k) dans le pire des cas.
// Avec k=la taille du mot le plus grand et n=le nombre de mots.
pub fn anagram<'a>(words: &[&'a str]) -> Vec<Vec<&'a str>> {
    let mut unique_words: Vec<&'a str> = Vec::new();
    for w in words {
        if !unique_words.contains(w) {
            unique_words.push(w);
        }
    }

    let mut signatures: HashMap<Vec<char>, Vec<&'a str>> = HashMap::new();
    for word in unique_words {
        let mut sorted_chars: Vec<char> = word.chars().collect::<Vec<char>>();
        sorted_chars.sort_unstable();

        if let Some(anagrams) = signatures.get_mut(&sorted_chars) {
            anagrams.push(word);
        } else {
            signatures.insert(sorted_chars, vec![word]);
        }
    }

    let mut result: Vec<Vec<&str>> = Vec::new();
    for (_, anagrams) in signatures {
        if anagrams.len() >= 2 {
            result.push(anagrams);
        }
    }

    result
}

#[cfg(test)]
mod anagram_test {
    use super::*;

    #[test]
    fn test_anagram() {
        let input: Vec<&str> = vec![
            "le", "chien", "marche", "vers", "sa", "niche", "et", "trouve", "une", "limace", "de",
            "chine", "nue", "pleine", "de", "malice", "qui", "fait", "du", "charme",
        ];
        let expected: Vec<Vec<&str>> = vec![
            vec!["une", "nue"],
            vec!["marche", "charme"],
            vec!["chien", "niche", "chine"],
            vec!["limace", "malice"],
        ];

        let result: Vec<Vec<&str>> = anagram(&input);
        for v in expected {
            assert!(result.contains(&v), "{v:?} not found.");
        }
    }
}
