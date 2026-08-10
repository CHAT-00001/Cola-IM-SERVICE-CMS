// adapter/cola_user/cola_user/mod.rs
// 插头 - 用户 - 用户 - 模块
// 2026/8/6 04:19 Created.

////////

use port::cola_music::ColaMusicPort;

pub mod add;
pub mod check;
pub mod get;
pub mod list;
pub mod manage;
pub(crate) mod del;
mod stat;

pub(crate) fn build_user_port() -> ColaMusicPort {
    todo!()
}