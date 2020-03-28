use crate::api::Api;
use crate::config::Config;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use rpassword::read_password;
use rpassword::read_password_with_reader;
use std::io::{self, BufRead, Empty, Write};

pub fn run(matches: ArgMatches) {
    if let Some(_matches) = matches.subcommand_matches("login") {
        login(None::<Empty>, io::stdout());
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

fn read_input<R: BufRead, W: Write>(
    label: &str,
    hidden: bool,
    source: Option<R>,
    mut writer: W,
) -> String {
    write!(&mut writer, "{}: ", label).unwrap();
    writer.flush().unwrap();

    if hidden {
        match source {
            Some(reader) => read_password_with_reader(Some(reader)).unwrap(),
            None => read_password().unwrap(),
        }
    } else {
        let mut input = String::new();
        match source {
            Some(mut reader) => {
                reader.read_line(&mut input).unwrap();
            }
            None => {
                io::stdin().read_line(&mut input).unwrap();
            }
        }
        input.trim().to_string()
    }
}

pub fn login<R: BufRead, W: Write>(mut source: Option<R>, mut writer: W) {
    write!(&mut writer, "Try logging in to MITERAS.\n").unwrap();

    let org = read_input("Org", false, source.as_mut(), &mut writer);
    let username = read_input("Username", false, source.as_mut(), &mut writer);
    let password = read_input("Password", true, source.as_mut(), &mut writer);
    let config = Config::new(org, username, password);

    let api = Api::new(&config);
    let res = api.login().unwrap();

    if res.url().path().ends_with("/cico") {
        config.save().ok();
        write!(&mut writer, "\nLogin successful.").unwrap();
    } else {
        write!(&mut writer, "\nLogin failed.").unwrap();
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
    use super::{build_app, login, run, Config};
    use chrono::prelude::*;
    use mockito::{mock, Matcher, Mock};
    use std::io::Cursor;

    fn mock_login() -> Mock {
        mock("GET", "/A123456/login")
            .with_body_from_file("tests/files/login.html")
            .create()
    }

    fn mock_auth(success: bool) -> Mock {
        let location = if success {
            "/A123456/cico"
        } else {
            "/A123456/login"
        };
        println!("{}", location);
        mock("POST", "/A123456/auth")
            .with_status(302)
            .with_header("Location", location)
            .create()
    }

    fn mock_cico() -> Mock {
        mock("GET", "/A123456/cico")
            .with_body_from_file("tests/files/cico.html")
            .create()
    }

    #[test]
    fn login_with_valid_args() {
        let _m1 = mock_login();
        let _m2 = mock_auth(true);
        let _m3 = mock_cico();

        let source = Cursor::new(b"A123456\nsinsoku\npass1234");
        let mut writer = Vec::<u8>::new();
        login(Some(source), &mut writer);

        _m1.assert();
        _m2.assert();
        _m3.assert();
        assert_eq!(
            String::from_utf8(writer).unwrap(),
            "Try logging in to MITERAS.\nOrg: Username: Password: \nLogin successful."
        );
    }

    #[test]
    fn login_with_invalid_args() {
        let _m1 = mock("GET", "/A123456/login")
            .with_body_from_file("tests/files/login.html")
            .expect(2)
            .create();
        let _m2 = mock_auth(false);

        let source = Cursor::new(b"A123456\nsinsoku\npassXXX");
        let mut writer = Vec::<u8>::new();
        login(Some(source), &mut writer);

        _m1.assert();
        _m2.assert();
        assert_eq!(
            String::from_utf8(writer).unwrap(),
            "Try logging in to MITERAS.\nOrg: Username: Password: \nLogin failed."
        );
    }

    #[test]
    fn clock_in_no_args() {
        let config = Config::new(
            "A123456".to_string(),
            "sinsoku".to_string(),
            "pass1234".to_string(),
        );
        config.save().ok();

        let _m1 = mock_login();
        let _m2 = mock_auth(true);
        let _m3 = mock_cico();

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
        let _m4 = mock("POST", "/A123456/submitClockIn")
            .match_header("content-type", "application/json")
            .match_body(Matcher::Json(params))
            .create();

        let matches = build_app().get_matches_from(vec!["miteras", "clock-in"]);
        run(matches);

        _m1.assert();
        _m2.assert();
        _m3.assert();
        _m4.assert();
    }

    #[test]
    fn clock_out_no_args() {
        let config = Config::new(
            "A123456".to_string(),
            "sinsoku".to_string(),
            "pass1234".to_string(),
        );
        config.save().ok();

        let _m1 = mock_login();
        let _m2 = mock_auth(true);
        let _m3 = mock_cico();

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
        let _m4 = mock("POST", "/A123456/submitClockOut")
            .match_header("content-type", "application/json")
            .match_body(Matcher::Json(params))
            .create();

        let matches = build_app().get_matches_from(vec!["miteras", "clock-out"]);
        run(matches);

        _m1.assert();
        _m2.assert();
        _m3.assert();
        _m4.assert();
    }
}
