#[macro_use]
extern crate serde_derive;
extern crate gerasdb;
extern crate crypto;
extern crate rustc_serialize;
use actix_web::{middleware, web, App, HttpServer};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use std::process;
use std::sync::Arc;
mod reqHandlers;

pub struct AppState {
    app_name: String,
    pool: Arc<Pool<SqliteConnectionManager>>,
}
#[derive(Deserialize)]
struct FormStruct {
    content: String,
}

#[derive(Deserialize)]
pub struct QueryParam {
    id: String,
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
                    .route(web::get().to(reqHandlers::index))
                    .route(web::post().to(reqHandlers::save_file)),
            )
            .service(web::resource("/get").route(web::get().to(reqHandlers::getItem)))
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
