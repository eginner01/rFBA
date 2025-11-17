/// WebSocket 动作模块
/// 提供主动向客户端推送消息的功能

use socketioxide::SocketIo;
use serde::{Serialize, Deserialize};
use tracing::{info, error};
use once_cell::sync::OnceCell;
use std::sync::Arc;

/// 全局 Socket.IO 实例
static SOCKETIO_INSTANCE: OnceCell<Arc<SocketIo>> = OnceCell::new();

/// 初始化全局 Socket.IO 实例
pub fn init_socketio_instance(io: Arc<SocketIo>) {
    if SOCKETIO_INSTANCE.set(io).is_err() {
        error!("Socket.IO 实例已初始化");
    }
}

/// 获取全局 Socket.IO 实例
fn get_socketio() -> Option<Arc<SocketIo>> {
    SOCKETIO_INSTANCE.get().cloned()
}

/// 任务通知消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskNotificationMsg {
    pub msg: String,
}

/// 向所有连接的客户端发送任务通知
/// 
/// # 示例
/// ```rust
/// use crate::websocket::task_notification;
/// 
/// task_notification("代码生成任务已完成").await;
/// ```
pub async fn task_notification(msg: &str) {
    let io = match get_socketio() {
        Some(io) => io,
        None => {
            error!("Socket.IO 实例未初始化");
            return;
        }
    };

    info!("发送任务通知: {}", msg);

    let data = TaskNotificationMsg {
        msg: msg.to_string(),
    };

    // 向所有连接的客户端广播
    std::mem::drop(io.emit("task_notification", &data));
}

/// 向特定会话发送消息
/// 
/// # 参数
/// * `session_uuid` - 会话 UUID
/// * `event` - 事件名称
/// * `data` - 消息数据
#[allow(dead_code)]
pub async fn emit_to_session<T: Serialize>(session_uuid: String, event: String, data: &T) {
    let io = match get_socketio() {
        Some(io) => io,
        None => {
            error!("Socket.IO 实例未初始化");
            return;
        }
    };

    info!("向会话 {} 发送事件: {}", session_uuid, event);

    // 向特定房间（session_uuid）发送消息
    std::mem::drop(io.to(session_uuid).emit(event, data));
}

/// 向所有客户端广播消息
/// 
/// # 参数
/// * `event` - 事件名称
/// * `data` - 消息数据
#[allow(dead_code)]
pub async fn broadcast<T: Serialize>(event: String, data: &T) {
    let io = match get_socketio() {
        Some(io) => io,
        None => {
            error!("Socket.IO 实例未初始化");
            return;
        }
    };

    info!("广播事件: {}", event);

    std::mem::drop(io.emit(event, data));
}
