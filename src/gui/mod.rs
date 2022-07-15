pub mod indicator;
pub mod menu;

use std::io::Write;
use std::process::Command;

fn handler(mut command: Command, origin: &str) {
    match command.output() {
        Ok(res) => {
            std::io::stdout().write_all(&res.stdout).unwrap_or_default();
            std::io::stderr().write_all(&res.stderr).unwrap_or_default();
        }
        Err(err) => eprintln!("Error with {}: {}", origin, err),
    }
}
