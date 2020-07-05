use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Dictionary(HashMap<String, Vec<String>>);

impl Dictionary {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn add_word(&mut self, word: String) {
        let sorted = sort_chars(&word);
        self.0.entry(sorted).or_insert(Vec::new()).push(word);
    }

    fn find_anagrams(&self, word: &str) -> Option<&Vec<String>> {
        let sorted = sort_chars(word);
        self.0.get(&sorted)
    }
}

fn sort_chars(s: &str) -> String {
    let mut chars: Vec<_> = s.chars().collect();
    chars.sort();

    chars.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dictionary_add_word() {
        let mut expected = Dictionary::new();
        expected.0.insert(
            "aet".to_string(),
            vec![
                "ate".to_string(),
                "eat".to_string(),
                "eta".to_string(),
                "tea".to_string(),
            ],
        );
        expected
            .0
            .insert("dorw".to_string(), vec!["word".to_string()]);

        let mut actual = Dictionary::new();

        actual.add_word("ate".to_string());
        actual.add_word("eat".to_string());
        actual.add_word("eta".to_string());
        actual.add_word("tea".to_string());
        actual.add_word("word".to_string());

        assert_eq!(expected, actual);
    }

    #[test]
    fn dictionary_find_anagrams() {
        let expected = vec![
            "ate".to_string(),
            "eat".to_string(),
            "eta".to_string(),
            "tea".to_string(),
        ];

        let mut dict = Dictionary::new();
        dict.add_word("ate".to_string());
        dict.add_word("eat".to_string());
        dict.add_word("eta".to_string());
        dict.add_word("tea".to_string());
        dict.add_word("word".to_string());

        let actual = dict.find_anagrams("eat").unwrap();

        assert_eq!(expected, *actual);
    }

    #[test]
    fn dictionary_find_no_anagrams() {
        let expected = None;

        let mut dict = Dictionary::new();
        dict.add_word("ate".to_string());
        dict.add_word("eat".to_string());
        dict.add_word("eta".to_string());
        dict.add_word("tea".to_string());
        dict.add_word("word".to_string());

        let actual = dict.find_anagrams("aaa");

        assert_eq!(expected, actual);
    }
}
