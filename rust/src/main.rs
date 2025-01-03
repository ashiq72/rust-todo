use actix_web::{Web, App, HttpServer, Responder, HttpResponse};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::sync::Mutex;
use chrono::{Utc, DateTime};

#[derive(Serialize, Deserialize, Clone)]
struct TodoItem {
    id: Uuid,
    title: String,
    completed: bool,
    created_at: DateTime<Utc>, // Ensure this works with the "serde" feature enabled.
}

#[derive(Deserialize)]
struct CreateTodoItem{
    title: String,
    completed: bool,
}

#[derive(Deserialize)]
struct UpdateTodoItem{
    title: Option<String>,
    completed: Option<bool>,
}

struct AppState{
    todo_list: Mutex<Vec<TodoItem>>
}

async fn get_todos(data: web::Data<AppState>) ->  impl Responder{
    let todos = data.todo_list.lock().unwrap();
    HttpResponse::Ok().json(todos)
}

async fn create_todo(data: web::Data<AppState>, item: web::Json<CreateTodoItem>) -> impl Responder {
    let mut todos = data.todo_list.lock().unwrap();
    // let new_id = Uuid::new_v4();
    let new_todo = TodoItem {
        id: Uuid::new_v4(),
        title: item.title.clone(),
        completed: item.completed,
        created_at: Utc::now(),
    };
    todos.push(new_todo);
    HttpResponse::Ok().json(&*todos)
}

async fn update_todo(
    path: web::Path<Uuid>,
    item: web::Json<UpdateTodoItem>,
    data: web::Data<AppState>,
)-> impl Responder{
    let mut todos = data.todo_list.lock().unwrap();
    if let Some(todo) = todos.iter_mut().find(|todo| todo.id == *path) {
if let Some(title) = &item.title {
    todo.title = title.clone();
} 
if let Some(completed) = item.completed {
    todo.completed = completed;
}
if let Some(completed) = item.completed {
    todo.completed = completed;
}
      HttpResponse::Ok().json(&*todos)
    } else {
        HttpResponse::NotFound().body("Todo not found")
    }
}

async fn delete_todo(path: web::Path<Uuid>, data: web::Data<AppState>) -> impl Responder {
    let mut todos = data.todo_list.lock().unwrap();
 
    if todos.iter().any(|todo| todo.id == *path) {
        todos.remove(index);
        HttpResponse::Ok().json(&*todos)
    } else {
        HttpResponse::NotFound().body("Todo not found")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Date::new(AppState {

        todo_list: Mutex::new(Vec::new())
    )}
    HttpServer::new(||{
        let cors = Cors::default().allow_any_origin()
        .allow_any_method()
        .allow_any_header()
        .max_age(3600)
    })

    App::new(app_data(app_state.clone()).warp(cors).route("/todos", web::get().to(get_todos())
))
};