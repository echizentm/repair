extern crate repair;
use repair::repair::RePair;
use repair::string_attractor::StringAttractor;
use std::io;

fn main() {
    loop {
        println!("input text to indicate string attractors:");

        let mut input_text = String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("cannot read line.");
        let input_text = input_text.trim();

        let repair = RePair::new(input_text);
        let string_attractor = StringAttractor::new(&repair);

        println!("text with string attractors:");
        string_attractor.print(input_text);
        println!("");
    }
}
