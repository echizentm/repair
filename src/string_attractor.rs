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

        string_attractor.get_indexes(repair);

        string_attractor
    }

    fn get_indexes(&mut self, repair: &RePair) {
        let decoded_text = repair.decode();

        let mut used_words = HashSet::new();
        for (index, word) in decoded_text.chars().enumerate() {
            if !used_words.contains(&word) {
                self.indexes.push(index);
            }
            used_words.insert(word);
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
        assert_eq!(string_attractor.indexes.len(), 5);
        assert_eq!(string_attractor.indexes, [0, 1, 2, 4, 6]);
    }
}
