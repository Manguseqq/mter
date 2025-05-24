use std::io::{self, Write};
mod prompt;

mod functions;

fn main() {
    functions::welcome();

    fn handle_input(input: &str) {
        functions::handle(input);
    }

    loop {
        let mut input = String::new();

        print!("{}", prompt::create_prompt());
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let trimmed_input = input.trim();

        handle_input(trimmed_input);
    }
}
