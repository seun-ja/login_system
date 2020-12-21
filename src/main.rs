mod login;
mod user;
use actix_web::{delete, get, post, HttpServer, web, HttpResponse, Responder, Error, HttpRequest, error, App};
use futures::stream::StreamExt;
pub use user::*;

//function responds to Http request 
async fn response(request: HttpRequest) -> impl Responder {
    web::Json(User{
        first_name: "".to_string(),
        last_name: "".to_string(),
        email: "".to_string(),
        phone_number: 0,
        location: "".to_string(),
        preferred_language: Language::English,
        premium_user: false,
    })
}

//routes
#[post("/")]
pub async fn index_manual(mut payload: web::Payload) -> Result<HttpResponse, Error> {
    // payload is a stream of Bytes objects
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > 10 {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    // body is loaded, now we can deserialize serde-json
    let obj = serde_json::from_slice::<User>(&body)?;
    Ok(HttpResponse::Ok().json(obj)) // <- send response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // .configure(user::init_routes)
            .route("/", web::get().to(response))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
