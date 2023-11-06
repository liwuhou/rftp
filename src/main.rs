use rftp::Cli;

fn main() {
    if let Err(e) = Cli::run() {
        eprintln!("{}", e);
    };
}
