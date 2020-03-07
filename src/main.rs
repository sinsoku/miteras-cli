#[macro_use]
extern crate clap;

extern crate chrono;
extern crate rpassword;
extern crate serde;
extern crate serde_json;
extern crate toml;

mod api;
mod cli;
mod config;

fn main() {
    let matches = cli::build_app().get_matches();

    if let Some(_matches) = matches.subcommand_matches("login") {
        cli::login();
    }

    if let Some(matches) = matches.subcommand_matches("clock-in") {
        cli::clock_in(matches);
    }

    if let Some(matches) = matches.subcommand_matches("clock-out") {
        cli::clock_out(matches);
    }
}
