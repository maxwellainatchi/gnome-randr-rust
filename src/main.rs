mod cli;
mod display_config;

fn main() {
    if let Err(error) = cli::run() {
        eprintln!("{}", error);
    }
}
