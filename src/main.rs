mod cli;

fn main() {
    if let Err(error) = cli::run() {
        eprintln!("{}", error);
    }
}
