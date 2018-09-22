use repair::RePair;
use std::collections::HashSet;

struct StringAttractor {
    pub indexes: Vec<usize>,
}

impl StringAttractor {
    pub fn new(repair: &RePair) -> StringAttractor {
        let mut string_attractor = StringAttractor {
            indexes: Vec::new(),
        };

        let decoded_text = repair.decode();
        string_attractor.get_indexes_from_words(&decoded_text, repair);
        string_attractor.get_indexes_from_rules(&decoded_text, repair);

        string_attractor
    }

    fn get_indexes_from_words(&mut self, decoded_text: &str, repair: &RePair) {
        let mut used_words = HashSet::new();
        for (index, word) in decoded_text.chars().enumerate() {
            if !used_words.contains(&word) {
                self.indexes.push(index);
            }
            used_words.insert(word);
        }
    }

    fn get_indexes_from_rules(&mut self, decoded_text: &str, repair: &RePair) {
        for rule in repair.index2rule.values() {
            let decoded_rule = repair.decode_from_rule(rule);
            let mut index = decoded_text.find(&decoded_rule).unwrap();

            index += match repair.index2rule.get(&rule[0]) {
                Some(sub_rule) => repair.decode_from_rule(&sub_rule).len() - 1,
                None => 0,
            };
            self.indexes.push(index);
            println!("{}: {}", decoded_rule, index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let text = "abracadabra";
        let repair = RePair::new(text);
        let string_attractor = StringAttractor::new(&repair);
        assert_eq!(string_attractor.indexes.len(), 12);
    }
}
