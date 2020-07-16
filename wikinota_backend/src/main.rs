use std::process;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

use rusqlite::{params, Connection, Result, NO_PARAMS};
use std::{io::Write, sync::Arc};

use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use gerasdb::DbSession;

extern crate gerasdb;

struct AppState {
    app_name: String,
    pool: Arc<Pool<SqliteConnectionManager>>,
}

async fn save_file(req: HttpRequest) -> HttpResponse {
    // let conn = pool.get().unwrap();
    // let appdata = req.app_data();
    // let data: Option<&AppState> = req.app_data();

    let html = r#"<html>
    <head><title>Upload Test</title></head>
    <body>
            OK
    </body>
    </html>"#;

    println!("HIER EIN TEST");

    // Ok(HttpResponse::Ok().into())
    HttpResponse::Ok().body(html)
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

    let ip = "0.0.0.0:3001";

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
