use std::process;

use reqwest::{blocking::Client, StatusCode};

pub struct Config{
    url: String,
    festl_name: String,
    username: String,
    password: String
}

impl Config {
    pub fn build(url: String, festl_name: String, username: String, password: String) -> Config{
        let username = username.trim().to_string();
        let password = password.trim().to_string();
        Config{url,festl_name, username, password}
    }
}

pub fn send_request(config: Config) -> Result<String, reqwest::Error>{
    let client = Client::builder().http1_title_case_headers().build()?;
    let url = format!("{}/api/festl/", config.url);
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
    
    

    Ok(password)
}