// repo_adapter/src/cola_user/black/mod.rs
// 🔌 插头 - 可乐用户 - 黑名单 - 模块
// 2026-08-02 Created.

////////

use async_trait::async_trait;
use cola_data::cola_user::info::config::UserConfigInfo;
use cola_data::cola_user::info::user::UserInfo;
use cola_data::cola_user::port::black::add::BlackAddPort;
use cola_data::cola_user::port::black::check::BlackCheckPort;
use cola_data::cola_user::port::black::del::BlackDelPort;
use cola_data::cola_user::port::black::get::BlackGetPort;
use cola_data::cola_user::port::black::list::BlackListPort;
use cola_data::cola_user::port::black::manage::BlackManagePort;

////////

pub mod add;
pub mod check;
pub mod del;
pub mod get;
pub mod list;
pub mod manage;

//////// END
