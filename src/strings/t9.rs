use std::collections::HashMap;

const T9: [u8; 26] = [
    2, 2, 2, // a, b, c
    3, 3, 3, // d, e, f
    4, 4, 4, // g, h, i
    5, 5, 5, // j, k, l
    6, 6, 6, // m, n, o
    7, 7, 7, 7, // p, q, r, s
    8, 8, 8, // t, u, v
    9, 9, 9, 9, // w, x, y, z
];
const A_NUMBER: usize = 'a' as usize;

/// Returns the phone digit associated with the char,
/// none if the character is unknown.
fn char_to_phone_digit(c: char) -> Option<&'static u8> {
    T9.get(c.to_ascii_lowercase() as usize - A_NUMBER)
}

/// Returns the phone digit code of the word,
/// or an error if the word contains unknown character.
fn word_to_code(word: &str) -> Result<String, String> {
    let mut result = String::new();

    for c in word.chars() {
        let char_digit: &u8 =
            char_to_phone_digit(c).ok_or("Character not on phone panel: '{c}'")?;
        result.push_str(&char_digit.to_string());
    }

    Ok(result)
}

// O(nk) pour l'initialisation du dictionnaire et
// O(k) pour chaque requête.
// Avec n=le nombre du mot du dictionnaire et k la taille du mot le plus grand.
pub fn predictive_test(dictionnary: HashMap<String, usize>) -> HashMap<String, String> {
    let mut freq: HashMap<String, usize> = HashMap::new();
    for (word, weight) in dictionnary {
        let mut prefixe = String::new();
        for c in word.chars() {
            prefixe.push(c);
            let prefixe_weight: usize = freq.get(&prefixe).unwrap_or(&0) + weight;
            freq.insert(prefixe.clone(), prefixe_weight);
        }
    }

    let mut prop: HashMap<String, String> = HashMap::new();
    for prefixe in freq.keys() {
        if let Ok(code) = word_to_code(prefixe) {
            if !prop.contains_key(&code) || freq[&prop[&code]] < freq[prefixe] {
                prop.insert(code, prefixe.to_string());
            }
        }
    }

    prop
}

fn propose<'a>(prop: &'a HashMap<String, String>, seq: &str) -> Option<&'a str> {
    prop.get(seq).map(std::string::String::as_str)
}

#[cfg(test)]
mod t9_test {
    use std::fs::File;

    use super::*;

    const DICTIONNARY_PATH: &str = "data/t9_dico.json";

    pub fn load_dictionnary() -> anyhow::Result<HashMap<String, usize>> {
        let file: File = File::open(DICTIONNARY_PATH)?;
        let dictionnary: HashMap<String, usize> = serde_json::from_reader(file)?;
        Ok(dictionnary)
    }

    #[test]
    fn test_char_to_phone_digit() {
        let a: char = 'a';
        let o: char = 'o';
        let z: char = 'z';

        let expected_a: Option<&u8> = Some(&2);
        let expected_o: Option<&u8> = Some(&6);
        let expected_z: Option<&u8> = Some(&9);

        let result_a: Option<&u8> = char_to_phone_digit(a);
        let result_o: Option<&u8> = char_to_phone_digit(o);
        let result_z: Option<&u8> = char_to_phone_digit(z);

        assert_eq!(expected_a, result_a);
        assert_eq!(expected_o, result_o);
        assert_eq!(expected_z, result_z);
    }

    #[test]
    fn test_word_to_code() {
        let input: &str = "Bonjour";
        let expected: &str = "2665687";
        let result: String = word_to_code(input).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn test() {
        let dico = load_dictionnary().unwrap();
        let prop = predictive_test(dico);

        let input: &str = "2665687";
        let expected: Option<&str> = Some("bonjour");

        let result: Option<&str> = propose(&prop, &input);
        assert_eq!(expected, result);
    }
}
