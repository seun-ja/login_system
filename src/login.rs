extern crate argon2;
use argon2::{Config};

pub fn run() {
    let client_password: String = String::from("seunfromogbomosho");
    check_password(client_password);
}

fn check_password(tried_password: String) -> bool {
    let password = b"password";
    let salt = b"randomsalt";
    let config = Config::default();
    let hash = argon2::hash_encoded(password, salt, &config).unwrap();
    let confirmed: bool = true;

    if argon2::verify_encoded(&hash, password).unwrap() {
        return confirmed; 
    } else {
        return !confirmed;
        println!("Wrong Password");
    };

}