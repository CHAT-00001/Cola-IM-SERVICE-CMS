// adapter/user/user/music.rs
// 插头 - 用户 - 用户 - 模块
// 2026/8/6 04:19 Created.

////////

use port::cola_music::ColaMusicPort;

pub mod add;
pub mod check;
pub(crate) mod del;
pub mod get;
pub mod list;
pub mod manage;
mod stat;

pub(crate) fn build_user_port() -> ColaMusicPort {
    todo!()
}
