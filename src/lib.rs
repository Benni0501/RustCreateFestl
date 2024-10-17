use std::{error::Error, process, time::Duration};
use dialoguer::{theme::ColorfulTheme, Input, Password};
use regex::Regex;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::{blocking::Client, StatusCode};

pub struct Config{
    url: String,
    festl_name: String,
    username: String,
    password: String
}

impl Config {
    pub fn build() -> Config {
        let (url, festl_name, username, password) = Self::get_input().unwrap_or_else(|err| {
            eprintln!("Something went wrong with the input: {:?}", err);
            process::exit(1);
        });

        let username = username.trim().to_string();
        let password = password.trim().to_string();
        

        Config{url,festl_name, username, password}
    }

    fn get_input() -> Result<(String,String,String,String), Box<dyn Error>>{
        let re = Regex::new(r"^https*:\/\/\w+(\.\w+)*(:[0-9]+)?\/?(\/[.\w]*)*$").unwrap();
        let url: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter URL (e.g. https://<url>:<port>):")
        .validate_with(|input: &String| -> Result<(),&str> {
            if re.is_match(input) {
                Ok(())
            } else {
                Err("The input is not a valid url!")
            }
        })
        .interact_text()?;

        let festl_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter Festl Name:")
        .interact_text()?;

        let username: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter Username:")
        .interact_text()?;

        let password: String = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter Password:")
        .with_confirmation("Repeat Password", "Error: The passwords do not match!")
        .interact()?;

        Ok((url, festl_name, username, password))
    }
}

pub fn send_request(config: Config) -> Result<String, reqwest::Error>{
    let client = Client::builder().http1_title_case_headers().build()?;
    let url = format!("{}/api/festl/", config.url);

    let pb_style: Vec<&str> = vec!["⠋","⠙","⠚","⠞","⠖","⠦","⠴","⠲","⠳","⠓"];
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(80));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.blue} {msg}")
            .unwrap()
            .tick_strings(&pb_style),
    );
    pb.set_message("Creating...");

    let res = client.post(url)
        .basic_auth(config.username, Some(config.password))
        .header("Content-Type", "text/plain")
        .body(config.festl_name)
        .send()?;
    if !StatusCode::is_success(&res.status()) {
        eprintln!("Server error: {}", &res.status());
        process::exit(1);
    }
    let password = res.text().unwrap_or_else(|err| {
        eprint!("Error when parsing the response: {}", err);
        process::exit(1);
    });
    pb.finish_with_message("Done!");
    Ok(password)
}
