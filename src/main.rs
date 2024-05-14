use std::process;
use create_festl::Config;

fn main() {
    let config = Config::build();

    let password = create_festl::send_request(config).unwrap_or_else(|err| {
        eprintln!("Problem happened when sending request: {}",err);
        process::exit(1);
    });

    println!("Festl was created successfully!");
    println!("Admin Password: {}", password);
}