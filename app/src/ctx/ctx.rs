// // cola_video/src/ctx.rs  -- 应用上下文模型
// // 2026/4/23 07:03 by wx: cestbon10080
//
// //////
//
// use std::sync::Arc;
// use crate::auth::port::session::SessionPort;
// use crate::port::add::AddPort;
// use crate::port::danmaku::DanmakuPort;
// use crate::port::home::VideoHomePort;
// use crate::port::video::VideoPort;
// use crate::port::view::ViewPort;
// use crate::user::port::user::UserPort;
// use crate::user::port::check::UserCheckPermissionPort;
// use crate::video::port::add::AddPort;
// use crate::video::port::danmaku::DanmakuPort;
// use crate::video::port::home::VideoHomePort;
// use crate::video::port::video::VideoPort;
// use crate::video::port::view::ViewPort;
//
// //////
//
// #[derive(Clone)]
// pub struct AppContext {
//     pub auth: AuthPorts,
//     pub user: UserPorts,
//     pub video: VideoPorts,
// }
//
// //////
//
// #[derive(Clone)]
// pub struct AuthPorts {
//     pub session: Arc<dyn SessionPort + Send + Sync>,
// }
//
// #[derive(Clone)]
// pub struct UserPorts {
//     pub user: Arc<dyn UserPort + Send + Sync>,
//     pub session: Arc<dyn SessionPort + Send + Sync>,
//     pub check: Arc<dyn UserCheckPermissionPort + Send + Sync>,
// }
//
// //////
//
// #[derive(Clone)]
// pub struct VideoPorts {
//     pub video: Arc<dyn VideoPort + Send + Sync>,
//     pub home: Arc<dyn VideoHomePort + Send + Sync>,
//     pub view: Arc<dyn ViewPort + Send + Sync>,
//     pub danmaku: Arc<dyn DanmakuPort + Send + Sync>,
//     pub add: Arc<dyn AddPort + Send + Sync>,
// }