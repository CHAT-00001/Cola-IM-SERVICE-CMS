// cola_data/src/auth/port/mod.rs  -- 认证中心服务端口
// 2026/6/10 07:45

////////

use std::sync::Arc;

/// # AUTH 上下文模型
#[derive(Clone)]
pub struct AuthServicePorts {
    // 登录接口
    // pub auth_login_port: Arc<dyn SessionPort + Send + Sync + 'static>,
    // // 用户接口
    // pub user_port: Arc<dyn UserPort + Send + Sync + 'static>,
    // // 视频接口
    // pub video_port: Arc<dyn VideoPort + Send + Sync + 'static>,
    // pub state_port: Arc<dyn StatePort + Send + Sync + 'static>,
    // // 会话接口
    // pub session_port: Arc<dyn SessionPort + Send + Sync + 'static>,
    // // 检查权限接口
    // pub check_port: Arc<dyn UserCheckPermissionPort + Send + Sync + 'static>,
}
