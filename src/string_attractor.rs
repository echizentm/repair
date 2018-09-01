use repair::RePair;

struct StringAttractor {
    words: Vec<usize>,
}

impl StringAttractor {
    pub fn new(repair: &RePair) -> StringAttractor {
        StringAttractor { words: Vec::new() }
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
        assert_eq!(string_attractor.words.len(), 0);
    }
}
