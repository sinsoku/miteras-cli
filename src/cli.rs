use crate::api::Api;
use crate::config::Config;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use rpassword::read_password;
use std::collections::HashMap;
use std::io::{stdin, stdout, Write};

pub fn build_app() -> App<'static, 'static> {
    app_from_crate!()
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("login").about("Authenticate to MITERAS and save credentials"),
        )
        .subcommand(
            SubCommand::with_name("clock-in")
                .about("Clock in with today's condition")
                .arg(
                    Arg::from_usage("[condition] 'Specify your condition'")
                        .takes_value(true)
                        .possible_values(&["best", "good", "normal", "bad"])
                        .default_value("good"),
                ),
        )
        .subcommand(
            SubCommand::with_name("clock-out")
                .about("Clock out with today's condition")
                .arg(
                    Arg::from_usage("[condition] 'Specify your condition'")
                        .takes_value(true)
                        .possible_values(&["best", "good", "normal", "bad"])
                        .default_value("good"),
                ),
        )
}

fn read_input(label: &str, hidden: bool) -> String {
    print!("{}: ", label);
    stdout().flush().unwrap();

    if hidden {
        read_password().unwrap()
    } else {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }
}

pub fn login() {
    println!("Try logging in to MITERAS.\n");

    let org = read_input("Org", false);
    let username = read_input("Username", false);
    let password = read_input("Password", true);
    let config = Config::new(org, username, password);
    let api = Api::new(&config);
    let res = api.login().unwrap();

    if res.url().path().ends_with("/cico") {
        config.save();
        println!("\nLogin successful.");
    } else {
        println!("\nLogin failed.");
    }
}

pub fn clock_in(matches: &ArgMatches) {
    let condition = matches.value_of("condition").unwrap();
    let config = Config::load().unwrap();
    let api = Api::new(&config);

    let res = api.clock_in(condition).unwrap();
    println!("{}", res.text().unwrap());
}

pub fn clock_out(matches: &ArgMatches) {
    let condition = matches.value_of("condition").unwrap();
    let config = Config::load().unwrap();
    let api = Api::new(&config);

    let res = api.clock_out(condition).unwrap();
    println!("{}", res.text().unwrap());
}
