use std::process;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

use rusqlite::{params, Connection, Result, NO_PARAMS};
use std::io::Write;

use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use gerasdb::DbSession;

extern crate gerasdb;

async fn save_file(
) -> Result<HttpResponse, Error> {
    // let conn = pool.get().unwrap();

    Ok(HttpResponse::Ok().into())
    // Ok(HttpResponse::Ok().body(foo.0));
}

fn index() -> HttpResponse {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form target="/" method="post">
                <input type="input"  name="content"/>
                <input type="submit" value="Submit"></button>
            </form>
        </body>
    </html>"#;

    HttpResponse::Ok().body(html)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    std::fs::create_dir_all("./tmp").unwrap();

    let ip = "0.0.0.0:3000";

    let db = match gerasdb::init() {
        Ok(e) => e,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    struct AppState {
        app_name: String,
        pool: Pool<SqliteConnectionManager>,
    }

    let app_data = web::Data::new(AppState {
        app_name: String::from("turreta"),
        pool: db.pool,
    });

    HttpServer::new(|| {
        App::new()
            .app_data(app_data.clone())
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/")
                    .route(web::get().to(index))
                    .route(web::post().to(save_file)),
            )
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
