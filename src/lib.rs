//! # RePair
//!
//! `repair` is a data structure encodes text with grammer compression.
use std::collections::HashMap;

/// Implementation of RePair.
pub struct RePair {
    rule_length: usize,
    index2word: HashMap<usize, char>,
    index2rule: HashMap<usize, Vec<usize>>,
}

impl RePair {
    pub fn new(text: &str) -> RePair {
        let mut re_pair = RePair {
            rule_length: 2,
            index2word: HashMap::new(),
            index2rule: HashMap::new(),
        };
        let mut ids = re_pair.make_index2word(text);
        while ids.len() >= re_pair.rule_length {
            ids = re_pair.make_index2rule(&ids);
        }

        re_pair
    }

    /// Decode text.
    ///
    /// # Examples
    ///
    /// ```
    /// use repair::RePair;
    ///
    /// let re_pair = RePair::new("abracadabra");
    ///
    /// assert_eq!(re_pair.decode(), "abracadabra".to_string());
    /// ```
    pub fn decode(&self) -> String {
        let mut ids = Vec::new();
        ids.extend(self.index2rule[&self.head_rule_id()].iter().clone());
        loop {
            let mut is_decoded = false;
            let mut next_ids = Vec::new();
            for parent_id in ids {
                match self.index2rule.get(&parent_id) {
                    Some(child_ids) => {
                        for child_id in child_ids.iter() {
                            next_ids.push(*child_id);
                        }
                        is_decoded = true;
                    }
                    None => next_ids.push(parent_id),
                }
            }
            ids = next_ids;

            if !is_decoded {
                break;
            }
        }

        let mut decoded = String::new();
        for id in ids {
            decoded.push(self.index2word[&id]);
        }

        decoded
    }

    fn make_index2word(&mut self, text: &str) -> Vec<usize> {
        let mut ids = Vec::new();
        let mut word2index = HashMap::new();

        for word in text.chars() {
            let next_index = word2index.len();
            word2index.entry(word).or_insert(next_index);

            let index = word2index[&word];
            self.index2word.entry(index).or_insert(word);

            ids.push(index);
        }

        ids
    }

    fn make_index2rule(&mut self, ids: &Vec<usize>) -> Vec<usize> {
        let mut counter = HashMap::new();

        for i in 0..(ids.len() - self.rule_length + 1) {
            let key = RePair::ids2key(&ids[i..i + self.rule_length]);
            let count = *counter.get(&key).unwrap_or(&0);
            counter.insert(key, count + 1);
        }

        let mut keys = counter.keys().collect::<Vec<&String>>();
        keys.sort_unstable_by(|a, b| counter[*b].cmp(&counter[*a]));
        let rule_key = keys[0];
        let rule_id = self.next_rule_id();
        self.index2rule.insert(rule_id, RePair::key2ids(rule_key));

        let mut skip_count = 0;
        let mut encoded_ids = Vec::new();
        for i in 0..ids.len() {
            if skip_count > 0 {
                skip_count -= 1;
                continue;
            }

            if i >= ids.len() - self.rule_length + 1 {
                encoded_ids.push(ids[i]);
                continue;
            }

            let key = RePair::ids2key(&ids[i..i + self.rule_length]);

            if key == *rule_key {
                encoded_ids.push(rule_id);
                skip_count = self.rule_length - 1;
            } else {
                encoded_ids.push(ids[i]);
            }
        }

        encoded_ids
    }

    fn next_rule_id(&self) -> usize {
        self.index2word.len() + self.index2rule.len()
    }

    fn head_rule_id(&self) -> usize {
        self.next_rule_id() - 1
    }

    fn ids2key(ids: &[usize]) -> String {
        let mut key = String::from("");
        for id in ids {
            key += " ";
            key += &id.to_string();
        }

        key
    }

    fn key2ids(key: &str) -> Vec<usize> {
        key.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<usize>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        for text in vec!["abracadabra", "みるみるミルキィ"] {
            let re_pair = RePair::new(text);
            assert_eq!(re_pair.decode(), text.to_string());
        }
    }
}
