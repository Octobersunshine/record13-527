use axum::{
    routing::get,
    Router,
};
use tower_http::cors::{Any, CorsLayer};

mod error;
mod handlers;
mod models;
mod state;

use state::AppState;

#[tokio::main]
async fn main() {
    let state = AppState::new();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/cleaners", get(handlers::list_cleaners).post(handlers::create_cleaner))
        .route(
            "/api/cleaners/:id",
            get(handlers::get_cleaner)
                .put(handlers::update_cleaner)
                .delete(handlers::delete_cleaner),
        )
        .route("/api/rooms", get(handlers::list_rooms).post(handlers::create_room))
        .route(
            "/api/rooms/:id",
            get(handlers::get_room)
                .put(handlers::update_room)
                .delete(handlers::delete_room),
        )
        .route("/api/tasks", get(handlers::list_tasks).post(handlers::create_task))
        .route(
            "/api/tasks/:id",
            get(handlers::get_task)
                .put(handlers::update_task)
                .delete(handlers::delete_task),
        )
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("服务器启动在 http://127.0.0.1:3000");
    println!();
    println!("API 接口:");
    println!("  保洁人员:");
    println!("    GET    /api/cleaners       - 获取保洁员列表");
    println!("    POST   /api/cleaners       - 创建保洁员");
    println!("    GET    /api/cleaners/:id   - 获取保洁员详情");
    println!("    PUT    /api/cleaners/:id   - 更新保洁员");
    println!("    DELETE /api/cleaners/:id   - 删除保洁员");
    println!();
    println!("  房间:");
    println!("    GET    /api/rooms          - 获取房间列表");
    println!("    POST   /api/rooms          - 创建房间");
    println!("    GET    /api/rooms/:id      - 获取房间详情");
    println!("    PUT    /api/rooms/:id      - 更新房间");
    println!("    DELETE /api/rooms/:id      - 删除房间");
    println!();
    println!("  清洁工单:");
    println!("    GET    /api/tasks          - 获取清洁工单列表");
    println!("    POST   /api/tasks          - 创建清洁工单");
    println!("    GET    /api/tasks/:id      - 获取清洁工单详情");
    println!("    PUT    /api/tasks/:id      - 更新清洁工单");
    println!("    DELETE /api/tasks/:id      - 删除清洁工单");

    axum::serve(listener, app).await.unwrap();
}
