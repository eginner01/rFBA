/// WebSocket 事件处理器
/// 处理各种 Socket.IO 事件

use socketioxide::extract::{SocketRef, Data};
use serde::{Deserialize, Serialize};
use tracing::{info, error};
use crate::websocket::auth::{authenticate_socket, SocketAuth};

/// Worker 状态数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerStatus {
    pub hostname: String,
    pub ok: bool,
}

/// 任务通知数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskNotification {
    pub msg: String,
}

/// Socket.IO 连接事件处理
pub async fn on_connect(socket: SocketRef, Data(auth): Data<SocketAuth>) {
    info!("新的 WebSocket 连接请求: socket_id={}", socket.id);

    // 验证认证信息
    match authenticate_socket(Some(auth.clone())).await {
        Ok(session_uuid) => {
            info!("WebSocket 连接认证成功: socket_id={}, session={}", socket.id, session_uuid);
            
            // 发送连接成功消息
            let _ = socket.emit("connected", &serde_json::json!({
                "status": "ok",
                "message": "连接成功",
                "session_uuid": session_uuid
            }));
        }
        Err(e) => {
            error!("WebSocket 连接认证失败: {}", e);
            let _ = socket.emit("auth_error", &serde_json::json!({
                "error": e.to_string()
            }));
            socket.disconnect().ok();
        }
    }
}

/// Socket.IO 断开连接事件处理
pub async fn on_disconnect(socket: SocketRef) {
    info!("WebSocket 连接断开: socket_id={}", socket.id);
    // 注意：由于 socketioxide 限制，无法直接存储 session_uuid
    // 实际生产中需要使用 Redis 映射 socket_id -> session_uuid
    // 或者使用 room 机制
}

/// 处理任务 Worker 状态请求
pub async fn on_task_worker_status(socket: SocketRef) {
    info!("收到任务 Worker 状态请求: socket_id={}", socket.id);

    // TODO: 从实际的任务队列获取 Worker 状态
    // 这里暂时返回示例数据
    let workers = vec![
        WorkerStatus {
            hostname: "rust-worker-1".to_string(),
            ok: true,
        },
        WorkerStatus {
            hostname: "rust-worker-2".to_string(),
            ok: true,
        },
    ];

    let _ = socket.emit("task_worker_status", &workers);
}

/// 处理 ping 请求
pub async fn on_ping(socket: SocketRef) {
    let _ = socket.emit("pong", &serde_json::json!({
        "timestamp": chrono::Utc::now().timestamp()
    }));
}
