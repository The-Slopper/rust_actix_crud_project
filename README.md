# Serviço de Alunos (Actix-web)

API de alunos em Rust com Actix-web e estado em memória protegido por `Mutex`.

## Metadados

| Campo | Valor |
|-------|-------|
| Linguagem | Rust (edition 2021) |
| Framework | Actix-web 4 |
| Serialização | serde / serde_json |
| Armazenamento | Em memória (`Mutex<AppState>`) |
| Versão | 1.0.0 |
| Licença | MIT |
| Responsável | Equipe de Plataforma |

## Descrição

Serviço HTTP com o CRUD de alunos. O estado é mantido em memória e compartilhado entre as
requisições por meio de `web::Data<Mutex<AppState>>`.

## Endpoints

| Método | Rota | Descrição |
|--------|------|-----------|
| GET | `/alunos` | Lista de alunos |
| GET | `/alunos/{id}` | Detalhe de um aluno |
| POST | `/alunos` | Cria um aluno |
| DELETE | `/alunos/{id}` | Remove um aluno |

## Estrutura

```
rust-actix/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── models.rs
    └── handlers.rs
```

## Como executar

```bash
cargo run
# servidor em http://localhost:8080
```

## Configuração

URL de conexão e constantes em `src/models.rs`. Testes com `cargo test`.
