use std::{process,io};
use create_festl::Config;

fn main() {
    let (url, festl_name, username, password) = get_input();

    let config = Config::build(url, festl_name, username, password);

    let password = create_festl::send_request(config).unwrap_or_else(|err| {
        eprintln!("Problem happened when sending request: {}",err);
        process::exit(1);
    });

    println!("Festl was created successfully!");
    println!("Admin Password: {}", password);
}

fn get_input() -> (String, String, String, String){
    let mut url = String::new();
    let mut festl_name = String::new();
    let mut username = String::new();
    let mut password = String::new();
    println!("Enter URL (e.g. https://<url>:<port>):");
    io::stdin().read_line(&mut url).unwrap_or_else(|err| {
        eprintln!("Something went wrong with the stdin: {}", err);
        process::exit(1);
    });

    println!("Enter Festl Name:");
    io::stdin().read_line(&mut festl_name).unwrap_or_else(|err| {
        eprintln!("Something went wrong with the stdin: {}", err);
        process::exit(1);
    });

    println!("Enter Username:");
    io::stdin().read_line(&mut username).unwrap_or_else(|err| {
        eprintln!("Something went wrong with the stdin: {}", err);
        process::exit(1);
    });

    println!("Enter Password:");
    io::stdin().read_line(&mut password).unwrap_or_else(|err| {
        eprintln!("Something went wrong with the stdin: {}", err);
        process::exit(1);
    });
    return (url, festl_name, username, password);
}
