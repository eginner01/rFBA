/// Complete Module DTO
/// 系统完整性检查和功能汇总模块

pub mod system_status;
pub mod module_info;
pub mod health_check;

pub use system_status::{SystemStatusResponse, SystemHealth, ModuleStatus, DatabaseStatus};
pub use module_info::{ModuleInfoResponse, ModuleInfo};
pub use health_check::{HealthCheckResponse, HealthStatus};
