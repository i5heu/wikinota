use crate::{AppState, QueryParam};
use actix_web::{http, web, web::Bytes, HttpRequest, HttpResponse};
use crypto::{digest::Digest, sha3::Sha3};
use db_layer::PersistentItem;
use gerasdb::db_layer;
use rustc_serialize::json;
use std::{fs, process};

pub fn redirect_to_https(req: HttpRequest) -> HttpResponse {
    let host = match get_host(&req) {
        Some(x) => {x},
        None    => "heidenstedt.org/"
    };

    let mut redirect_url: String = "https://".to_owned();
    redirect_url.push_str(host);


    HttpResponse::PermanentRedirect()
        .header(http::header::LOCATION, redirect_url)
        .finish()
}

fn get_host<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    req.headers().get("Host")?.to_str().ok()
}

pub fn index() -> HttpResponse {
    HttpResponse::Ok().body(get_index_html())
}

fn get_index_html() -> String {
    let contents = fs::read_to_string("../wikinota_frontend/index.html")
        .expect("Something went wrong reading the file");
    return contents;
}

pub async fn get_item(
    actix_data: web::Data<AppState>,
    query_params: web::Query<QueryParam>,
) -> HttpResponse {
    let foo = match db_layer::get_by_hash(&actix_data.pool, &query_params.id.to_string()) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    let html = format!(r#"{}"#, json::encode(&foo).unwrap());

    HttpResponse::Ok().body(&html)
}

pub async fn save_file(actix_data: web::Data<AppState>, bytes: Bytes) -> HttpResponse {
    let name =
        match String::from_utf8(bytes.to_vec()).map_err(|_| HttpResponse::BadRequest().finish()) {
            Ok(e) => e,
            Err(_) => {
                eprintln!("Errrrrror");
                process::exit(1);
            }
        };

    let mut hasher = Sha3::sha3_256();
    hasher.input_str(&name);

    let hex = hasher.result_str();
    let hash = "";

    let test_item: &PersistentItem = &PersistentItem {
        hash: hex.to_string(),
        key: "testing:test".to_string(),
        tree_hash: hex.to_string(),
        parent_hash: hex.to_string(),
        hash_if_deleted: hex.to_string(),
        lvl: 456835687,
        creator: String::from(hash),
        created: 567445672,
        importance: 234235675,
        content: String::from(hash),
        deleted: false,
        last_checked: 2141235,
        reading_errors: 235235,
        extras: String::from(hash),
    };

    match db_layer::insert(&actix_data.pool, &test_item) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    let foo = match db_layer::get_by_hash(&actix_data.pool, &hex.to_string()) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    let html = format!(r#"{}"#, json::encode(&foo).unwrap());

    HttpResponse::Ok().body(&html)
}

#[test]
fn t_get_index_html() {
    let res = get_index_html();
    assert!(res.contains("<html>"));
}
