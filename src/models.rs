use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cleaner {
    pub id: Uuid,
    pub name: String,
    pub phone: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateCleaner {
    pub name: String,
    pub phone: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateCleaner {
    pub name: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RoomStatus {
    Available,
    Occupied,
    Cleaning,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    pub id: Uuid,
    pub room_number: String,
    pub floor: i32,
    pub room_type: String,
    pub status: RoomStatus,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateRoom {
    pub room_number: String,
    pub floor: i32,
    pub room_type: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateRoom {
    pub room_number: Option<String>,
    pub floor: Option<i32>,
    pub room_type: Option<String>,
    pub status: Option<RoomStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleaningTask {
    pub id: Uuid,
    pub cleaner_id: Uuid,
    pub room_id: Uuid,
    pub status: TaskStatus,
    pub scheduled_date: String,
    pub remarks: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateCleaningTask {
    pub cleaner_id: Uuid,
    pub room_id: Uuid,
    pub scheduled_date: String,
    pub remarks: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateCleaningTask {
    pub cleaner_id: Option<Uuid>,
    pub room_id: Option<Uuid>,
    pub status: Option<TaskStatus>,
    pub scheduled_date: Option<String>,
    pub remarks: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CleaningTaskDetail {
    pub id: Uuid,
    pub cleaner: Cleaner,
    pub room: Room,
    pub status: TaskStatus,
    pub scheduled_date: String,
    pub remarks: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateProgress {
    pub status: TaskStatus,
}
