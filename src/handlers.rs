use axum::{
    extract::{Path, State},
    Json,
};
use chrono::Utc;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::{
    Cleaner, CleaningTask, CleaningTaskDetail, CreateCleaner, CreateCleaningTask, CreateRoom,
    RoomStatus, TaskStatus, UpdateCleaner, UpdateCleaningTask, UpdateProgress, UpdateRoom, Room,
};
use crate::state::AppState;

pub async fn list_cleaners(State(state): State<AppState>) -> Json<Vec<Cleaner>> {
    let inner = state.inner.lock().await;
    Json(inner.cleaners.clone())
}

pub async fn get_cleaner(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Cleaner>, AppError> {
    let inner = state.inner.lock().await;
    inner
        .cleaners
        .iter()
        .find(|c| c.id == id)
        .cloned()
        .map(Json)
        .ok_or(AppError::NotFound)
}

pub async fn create_cleaner(
    State(state): State<AppState>,
    Json(payload): Json<CreateCleaner>,
) -> Result<Json<Cleaner>, AppError> {
    if payload.name.trim().is_empty() {
        return Err(AppError::BadRequest("保洁员姓名不能为空".to_string()));
    }

    let now = Utc::now().naive_utc();
    let cleaner = Cleaner {
        id: Uuid::new_v4(),
        name: payload.name,
        phone: payload.phone,
        created_at: now,
    };

    let mut inner = state.inner.lock().await;
    inner.cleaners.push(cleaner.clone());

    Ok(Json(cleaner))
}

pub async fn update_cleaner(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateCleaner>,
) -> Result<Json<Cleaner>, AppError> {
    let mut inner = state.inner.lock().await;

    let cleaner = inner
        .cleaners
        .iter_mut()
        .find(|c| c.id == id)
        .ok_or(AppError::NotFound)?;

    if let Some(name) = payload.name {
        if name.trim().is_empty() {
            return Err(AppError::BadRequest("保洁员姓名不能为空".to_string()));
        }
        cleaner.name = name;
    }
    if let Some(phone) = payload.phone {
        cleaner.phone = phone;
    }

    Ok(Json(cleaner.clone()))
}

pub async fn delete_cleaner(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    let mut inner = state.inner.lock().await;

    let has_tasks = inner.tasks.iter().any(|t| t.cleaner_id == id);
    if has_tasks {
        return Err(AppError::BadRequest(
            "该保洁员有关联的清洁工单，无法删除".to_string(),
        ));
    }

    let initial_len = inner.cleaners.len();
    inner.cleaners.retain(|c| c.id != id);

    if inner.cleaners.len() == initial_len {
        return Err(AppError::NotFound);
    }

    Ok(Json(serde_json::json!({ "message": "删除成功" })))
}

pub async fn list_rooms(State(state): State<AppState>) -> Json<Vec<Room>> {
    let inner = state.inner.lock().await;
    Json(inner.rooms.clone())
}

pub async fn get_room(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Room>, AppError> {
    let inner = state.inner.lock().await;
    inner
        .rooms
        .iter()
        .find(|r| r.id == id)
        .cloned()
        .map(Json)
        .ok_or(AppError::NotFound)
}

pub async fn create_room(
    State(state): State<AppState>,
    Json(payload): Json<CreateRoom>,
) -> Result<Json<Room>, AppError> {
    if payload.room_number.trim().is_empty() {
        return Err(AppError::BadRequest("房间编号不能为空".to_string()));
    }

    let mut inner = state.inner.lock().await;

    if inner.rooms.iter().any(|r| r.room_number == payload.room_number) {
        return Err(AppError::BadRequest("房间编号已存在".to_string()));
    }

    let now = Utc::now().naive_utc();
    let room = Room {
        id: Uuid::new_v4(),
        room_number: payload.room_number,
        floor: payload.floor,
        room_type: payload.room_type,
        status: RoomStatus::Available,
        created_at: now,
    };

    inner.rooms.push(room.clone());

    Ok(Json(room))
}

pub async fn update_room(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateRoom>,
) -> Result<Json<Room>, AppError> {
    let mut inner = state.inner.lock().await;

    let room_idx = inner
        .rooms
        .iter()
        .position(|r| r.id == id)
        .ok_or(AppError::NotFound)?;

    if let Some(room_number) = &payload.room_number {
        if room_number.trim().is_empty() {
            return Err(AppError::BadRequest("房间编号不能为空".to_string()));
        }
        if inner
            .rooms
            .iter()
            .any(|r| r.room_number == *room_number && r.id != id)
        {
            return Err(AppError::BadRequest("房间编号已存在".to_string()));
        }
    }

    let room = &mut inner.rooms[room_idx];

    if let Some(room_number) = payload.room_number {
        room.room_number = room_number;
    }
    if let Some(floor) = payload.floor {
        room.floor = floor;
    }
    if let Some(room_type) = payload.room_type {
        room.room_type = room_type;
    }
    if let Some(status) = payload.status {
        room.status = status;
    }

    Ok(Json(room.clone()))
}

pub async fn delete_room(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    let mut inner = state.inner.lock().await;

    let has_tasks = inner.tasks.iter().any(|t| t.room_id == id);
    if has_tasks {
        return Err(AppError::BadRequest(
            "该房间有关联的清洁工单，无法删除".to_string(),
        ));
    }

    let initial_len = inner.rooms.len();
    inner.rooms.retain(|r| r.id != id);

    if inner.rooms.len() == initial_len {
        return Err(AppError::NotFound);
    }

    Ok(Json(serde_json::json!({ "message": "删除成功" })))
}

pub async fn list_tasks(State(state): State<AppState>) -> Json<Vec<CleaningTaskDetail>> {
    let inner = state.inner.lock().await;

    let details: Vec<CleaningTaskDetail> = inner
        .tasks
        .iter()
        .filter_map(|task| {
            let cleaner = inner.cleaners.iter().find(|c| c.id == task.cleaner_id)?;
            let room = inner.rooms.iter().find(|r| r.id == task.room_id)?;

            Some(CleaningTaskDetail {
                id: task.id,
                cleaner: cleaner.clone(),
                room: room.clone(),
                status: task.status.clone(),
                scheduled_date: task.scheduled_date.clone(),
                remarks: task.remarks.clone(),
                created_at: task.created_at,
                updated_at: task.updated_at,
            })
        })
        .collect();

    Json(details)
}

pub async fn get_task(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<CleaningTaskDetail>, AppError> {
    let inner = state.inner.lock().await;

    let task = inner
        .tasks
        .iter()
        .find(|t| t.id == id)
        .ok_or(AppError::NotFound)?;

    let cleaner = inner
        .cleaners
        .iter()
        .find(|c| c.id == task.cleaner_id)
        .cloned()
        .ok_or(AppError::InternalServerError)?;

    let room = inner
        .rooms
        .iter()
        .find(|r| r.id == task.room_id)
        .cloned()
        .ok_or(AppError::InternalServerError)?;

    Ok(Json(CleaningTaskDetail {
        id: task.id,
        cleaner,
        room,
        status: task.status.clone(),
        scheduled_date: task.scheduled_date.clone(),
        remarks: task.remarks.clone(),
        created_at: task.created_at,
        updated_at: task.updated_at,
    }))
}

pub async fn create_task(
    State(state): State<AppState>,
    Json(payload): Json<CreateCleaningTask>,
) -> Result<Json<CleaningTaskDetail>, AppError> {
    if payload.scheduled_date.trim().is_empty() {
        return Err(AppError::BadRequest("计划日期不能为空".to_string()));
    }

    let mut inner = state.inner.lock().await;

    if !inner.cleaners.iter().any(|c| c.id == payload.cleaner_id) {
        return Err(AppError::BadRequest("指定的保洁员不存在".to_string()));
    }

    if !inner.rooms.iter().any(|r| r.id == payload.room_id) {
        return Err(AppError::BadRequest("指定的房间不存在".to_string()));
    }

    let duplicate = inner.tasks.iter().any(|t| {
        t.room_id == payload.room_id
            && t.scheduled_date == payload.scheduled_date
            && !matches!(t.status, TaskStatus::Completed | TaskStatus::Cancelled)
    });
    if duplicate {
        return Err(AppError::BadRequest(
            "该房间在指定日期已有未完成的清洁工单".to_string(),
        ));
    }

    let now = Utc::now().naive_utc();
    let task = CleaningTask {
        id: Uuid::new_v4(),
        cleaner_id: payload.cleaner_id,
        room_id: payload.room_id,
        status: TaskStatus::Pending,
        scheduled_date: payload.scheduled_date,
        remarks: payload.remarks,
        created_at: now,
        updated_at: now,
    };

    inner.tasks.push(task.clone());

    if let Some(room) = inner.rooms.iter_mut().find(|r| r.id == task.room_id) {
        room.status = RoomStatus::Cleaning;
    }

    let cleaner = inner
        .cleaners
        .iter()
        .find(|c| c.id == task.cleaner_id)
        .cloned()
        .unwrap();

    let room = inner
        .rooms
        .iter()
        .find(|r| r.id == task.room_id)
        .cloned()
        .unwrap();

    Ok(Json(CleaningTaskDetail {
        id: task.id,
        cleaner,
        room,
        status: task.status,
        scheduled_date: task.scheduled_date,
        remarks: task.remarks,
        created_at: task.created_at,
        updated_at: task.updated_at,
    }))
}

pub async fn update_task(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateCleaningTask>,
) -> Result<Json<CleaningTaskDetail>, AppError> {
    let mut inner = state.inner.lock().await;

    let task_idx = inner
        .tasks
        .iter()
        .position(|t| t.id == id)
        .ok_or(AppError::NotFound)?;

    if let Some(cleaner_id) = payload.cleaner_id {
        if !inner.cleaners.iter().any(|c| c.id == cleaner_id) {
            return Err(AppError::BadRequest("指定的保洁员不存在".to_string()));
        }
    }

    if let Some(room_id) = payload.room_id {
        if !inner.rooms.iter().any(|r| r.id == room_id) {
            return Err(AppError::BadRequest("指定的房间不存在".to_string()));
        }
    }

    if let Some(scheduled_date) = &payload.scheduled_date {
        if scheduled_date.trim().is_empty() {
            return Err(AppError::BadRequest("计划日期不能为空".to_string()));
        }
    }

    let current_task = &inner.tasks[task_idx];
    let effective_room_id = payload.room_id.unwrap_or(current_task.room_id);
    let effective_date = payload
        .scheduled_date
        .as_deref()
        .unwrap_or(&current_task.scheduled_date);

    let duplicate = inner.tasks.iter().any(|t| {
        t.id != id
            && t.room_id == effective_room_id
            && t.scheduled_date == effective_date
            && !matches!(t.status, TaskStatus::Completed | TaskStatus::Cancelled)
    });
    if duplicate {
        return Err(AppError::BadRequest(
            "该房间在指定日期已有未完成的清洁工单".to_string(),
        ));
    }

    let current_room_id = inner.tasks[task_idx].room_id;

    if let Some(cleaner_id) = payload.cleaner_id {
        inner.tasks[task_idx].cleaner_id = cleaner_id;
    }

    if let Some(room_id) = payload.room_id {
        inner.tasks[task_idx].room_id = room_id;
    }

    let new_room_id = inner.tasks[task_idx].room_id;

    if let Some(status) = &payload.status {
        match status {
            TaskStatus::Completed => {
                if let Some(room) = inner.rooms.iter_mut().find(|r| r.id == new_room_id) {
                    room.status = RoomStatus::Available;
                }
            }
            TaskStatus::InProgress => {
                if let Some(room) = inner.rooms.iter_mut().find(|r| r.id == new_room_id) {
                    room.status = RoomStatus::Cleaning;
                }
            }
            TaskStatus::Cancelled => {
                let room_has_other_active = inner.tasks.iter().any(|t| {
                    t.id != id
                        && t.room_id == new_room_id
                        && !matches!(t.status, TaskStatus::Completed | TaskStatus::Cancelled)
                });
                if !room_has_other_active {
                    if let Some(room) = inner.rooms.iter_mut().find(|r| r.id == new_room_id) {
                        room.status = RoomStatus::Available;
                    }
                }
                if current_room_id != new_room_id {
                    let old_has_other = inner.tasks.iter().any(|t| {
                        t.room_id == current_room_id
                            && !matches!(t.status, TaskStatus::Completed | TaskStatus::Cancelled)
                    });
                    if !old_has_other {
                        if let Some(room) = inner.rooms.iter_mut().find(|r| r.id == current_room_id) {
                            room.status = RoomStatus::Available;
                        }
                    }
                }
            }
            TaskStatus::Pending => {}
        }
        inner.tasks[task_idx].status = status.clone();
    }

    if let Some(scheduled_date) = payload.scheduled_date {
        inner.tasks[task_idx].scheduled_date = scheduled_date;
    }

    if payload.remarks.is_some() {
        inner.tasks[task_idx].remarks = payload.remarks;
    }

    inner.tasks[task_idx].updated_at = Utc::now().naive_utc();

    let task = inner.tasks[task_idx].clone();
    let cleaner = inner
        .cleaners
        .iter()
        .find(|c| c.id == task.cleaner_id)
        .cloned()
        .unwrap();

    let room = inner
        .rooms
        .iter()
        .find(|r| r.id == task.room_id)
        .cloned()
        .unwrap();

    Ok(Json(CleaningTaskDetail {
        id: task.id,
        cleaner,
        room,
        status: task.status,
        scheduled_date: task.scheduled_date,
        remarks: task.remarks,
        created_at: task.created_at,
        updated_at: task.updated_at,
    }))
}

pub async fn delete_task(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    let mut inner = state.inner.lock().await;

    let task_room_id = inner
        .tasks
        .iter()
        .find(|t| t.id == id)
        .map(|t| t.room_id);

    let initial_len = inner.tasks.len();
    inner.tasks.retain(|t| t.id != id);

    if inner.tasks.len() == initial_len {
        return Err(AppError::NotFound);
    }

    if let Some(room_id) = task_room_id {
        let room_has_active = inner.tasks.iter().any(|t| {
            t.room_id == room_id
                && !matches!(t.status, TaskStatus::Completed | TaskStatus::Cancelled)
        });
        if !room_has_active {
            if let Some(room) = inner.rooms.iter_mut().find(|r| r.id == room_id) {
                room.status = RoomStatus::Available;
            }
        }
    }

    Ok(Json(serde_json::json!({ "message": "删除成功" })))
}

pub async fn update_task_progress(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateProgress>,
) -> Result<Json<CleaningTaskDetail>, AppError> {
    let mut inner = state.inner.lock().await;

    let task_idx = inner
        .tasks
        .iter()
        .position(|t| t.id == id)
        .ok_or(AppError::NotFound)?;

    let current_status = inner.tasks[task_idx].status.clone();
    let room_id = inner.tasks[task_idx].room_id;

    match &payload.status {
        TaskStatus::InProgress => {
            if !matches!(current_status, TaskStatus::Pending) {
                return Err(AppError::BadRequest(
                    "只有待处理的工单才能开始清洁".to_string(),
                ));
            }
        }
        TaskStatus::Completed => {
            if !matches!(current_status, TaskStatus::InProgress) {
                return Err(AppError::BadRequest(
                    "只有进行中的工单才能标记完成".to_string(),
                ));
            }
        }
        TaskStatus::Cancelled => {
            if matches!(current_status, TaskStatus::Completed | TaskStatus::Cancelled) {
                return Err(AppError::BadRequest(
                    "已完成或已取消的工单无法取消".to_string(),
                ));
            }
        }
        TaskStatus::Pending => {
            return Err(AppError::BadRequest(
                "不能将工单状态回退为待处理".to_string(),
            ));
        }
    }

    match &payload.status {
        TaskStatus::InProgress => {
            if let Some(room) = inner.rooms.iter_mut().find(|r| r.id == room_id) {
                room.status = RoomStatus::Cleaning;
            }
        }
        TaskStatus::Completed => {
            if let Some(room) = inner.rooms.iter_mut().find(|r| r.id == room_id) {
                room.status = RoomStatus::Available;
            }
        }
        TaskStatus::Cancelled => {
            let room_has_other_active = inner.tasks.iter().any(|t| {
                t.id != id
                    && t.room_id == room_id
                    && !matches!(t.status, TaskStatus::Completed | TaskStatus::Cancelled)
            });
            if !room_has_other_active {
                if let Some(room) = inner.rooms.iter_mut().find(|r| r.id == room_id) {
                    room.status = RoomStatus::Available;
                }
            }
        }
        TaskStatus::Pending => unreachable!(),
    }

    let task = &mut inner.tasks[task_idx];
    task.status = payload.status;
    task.updated_at = Utc::now().naive_utc();

    let task = task.clone();
    let cleaner = inner
        .cleaners
        .iter()
        .find(|c| c.id == task.cleaner_id)
        .cloned()
        .unwrap();

    let room = inner
        .rooms
        .iter()
        .find(|r| r.id == task.room_id)
        .cloned()
        .unwrap();

    Ok(Json(CleaningTaskDetail {
        id: task.id,
        cleaner,
        room,
        status: task.status,
        scheduled_date: task.scheduled_date,
        remarks: task.remarks,
        created_at: task.created_at,
        updated_at: task.updated_at,
    }))
}
