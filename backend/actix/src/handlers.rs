use std::sync::Mutex;
use actix_web::delete;
use actix_web::put;
use actix_web::web::Data;
use actix_web::web::Redirect;
use actix_files::NamedFile;
use actix_web::{get, post, web, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::database::db::DB;
use crate::database::db::Task;

#[derive(Deserialize, Serialize, Debug)]
struct TaskJson {
    task: String
}

#[get("/error")]
async fn error() -> impl Responder {
    "An unexpected error occurred"
}

#[get("/")]
async fn index() -> NamedFile {
    NamedFile::open_async("../../frontend/src/index.html").await.unwrap()
}

#[post("/addtask")]
async fn task_add(json: web::Json<TaskJson>, data: Data<Mutex<DB>>) -> impl Responder {
    let datab = data.lock().unwrap();
    let uuid = Uuid::new_v4();
    
    let filter_string: String = json.task.chars()
        .map(|x| match x {
            '<' => ' ',
            '>' => ' ',
            '/' => ' ',
            _ => x
        })
        .collect();

    let task = Task {task: String::from(filter_string), uuid: uuid.to_string(), marked: "no-underline".to_string()}; 

    match datab.add_task(task).await {
        Ok(_) => Redirect::to("/gettask").see_other(),
        Err(_) => Redirect::to("/error").see_other()
    }
}

#[put("/checktask/{uuid}")]
async fn task_check(data: Data<Mutex<DB>>, uuid: web::Path<String>) -> impl Responder {
    let datab = data.lock().unwrap();
    match datab.update_task(uuid.to_string()).await {
        Ok(_) => Redirect::to("/gettask").see_other(),
        Err(_) => Redirect::to("/error").see_other()
    }
}

#[get("/gettask")]
async fn task_get(data: Data<Mutex<DB>>) -> impl Responder {
    let datab = data.lock().unwrap(); 
    let data_vec = datab.get_task().await.unwrap();
    let mut page_html = String::new();

    for i in data_vec {
        let formated = format!("<div class='w-full flex justify-center items-center border-b'> 
                                    <div> 
                                        <h2 class='text-white text-2xl m-4 {}'> {} </h2> 
                                    </div> 
                                    
                                    <div class='flex flex-col'> 
                                        <button class='font-bold text-blue-900' hx-put='/checktask/{}' hx-target='#tasks' hx-swap='innerHTML'>✅ </button> 
                                        <button class='font-bold text-red-800' hx-delete='/delete/{}' hx-target='#tasks' hx-swap='innerHTML'> ❌ </button> 
                                    </div>
                                </div>", &i.marked, &i.task, &i.uuid, &i.uuid);
        page_html.push_str(&formated);
    }
    page_html
}

#[delete("/delete/{uuid}")]
async fn task_delete(data: Data<Mutex<DB>>, uuid: web::Path<String>) -> impl Responder {
    let datab = data.lock().unwrap();
    match datab.delete_task(uuid.to_string()).await {
        Ok(_) => Redirect::to("/gettask").see_other(),
        Err(_) => Redirect::to("/error").see_other()
    }
}

