use crate::config::Config;
use chrono::prelude::*;
#[cfg(test)]
use mockito;
use reqwest::blocking::{Client, Response};
use reqwest::header;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

static APP_USER_AGENT: &str = "miteras-cli";

pub struct Api {
    config: Config,
    client: Client,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ClockInParams {
    clock_in_condition: HashMap<String, i32>,
    daily_place_evidence: HashMap<String, i32>,
    work_date_string: String,
    enable_break_time: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ClockOutParams {
    clock_out_condition: HashMap<String, i32>,
    daily_place_evidence: HashMap<String, i32>,
    work_date_string: String,
    stamp_break_start: String,
    stamp_break_end: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdatePasswordParams {
    old_pass: String,
    new_pass: String,
    confirm_pass: String,
}

// TODO: Refactor to enum
fn condition_value(condition: &str) -> i32 {
    match condition {
        "best" => 1,
        "good" => 2,
        "normal" => 3,
        "bad" => 4,
        // NOTE: not reached
        _ => -1,
    }
}

fn work_date_string() -> String {
    let today = Local::today();
    format!("{}-{}-{}", today.year(), today.month(), today.day())
}

fn parse_csrf(body: String) -> String {
    let fragment = Html::parse_fragment(&body);
    let selector = Selector::parse("meta[name='_csrf'], input[name='_csrf']").unwrap();
    let tag = fragment.select(&selector).next().unwrap().value();

    let attr = if tag.name() == "meta" {
        "content"
    } else {
        "value"
    };
    tag.attr(attr).unwrap().to_string()
}

impl Api {
    pub fn new(config: &Config) -> Api {
        let conf = Config::new(
            (*config.org).to_string(),
            (*config.username).to_string(),
            (*config.password).to_string(),
        );
        let client = Client::builder()
            .cookie_store(true)
            .user_agent(APP_USER_AGENT)
            .build()
            .unwrap();

        Api {
            config: conf,
            client: client,
        }
    }

    pub fn login(&self) -> Result<Response, reqwest::Error> {
        let login_url = self.build_url("login");
        let login_res = self.client.get(&login_url).send().unwrap();
        let csrf = parse_csrf(login_res.text().unwrap());

        let auth_url = self.build_url("auth");
        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("username", &self.config.username);
        params.insert("password", &self.config.password);
        params.insert("_csrf", &csrf);
        self.client
            .post(&auth_url)
            .form(&params)
            .header(header::REFERER, login_url)
            .send()
    }

    pub fn clock_in(&self, condition: &str) -> Result<Response, reqwest::Error> {
        let auth_res = self.login().unwrap();
        let csrf = parse_csrf(auth_res.text().unwrap());

        let cico_url = self.build_url("cico");
        let url = self.build_url("submitClockIn");
        let mut clock_in_condition = HashMap::new();
        clock_in_condition.insert("condition".to_string(), condition_value(condition));
        let params = ClockInParams {
            clock_in_condition: clock_in_condition,
            daily_place_evidence: HashMap::new(),
            work_date_string: work_date_string(),
            enable_break_time: false,
        };

        self.client
            .post(&url)
            .json(&params)
            .header("X-CSRF-TOKEN", csrf)
            .header(header::REFERER, cico_url)
            .send()
    }

    pub fn clock_out(&self, condition: &str) -> Result<Response, reqwest::Error> {
        let auth_res = self.login().unwrap();
        let csrf = parse_csrf(auth_res.text().unwrap());

        let cico_url = self.build_url("cico");
        let url = self.build_url("submitClockOut");
        let mut clock_out_condition = HashMap::new();
        clock_out_condition.insert("condition".to_string(), condition_value(condition));
        let params = ClockOutParams {
            clock_out_condition: clock_out_condition,
            daily_place_evidence: HashMap::new(),
            work_date_string: work_date_string(),
            stamp_break_start: "".to_string(),
            stamp_break_end: "".to_string(),
        };

        self.client
            .post(&url)
            .json(&params)
            .header("X-CSRF-TOKEN", csrf)
            .header(header::REFERER, cico_url)
            .send()
    }

    pub fn update_password(&self, new_password: String) -> Result<Response, reqwest::Error> {
        let auth_res = self.login().unwrap();
        let csrf = parse_csrf(auth_res.text().unwrap());

        let url = self.build_url("resetPassword");
        let params = UpdatePasswordParams {
            old_pass: self.config.password.to_string(),
            new_pass: new_password.to_string(),
            confirm_pass: new_password.to_string(),
        };

        self.client
            .post(&url)
            .form(&params)
            .header("X-CSRF-TOKEN", csrf)
            .send()
    }

    fn build_url(&self, path: &str) -> String {
        #[cfg(not(test))]
        let endpoint = "https://kintai.miteras.jp";
        #[cfg(test)]
        let endpoint = &mockito::server_url();

        format!("{}/{}/{}", endpoint, self.config.org, path)
    }
}

#[cfg(test)]
mod tests {
    use super::condition_value;

    #[test]
    fn str_to_num() {
        assert_eq!(1, condition_value("best"));
        assert_eq!(2, condition_value("good"));
        assert_eq!(3, condition_value("normal"));
        assert_eq!(4, condition_value("bad"));
    }
}
