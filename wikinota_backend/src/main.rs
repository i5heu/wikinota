#[macro_use]
extern crate serde_derive;
extern crate crypto;
extern crate rustc_serialize;
use self::crypto::digest::Digest;
use self::crypto::sha3::Sha3;
use rustc_serialize::json;
use std::process;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

use rusqlite::{params, Connection, Result, NO_PARAMS};
use std::{io::Write, sync::Arc};

use actix_web::{
    middleware, web,
    web::{post, Bytes, Query},
    App, Error, HttpRequest, HttpResponse, HttpServer,
};
use db_layer::PersistentItem;
use gerasdb::{db_layer, DbSession};

extern crate gerasdb;

struct AppState {
    app_name: String,
    pool: Arc<Pool<SqliteConnectionManager>>,
}
#[derive(Deserialize)]
struct FormStruct {
    content: String,
}

#[derive(Deserialize)]
struct QueryParam {
    id: String,
}


async fn getItem(actix_data: web::Data<AppState>, query_params: web::Query<QueryParam>) -> HttpResponse {
    // let conn = pool.get().unwrap();
    // let appdata = req.app_data();

    let foo = match db_layer::get(&actix_data.pool, &query_params.id.to_string()) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    let html = format!(r#"{}"#, json::encode(&foo).unwrap());

    // Ok(HttpResponse::Ok().into())
    HttpResponse::Ok().body(&html)
}


async fn save_file(actix_data: web::Data<AppState>, bytes: Bytes) -> HttpResponse {
    // let conn = pool.get().unwrap();
    // let appdata = req.app_data();

    let name =
        match String::from_utf8(bytes.to_vec()).map_err(|_| HttpResponse::BadRequest().finish()) {
            Ok(e) => e,
            Err(e) => {
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

    let foo = match db_layer::get(&actix_data.pool, &hex.to_string()) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    let html = format!(r#"{}"#, json::encode(&foo).unwrap());

    // Ok(HttpResponse::Ok().into())
    HttpResponse::Ok().body(&html)
}

fn index() -> HttpResponse {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
        <form id="as">
            <input type="input"   id="content"/>
            <input type="submit" value="Submit"></button>
        </form>
        <div id="bob"></div>
        <script>
        async function postData(url = '', data = {}) {
            // Default options are marked with *
            const response = await fetch(url, {
              method: 'POST', // *GET, POST, PUT, DELETE, etc.
              mode: 'cors', // no-cors, *cors, same-origin
              cache: 'no-cache', // *default, no-cache, reload, force-cache, only-if-cached
              credentials: 'same-origin', // include, *same-origin, omit
              headers: {
                'Content-Type': 'application/json'
                // 'Content-Type': 'application/x-www-form-urlencoded',
              },
              redirect: 'follow', // manual, *follow, error
              referrerPolicy: 'no-referrer', // no-referrer, *no-referrer-when-downgrade, origin, origin-when-cross-origin, same-origin, strict-origin, strict-origin-when-cross-origin, unsafe-url
              body: JSON.stringify(data) // body data type must match "Content-Type" header
            });
            return response.json(); // parses JSON response into native JavaScript objects
          }

          function postString() {
            postData('/', document.getElementById("content").value)
            .then(data => {
              console.log(data);
              document.getElementById("bob").innerText = JSON.stringify(data, null,2);
            });
          }
          document.getElementById("as").addEventListener("submit",e => {
              e.preventDefault();
              postString();
          })

        </script>
        </body>
    </html>"#;

    HttpResponse::Ok().body(html)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    std::fs::create_dir_all("./tmp").unwrap();

    let ip = "0.0.0.0:3026";

    let db = match gerasdb::init() {
        Ok(e) => e,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    let db_arc = Arc::new(db.pool);

    let app_data = web::Data::new(AppState {
        app_name: String::from("turreta"),
        pool: db_arc,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/")
                    .route(web::get().to(index))
                    .route(web::post().to(save_file)),
            )
            .service(web::resource("/get").route(web::get().to(getItem)))
    })
    .bind(ip)?
    .run()
    .await
}

#[test]
fn it_works() -> Result<(), gerasdb::dbError> {
    let result = gerasdb::init()?;
    assert_eq!(result.db_name, "HelloDBName");

    Ok(())
}
