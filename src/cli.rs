use crate::api::Api;
use crate::config::Config;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use rpassword::read_password;
use std::io::{stdin, stdout, Write};

pub fn run() {
    let matches = build_app().get_matches();

    if let Some(_matches) = matches.subcommand_matches("login") {
        login();
    }

    if let Some(matches) = matches.subcommand_matches("clock-in") {
        clock_in(matches);
    }

    if let Some(matches) = matches.subcommand_matches("clock-out") {
        clock_out(matches);
    }
}

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
        config.save().ok();
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

#[cfg(test)]
mod tests {
    use super::{build_app, clock_in, clock_out, Config};
    use chrono::prelude::*;
    use handlebars::Handlebars;
    use mockito::{mock, Matcher};
    use std::collections::BTreeMap;

    #[test]
    fn clock_in_no_args() {
        let config = Config::new(
            "A123456".to_string(),
            "sinsoku".to_string(),
            "pass1234".to_string(),
        );
        config.save().ok();

        let mut handlebars = Handlebars::new();
        handlebars
            .register_templates_directory(".hbs", "./tests/templates")
            .unwrap();
        let mut data = BTreeMap::new();
        data.insert("org".to_string(), "A123456".to_string());
        let _m1 = mock("GET", "/A123456/login")
            .with_body(handlebars.render("login", &data).unwrap())
            .create();
        let _m2 = mock("POST", "/A123456/auth")
            .with_body(handlebars.render("cico", &data).unwrap())
            .create();

        let today = Local::today();
        let work_date_string = format!("{}-{}-{}", today.year(), today.month(), today.day());
        let params = json!({
            "clockInCondition": {
                "condition": 2
            },
            "dailyPlaceEvidence": {},
            "workDateString": work_date_string,
            "enableBreakTime": false
        });
        let _m3 = mock("POST", "/A123456/submitClockIn")
            .match_header("content-type", "application/json")
            .match_body(Matcher::Json(params))
            .create();

        let app = build_app().get_matches_from(vec!["miteras", "clock-in"]);
        if let Some(matches) = app.subcommand_matches("clock-in") {
            clock_in(&matches);
        }

        _m1.assert();
        _m2.assert();
        _m3.assert();
    }

    #[test]
    fn clock_out_no_args() {
        let config = Config::new(
            "A123456".to_string(),
            "sinsoku".to_string(),
            "pass1234".to_string(),
        );
        config.save().ok();

        let mut handlebars = Handlebars::new();
        handlebars
            .register_templates_directory(".hbs", "./tests/templates")
            .unwrap();
        let mut data = BTreeMap::new();
        data.insert("org".to_string(), "A123456".to_string());
        let _m1 = mock("GET", "/A123456/login")
            .with_body(handlebars.render("login", &data).unwrap())
            .create();
        let _m2 = mock("POST", "/A123456/auth")
            .with_body(handlebars.render("cico", &data).unwrap())
            .create();

        let today = Local::today();
        let work_date_string = format!("{}-{}-{}", today.year(), today.month(), today.day());
        let params = json!({
            "clockOutCondition": {
                "condition": 2
            },
            "dailyPlaceEvidence": {},
            "workDateString": work_date_string,
            "stampBreakStart": "",
            "stampBreakEnd": ""
        });
        let _m3 = mock("POST", "/A123456/submitClockOut")
            .match_header("content-type", "application/json")
            .match_body(Matcher::Json(params))
            .create();

        let app = build_app().get_matches_from(vec!["miteras", "clock-out"]);
        if let Some(matches) = app.subcommand_matches("clock-out") {
            clock_out(&matches);
        }

        _m1.assert();
        _m2.assert();
        _m3.assert();
    }
}
