// repo_adapter/src/im/chat.rs  -- 适配器 - IM - 聊天
// 2026-07-07

use async_trait::async_trait;
use cola_data::im::port::chat::ChatRepo;
use cola_data::im::command::chat::ChatCommand;
use cola_data::im::command::setting::ChatSettingCommand;
use cola_data::im::info::chat::ChatInfo;
use repo::im::service::chat::ImChatService;

pub struct ChatPortAdapter;

#[async_trait]
impl ChatRepo for ChatPortAdapter {
    async fn add_chat(&self, uid: i64, cmd: ChatCommand) -> anyhow::Result<()> {
        ImChatService::add_chat(uid, &cmd.title).await
    }

    async fn close_chat(&self, _uid: i64, _chat_id: i64) -> anyhow::Result<()> {
        Ok(())
    }

    async fn del_chat(&self, _uid: i64, _chat_id: i64) -> anyhow::Result<()> {
        Ok(())
    }

    async fn set_chat_setting(&self, _uid: i64, _cmd: ChatSettingCommand) -> anyhow::Result<()> {
        Ok(())
    }

    async fn pin_chat(&self, _uid: i64, _chat_id: i64, _is_pinned: bool) -> anyhow::Result<()> {
        Ok(())
    }

    async fn sync_chats(&self, _uid: i64, _offset: i64, _limit: i64) -> anyhow::Result<Vec<ChatInfo>> {
        Ok(vec![])
    }
}