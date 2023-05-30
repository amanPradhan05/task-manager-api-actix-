use actix_web::{test, App};
use dotenv::dotenv;
use sqlx::{Connection, Executor, PgConnection};
use std::env;

use task_manager_api::handlers::{create_task, get_tasks};
use task_manager_api::models::{NewTask, Task};
use task_manager_api::utils::validate_user_id;

#[actix_rt::test]
async fn test_create_task() {
    // Load environment variables
    dotenv().ok();

    // Set up a connection to the test database
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let mut conn = PgConnection::connect(&database_url).await.unwrap();

    // Create a test user ID
    let user_id = 1;

    // Create a test task
    let new_task = NewTask {
        title: "Test Task".to_owned(),
        description: "Test Description".to_owned(),
    };

    // Call the create_task handler
    let task = create_task(
        web::Data::new(conn.clone()),
        web::Path::from(user_id),
        web::Json(new_task),
    )
    .await
    .unwrap();

    // Verify the task details
    assert_eq!(task.title, "Test Task");
    assert_eq!(task.description, "Test Description");
    assert_eq!(task.completed, false);
    assert_eq!(task.user_id, user_id);
}

#[actix_rt::test]
async fn test_get_tasks() {
    // Load environment variables
    dotenv().ok();

    // Set up a connection to the test database
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let mut conn = PgConnection::connect(&database_url).await.unwrap();

    // Create a test user ID
    let user_id = 1;

    // Call the get_tasks handler
    let tasks = get_tasks(web::Data::new(conn.clone()), web::Path::from(user_id))
        .await
        .unwrap();

    // Verify the tasks are returned correctly
    assert_eq!(tasks.len(), 1);
    let task = &tasks[0];
    assert_eq!(task.title, "Test Task");
    assert_eq!(task.description, "Test Description");
    assert_eq!(task.completed, false);
    assert_eq!(task.user_id, user_id);
}

#[actix_rt::test]
fn test_validate_user_id() {
    // Test valid user ID
    let user_id = 1;
    assert_eq!(validate_user_id(user_id), true);

    // Test invalid user ID
    let user_id = -1;
    assert_eq!(validate_user_id(user_id), false);
}
