use colored::*;

const _FUNCTIONS: [&str; 3] = ["hello", "help", "exit"];

fn hello() {
    println!("Hello!");
}

pub fn welcome() {
    print!("{}", "    Welcome to ".white());
    println!("{}", "[mter]".white().on_bright_magenta().bold());
    println!("{}", "    Type 'help' for a list of commands.".bright_black());
}


pub fn handle(input: &str) {
    if _FUNCTIONS.contains(&input) {
        match input {
            "hello" => hello(),

            "help" => println!(
                "{} {:?}",
                "Available commands:".bright_black(),
                _FUNCTIONS
            ),

            "exit" => {
                println!("{}", "Exiting...".bright_black());
                std::process::exit(0);
            }

            _ => println!(
                "{} {}",
                "Unknown command:".bright_black(),
                input
            ),
        }
        return;
    }

    println!(
        "{} {} {}",
        "[functions.rs] Unknown command:".bright_black(),
        input,
        "type 'help' for a list of commands.".bright_black()
    );
}
