use whoami;
use colored::*;

pub fn create_prompt() -> String {
    let current_dir = std::env::current_dir().unwrap();
    let user = whoami::username();

    // Weź ostatnie dwa foldery z current_dir
    let mut parts = current_dir
        .components()
        .rev()
        .filter_map(|c| match c {
            std::path::Component::Normal(os_str) => Some(os_str.to_string_lossy().to_string()),
            _ => None,
        })
        .take(2)
        .collect::<Vec<_>>();

    parts.reverse();

    let short_path = parts.join("/");

    format!(
        "{}@{} {} {}",
        short_path.bright_green().bold(),
        user.bright_blue().bold(),
        "#".bright_magenta().blink(),
        "> ".white()
    )
}