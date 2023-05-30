use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub user_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewTask {
    pub title: String,
    pub description: String,
}
