// port/src/cola_im/mod.rs
// ⏩️ 端口 - 可乐IM - mod
// 2026-07-07

////////

use crate::cola_im::card::CardRepo;
use crate::cola_im::chat::ChatRepo;
use crate::cola_im::contact::ContactRepo;
use crate::cola_im::contact_request::ContactRequestRepo;
use crate::cola_im::message::MessageRepo;
use std::sync::Arc;

////////

pub mod card; // 用户名片
pub mod chat; // 聊天会话
pub mod contact; // 联系人
pub mod contact_request; // 联系人请求
pub mod message; // 消息

////////

/// # [IM PORT] - 即时通讯
/// * `desc`: `可乐IM ServicePorts`
#[derive(Clone)]
pub struct ColaImPort {
    pub contact: Arc<dyn ContactRepo + Send + Sync + 'static>,
    pub contact_request: Arc<dyn ContactRequestRepo + Send + Sync + 'static>,
    pub card: Arc<dyn CardRepo + Send + Sync + 'static>,
    pub message: Arc<dyn MessageRepo + Send + Sync + 'static>,
    pub chat: Arc<dyn ChatRepo + Send + Sync + 'static>,
}

//////// END
