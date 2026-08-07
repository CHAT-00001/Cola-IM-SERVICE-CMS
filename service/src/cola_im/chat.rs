// service/src/cola_im/chat/chat.rs
// 服务 - 可乐IM - 聊天 - 模块
// 2026-07-07

////////

use repository::cola_im::pg::chat::ImChatRepo;

////////

pub struct ImChatService;

impl ImChatService {
    pub async fn add_chat(uid: i64, title: &str) -> anyhow::Result<()> {
        ImChatRepo::save_chat(uid, title).await?;
        Ok(())
    }
}

//////// END