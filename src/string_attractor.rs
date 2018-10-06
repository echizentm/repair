use repair::RePair;
use std::collections::HashSet;

struct StringAttractor {
    pub indices: HashSet<usize>,
}

impl StringAttractor {
    pub fn new(repair: &RePair) -> StringAttractor {
        let mut string_attractor = StringAttractor {
            indices: HashSet::new(),
        };

        let decoded_text = repair.decode();
        string_attractor.update_indices_by_words(&decoded_text);
        string_attractor.update_indices_by_rules(&decoded_text, repair);

        string_attractor
    }

    pub fn get_indices(&self, subtext: &str, text: &str) -> HashSet<usize> {
        let mut indices = HashSet::new();
        for (start_byte_index, matched_text) in text.match_indices(subtext) {
            let start_index = text[0..start_byte_index].chars().count();
            let end_index = start_index + subtext.chars().count();

            for index in start_index..end_index {
                if self.indices.contains(&index) {
                    indices.insert(index);
                }
            }
        }

        indices
    }

    pub fn print(&self, text: &str) {
        let mut text_to_print = String::new();
        for (index, char) in text.chars().enumerate() {
            if self.indices.contains(&index) {
                text_to_print += &format!("[{}]", char);
            } else {
                text_to_print += &format!("{}", char);
            }
        }
        println!("{}", text_to_print);
    }

    fn update_indices_by_words(&mut self, decoded_text: &str) {
        let mut used_words = HashSet::new();
        for (index, word) in decoded_text.chars().enumerate() {
            if !used_words.contains(&word) {
                self.indices.insert(index);
            }
            used_words.insert(word);
        }
    }

    fn update_indices_by_rules(&mut self, decoded_text: &str, repair: &RePair) {
        for rule in repair.index2rule.values() {
            let decoded_rule = repair.decode_from_rule(rule);
            let mut index = decoded_text.find(&decoded_rule).unwrap();

            index += match repair.index2rule.get(&rule[0]) {
                Some(sub_rule) => repair.decode_from_rule(&sub_rule).len() - 1,
                None => 0,
            };
            self.indices.insert(index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        for text in vec!["abracadabra", "みるみるミルキィ"] {
            let repair = RePair::new(text);
            let string_attractor = StringAttractor::new(&repair);

            string_attractor.print(text);

            let text_chars = text.chars().collect::<Vec<char>>();
            for start_index in 0..text.chars().count() {
                for end_index in (start_index + 1)..(text.chars().count() + 1) {
                    let subtext = &text_chars[start_index..end_index]
                        .iter()
                        .collect::<String>();
                    let index_num = string_attractor.get_indices(subtext, text).len();
                    assert!(index_num > 0);
                }
            }
        }
    }
}
