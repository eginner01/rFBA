/// WebSocket 服务器模块
/// 创建和配置 Socket.IO 服务器

use socketioxide::{SocketIo, extract::SocketRef};
use std::sync::Arc;
use tracing::info;

use crate::websocket::handlers::{
    on_task_worker_status, on_ping
};
use crate::websocket::actions::init_socketio_instance;

/// 创建 Socket.IO 服务器
/// 
/// 返回值：(layer, io)
/// - layer: 用于 axum 的中间件层
/// - io: Socket.IO 实例，可用于主动推送消息
pub fn create_socketio_server() -> (socketioxide::layer::SocketIoLayer, Arc<SocketIo>) {
    info!("正在创建 Socket.IO 服务器...");

    // 创建 Socket.IO 服务器
    let (layer, io) = SocketIo::builder()
        .req_path("/ws/socket.io")
        .build_layer();

    let io_arc = Arc::new(io.clone());

    // 初始化全局实例
    init_socketio_instance(io_arc.clone());

    // 注册事件处理器
    io.ns("/", |socket: SocketRef| async move {
        info!("新连接到默认命名空间: socket_id={}", socket.id);

        // 任务 Worker 状态事件
        socket.on("task_worker_status", on_task_worker_status);

        // Ping/Pong 心跳
        socket.on("ping", on_ping);
    });

    info!("Socket.IO 服务器创建完成");
    info!("  路径: /ws/socket.io");
    info!("  命名空间: /");

    (layer, io_arc)
}
