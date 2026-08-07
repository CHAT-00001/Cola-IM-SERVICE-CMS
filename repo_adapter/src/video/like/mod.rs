// repo_adapter/src/video/like/mod.rs
// 🔌 插头 - 可乐视频 - 点赞 - 模块
// 2026/8/6 19:21 Created.
////////

use async_trait::async_trait;
use cola_data::video::port::like::LikePort;
use repository::video::service::like::add::VideoLikeAddService;

/////////
pub mod add; // 添加/修改
pub mod alive; // 存活
pub mod del; // 删除
pub mod get; // 获取IDs
pub mod list; // 列表记录
pub mod manage; //管理
pub mod stat; // 统计
