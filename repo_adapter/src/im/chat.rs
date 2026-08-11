// repo_adapter/src/cola_im/chat.rs
// 🔌 适配器 - 可乐IM - 聊天会话
// 2026-07-07 14:10

////////

use async_trait::async_trait;
use cola_data::cola_im::command::chat::ChatCommand;
use cola_data::cola_im::command::setting::ChatSettingCommand;
use cola_data::cola_im::info::chat::ChatInfo;
use port::cola_im::chat::ChatRepo;
use repository::cola_im::service::chat::ImChatService;

////////

/// # [CHAT ADAPTER] -  聊天会话
/// * `desc`: `IM - 💬聊天会话 适配器`
pub struct ChatPortAdapter;

#[async_trait]
impl ChatRepo for ChatPortAdapter {
    //

    ////////

    /// # [ADAPTER] - 添加聊天会话
    async fn add_chat(&self, uid: i64, cmd: ChatCommand) -> anyhow::Result<()> {
        ImChatService::add_chat(uid, &cmd.title).await
    }

    ////////

    /// # [ADAPTER] - 关闭聊天会话
    async fn close_chat(&self, _uid: i64, _chat_id: i64) -> anyhow::Result<()> {
        Ok(())
    }

    ////////

    /// # [ADAPTER] - 删除聊天会话
    async fn del_chat(&self, _uid: i64, _chat_id: i64) -> anyhow::Result<()> {
        Ok(())
    }

    ////////

    /// # [ADAPTER] - 设置聊天
    async fn set_chat_setting(&self, _uid: i64, _cmd: ChatSettingCommand) -> anyhow::Result<()> {
        Ok(())
    }

    ////////

    /// # [ADAPTER] - 置顶聊天会话
    async fn pin_chat(&self, _uid: i64, _chat_id: i64, _is_pinned: bool) -> anyhow::Result<()> {
        Ok(())
    }

    ////////

    /// # [ADAPTER] - 同步聊天列表
    async fn sync_chats(&self, _uid: i64, _offset: i64, _limit: i64) -> anyhow::Result<Vec<ChatInfo>> {
        Ok(vec![])
    }
}

//////// END