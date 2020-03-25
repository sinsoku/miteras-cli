extern crate miteras;
use miteras::cli;

fn main() {
    let matches = cli::build_app().get_matches();
    cli::run(matches);
}
