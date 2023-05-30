use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use serde::Deserialize;

use crate::models::{NewTask, Task};
use crate::utils::validate_user_id;

#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,
    pub description: String,
}

pub async fn create_task(
    pool: web::Data<PgPool>,
    user_id: web::Path<i32>,
    task: web::Json<CreateTaskRequest>,
) -> impl Responder {
    if !validate_user_id(user_id.into_inner()) {
        return HttpResponse::Forbidden().body("Invalid user ID");
    }

    let new_task = NewTask {
        title: task.title.clone(),
        description: task.description.clone(),
    };

    match create_task_in_db(&pool, user_id.into_inner(), &new_task).await {
        Ok(task) => HttpResponse::Created().json(task),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_tasks(pool: web::Data<PgPool>, user_id: web::Path<i32>) -> impl Responder {
    if !validate_user_id(user_id.into_inner()) {
        return HttpResponse::Forbidden().body("Invalid user ID");
    }

    match get_tasks_from_db(&pool, user_id.into_inner()).await {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_task(
    pool: web::Data<PgPool>,
    user_id: web::Path<i32>,
    task_id: web::Path<i32>,
) -> impl Responder {
    if !validate_user_id(user_id.into_inner()) {
        return HttpResponse::Forbidden().body("Invalid user ID");
    }

    match get_task_from_db(&pool, user_id.into_inner(), task_id.into_inner()).await {
        Ok(Some(task)) => HttpResponse::Ok().json(task),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn update_task(
    pool: web::Data<PgPool>,
    user_id: web::Path<i32>,
    task_id: web::Path<i32>,
    updated_task: web::Json<CreateTaskRequest>,
) -> impl Responder {
    if !validate_user_id(user_id.into_inner()) {
        return HttpResponse::Forbidden().body("Invalid user ID");
    }

    let updated_task = NewTask {
        title: updated_task.title.clone(),
        description: updated_task.description.clone(),
    };

    match update_task_in_db(&pool, user_id.into_inner(), task_id.into_inner(), &updated_task).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_task(
    pool: web::Data<PgPool>,
    user_id: web::Path<i32>,
    task_id: web::Path<i32>,
) -> impl Responder {
    if !validate_user_id(user_id.into_inner()) {
        return HttpResponse::Forbidden().body("Invalid user ID");
    }

    match delete_task_from_db(&pool, user_id.into_inner(), task_id.into_inner()).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn create_task_in_db(
    pool: &PgPool,
    user_id: i32,
    new_task: &NewTask,
) -> Result<Task, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        INSERT INTO tasks (title, description, completed, user_id)
        VALUES ($1, $2, false, $3)
        RETURNING id, title, description, completed, user_id
        "#,
        new_task.title,
        new_task.description,
        user_id
    )
    .fetch_one(pool)
    .await?;

    Ok(Task {
        id: row.id,
        title: row.title,
        description: row.description,
        completed: row.completed,
        user_id: row.user_id,
    })
}

async fn get_tasks_from_db(pool: &PgPool, user_id: i32) -> Result<Vec<Task>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"
        SELECT id, title, description, completed, user_id
        FROM tasks
        WHERE user_id = $1
        ORDER BY id ASC
        "#,
        user_id
    )
    .fetch_all(pool)
    .await?;

    let tasks = rows
        .into_iter()
        .map(|row| Task {
            id: row.id,
            title: row.title,
            description: row.description,
            completed: row.completed,
            user_id: row.user_id,
        })
        .collect();

    Ok(tasks)
}

async fn get_task_from_db(
    pool: &PgPool,
    user_id: i32,
    task_id: i32,
) -> Result<Option<Task>, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        SELECT id, title, description, completed, user_id
        FROM tasks
        WHERE id = $1 AND user_id = $2
        "#,
        task_id,
        user_id
    )
    .fetch_optional(pool)
    .await?;

    match row {
       
        Some(row) => {
            let task = Task {
                id: row.id,
                title: row.title,
                description: row.description,
                completed: row.completed,
                user_id: row.user_id,
            };
            Ok(Some(task))
        }
        None => Ok(None),
    }
}

async fn update_task_in_db(
    pool: &PgPool,
    user_id: i32,
    task_id: i32,
    updated_task: &NewTask,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        UPDATE tasks
        SET title = $1, description = $2
        WHERE id = $3 AND user_id = $4
        "#,
        updated_task.title,
        updated_task.description,
        task_id,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected() == 1)
}

async fn delete_task_from_db(
    pool: &PgPool,
    user_id: i32,
    task_id: i32,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        DELETE FROM tasks
        WHERE id = $1 AND user_id = $2
        "#,
        task_id,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected() == 1)
}
mod task_handlers;
pub use task_handlers::*;

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/tasks")
                    .route("", web::post().to(create_task))
                    .route("", web::get().to(get_tasks))
                    .route("/{task_id}", web::get().to(get_task))
                    .route("/{task_id}", web::put().to(update_task))
                    .route("/{task_id}", web::delete().to(delete_task)),
            ),
    );
}
