use clap::{App, AppSettings, Arg, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
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
        .subcommand(
            SubCommand::with_name("update-password")
                .about("Change your password with an auto-generated password")
        )
}
