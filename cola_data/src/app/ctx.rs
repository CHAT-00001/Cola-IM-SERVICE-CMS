// // cola_video/src/ctx.rs  -- 应用上下文模型
// // 2026/4/23 07:03 by wx: cestbon10080
//
// ////////
//
// use std::sync::Arc;
// use crate::auth::port::session::SessionPort;
// use crate::router::port::check::UserCheckPermissionPort;
// use crate::router::port::app::StatePort;
// use crate::router::port::router::UserPort;
// use crate::live::port::add::AddPort;
// use crate::live::port::live::VideoPort;
// use crate::live::port::view::ViewPort;
//
// ////////
//
// /// # AUTH 上下文模型
// #[derive(Clone)]
// pub struct AuthServicePorts {
//
//     // 登录接口
//     pub auth_login_port: Arc<dyn SessionPort + Send + Sync + 'static>,
//     // 用户接口
//     pub user_port: Arc<dyn UserPort + Send + Sync + 'static>,
//     // 视频接口
//     pub video_port: Arc<dyn VideoPort + Send + Sync + 'static>,
//     pub state_port: Arc<dyn StatePort + Send + Sync + 'static>,
//     // 会话接口
//     pub session_port: Arc<dyn SessionPort + Send + Sync + 'static>,
//     // 检查权限接口
//     pub check_port: Arc<dyn UserCheckPermissionPort + Send + Sync + 'static>,
//
// }
//
//
// /// # VIDEO 上下文模型
// #[derive(Clone)]
// pub struct VideoServicePorts {
//     // 权限验证
//     pub auth_port: Arc<dyn SessionPort + Send + Sync + 'static>,
//     // 用户接口
//     pub user_port: Arc<dyn UserPort + Send + Sync + 'static>,
//     // 视频接口
//     pub video_port: Arc<dyn HomeVideoPort + Send + Sync + 'static>,
//     // 状态接口
//     pub state_port: Arc<dyn StatePort + Send + Sync + 'static>,
//     // 会话接口
//     pub session_port: Arc<dyn SessionPort + Send + Sync + 'static>,
//     // 浏览接口
//     pub view_port: Arc<dyn ViewPort + Send + Sync + 'static>,
//     // 发布接口
//     pub add_port: Arc<dyn AddPort + Send + Sync + 'static>,
//     // 检查权限接口
//     pub check_port: Arc<dyn UserCheckPermissionPort + Send + Sync + 'static>,
// }
//
// /// # APP 上下文模型
// #[derive(Clone)]
// pub struct AppContext {
//     // 认证上下文
//     pub auth_service_ports: AuthServicePorts,
//     // 视频服务接口
//     pub video_service_ports: VideoServicePorts,
// }

use std::sync::Arc;
use crate::auth::port::session::SessionPort;
use crate::user::port::user::UserPort;
use crate::user::port::check::UserCheckPermissionPort;
use crate::video::port::add::AddPort;
use crate::video::port::danmaku::DanmakuPort;
use crate::video::port::video::VideoPort;
use crate::video::port::view::ViewPort;

pub struct AppContext {
    pub user_port: Arc<dyn UserPort + Send + Sync + 'static>,
    pub video_port: Arc<dyn VideoPort + Send + Sync + 'static>,
    pub view_port: Arc<dyn ViewPort + Send + Sync + 'static>,
    pub danmaku_port: Arc<dyn DanmakuPort + Send + Sync + 'static>,
    pub session_port: Arc<dyn SessionPort + Send + Sync + 'static>,
    pub add_port: Arc<dyn AddPort + Send + Sync + 'static>,
    pub check_port: Arc<dyn UserCheckPermissionPort + Send + Sync + 'static>,
}
