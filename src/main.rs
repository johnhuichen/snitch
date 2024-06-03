use std::process;

use snitch;

fn main() {
    if let Err(e) = snitch::run() {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
