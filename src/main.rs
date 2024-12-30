mod commands;

fn main() {
    commands::init().unwrap_or_else(|_| {
        std::process::exit(1);
    });
}
