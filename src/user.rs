use serde::{Deserialize, Serialize};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Error,error, App};
use futures::stream::StreamExt;

#[derive(Serialize, Deserialize)]
pub enum Language {
    Yoruba,
    Igbo,
    Hausa,
    English
}

//user schema
#[derive(Serialize, Deserialize)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: u64,
    pub location: String,
    pub preferred_language: Language,
    pub premium_user: bool
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(create_user);
}

//get user greeting name
#[get("/name")]
async fn default_name(data: web::Data<User>) -> String {
    let user_name = &data.first_name; 

    format!("Hello {}!", user_name) 
}

#[get("/")]
pub async fn create_user() -> impl Responder {
    let user = User {
        first_name: "".to_string(),
        last_name: "".to_string(),
        email: "".to_string(),
        phone_number: 0,
        location: "".to_string(),
        preferred_language: Language::English,
        premium_user: false,
    };

    // let json_output = json::encode(&user).unwrap;
    // let json_output = serde_json::to_string(&user)?;

    HttpResponse::Ok().json(user)
}

pub fn run() {

}




