use colored::*;

const _FUNCTIONS: [&str; 5] = ["hello", "help", "exit","whoami","say"];

fn hello() {
    println!("{}","Hello! :)".bright_green().bold());
}

fn whoami() {
    println!("{}",whoami::username().bright_blue().bold());
}

fn say(args: Vec<&str>) {
    for (_, arg) in args.iter().enumerate() {
        println!("{}", arg.trim().bright_yellow());
    }
}

pub fn welcome() {
    print!("{}", "    Welcome to ".white());
    println!("{}", "[mter]".white().on_bright_magenta().bold());
    println!("    Version: {}", env!("CARGO_PKG_VERSION").bright_blue().bold());
    println!("{}", "    Type 'help' for a list of commands.".bright_black());
}


pub fn handle(input: &str) {
    let trimmed = input.trim();
    let mut parts = trimmed.split_whitespace();
    let cmd = parts.next().unwrap_or("");
    let args: Vec<&str> = parts.collect();

    match cmd {
        "hello" => hello(),

        "help" => {
            println!(
                "{} {:?}",
                "Available commands:".bright_black(),
                _FUNCTIONS
            );
            println!(
                "{} {}",
                "Version:".bright_black(),
                env!("CARGO_PKG_VERSION").bright_green()
            );
        },

        "exit" => {
            println!("{}", "Exiting...".bright_black());
            std::process::exit(0);
        }
        "whoami" => whoami(),

        "say" => {
            if args.is_empty() {
                println!(
                    "{}",
                    "[functions.rs] 'say' functions needs at least one argument".bright_black()
                );
            } else {

                let joined = args.join(" ");
                let parts: Vec<&str> = joined.split('|').collect();
                say(parts);
            }
        }
        _ => println!(
            "{} {} {}",
            "[functions.rs] Unknown command:".bright_black(),
            input,
            "type 'help' for a list of commands.".bright_black()
        ),
    }
}
