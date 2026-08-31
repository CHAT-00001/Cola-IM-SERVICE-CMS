// repo_adapter/src/im/mod.rs -- 🔌 适配器 - 可乐 IM - mod
// 2026/8/10 20:00 Updated.

////////

use port::cola_im::ColaImPort;
use std::sync::Arc;

////////

pub mod card;
pub mod chat;
pub mod contact;
pub mod contact_request;
pub mod message;

////////

/// # [BUILD] - 构建 IM Port
/// * `desc`: 构建即时通讯 Port 聚合体，包含联系人、消息、聊天等功能
pub fn build_im_port() -> ColaImPort {
    ColaImPort {
        contact: Arc::new(contact::ContactPortAdapter),
        contact_request: Arc::new(contact_request::ContactRequestPortAdapter),
        card: Arc::new(card::CardPortAdapter),
        message: Arc::new(message::MessagePortAdapter),
        chat: Arc::new(chat::ChatPortAdapter),
    }
}

//////// END
