use std::sync::Mutex;
use actix_web::web::Data;
use actix_web::{App, HttpServer};

pub mod database;
pub mod handlers;

use crate::database::db::DB;

use crate::handlers::index;
use crate::handlers::task_add;
use crate::handlers::task_get;
use crate::handlers::task_check;
use crate::handlers::task_delete;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = DB::new().await;
    let data = Data::new(Mutex::new(db));

    HttpServer::new(move || App::new()
                    .app_data(Data::clone(&data))
                    .service(index)
                    .service(task_add)
                    .service(task_get)
                    .service(task_delete)
                    .service(task_check))
                    .bind(("127.0.0.1", 8080))?
                    .run().await 
}
