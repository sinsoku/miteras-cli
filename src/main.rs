extern crate miteras;
use miteras::app;

fn main() {
    let matches = app::build_app().get_matches();
    app::run(matches);
}
