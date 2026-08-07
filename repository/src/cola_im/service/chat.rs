// repository/src/cola_im/service/chat.rs  -- 仓储 - IM - Service - 聊天
// 2026-07-07

use crate::cola_im::pg::chat::ImChatRepo;

pub struct ImChatService;

impl ImChatService {
    pub async fn add_chat(uid: i64, title: &str) -> anyhow::Result<()> {
        ImChatRepo::save_chat(uid, title).await?;
        Ok(())
    }
}