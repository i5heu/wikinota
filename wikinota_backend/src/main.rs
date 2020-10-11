#[macro_use]
extern crate serde_derive;
extern crate crypto;
extern crate gerasdb;
extern crate rustc_serialize;
use actix_web::{middleware, web, App, HttpServer};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use std::process;
use std::sync::Arc;
mod req_handlers;
mod auth;

pub struct AppState {
    app_name: String,
    pool: Arc<Pool<SqliteConnectionManager>>,
}

#[derive(Deserialize)]
pub struct QueryParam {
    id: String,
}

// openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    std::fs::create_dir_all("./tmp").unwrap();

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

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(|| App::new().route("*", web::get().to(req_handlers::redirect_to_https)))
        .bind("0.0.0.0:80")?
        .run();

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/")
                    .route(web::get().to(req_handlers::index))
                    .route(web::post().to(req_handlers::save_file)),
            )
            .service(web::resource("/api/loginToken").route(web::get().to(auth::get_loginToken)))
            .service(web::resource("/get").route(web::get().to(req_handlers::get_item)))
            .service(web::resource("/js/*").route(web::get().to(req_handlers::js_files)))
            .service(web::resource("/css/*").route(web::get().to(req_handlers::css_files)))
    })
    .bind_openssl("0.0.0.0:443", builder)?
    .run()
    .await
}

#[test]
fn it_works() -> Result<(), gerasdb::dbError> {
    let result = gerasdb::init()?;
    assert_eq!(result.db_name, "HelloDBName");
    Ok(())
}
