use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use crate::{AppState, QueryParam};
use actix_web::{http, http::header::ContentType, web, web::Bytes, HttpRequest, HttpResponse};
use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable)]
pub struct LoginToken {
    loginToken: String,
}

pub async fn get_loginToken(
    actix_data: web::Data<AppState>,
) -> HttpResponse {

    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .collect();

    let foo = LoginToken {
        loginToken: rand_string
    };

    let html = format!(r#"{}"#, json::encode(&foo).unwrap());

    HttpResponse::Ok().body(&html)
}
