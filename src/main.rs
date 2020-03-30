extern crate miteras;
use miteras::app;
use std::io;

fn main() {
    let matches = app::build_app().get_matches();
    app::run(matches, io::stdout());
}
