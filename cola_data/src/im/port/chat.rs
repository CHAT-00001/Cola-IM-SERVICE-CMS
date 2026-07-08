// cola_data/src/im/port/chat.rs  -- IM - port - 聊天
// 2026-07-07

use crate::im::command::chat::ChatCommand;
use crate::im::command::setting::ChatSettingCommand;
use crate::im::info::chat::ChatInfo;

#[async_trait::async_trait]
pub trait ChatRepo: Send + Sync {
    /// 添加新聊天
    async fn add_chat(&self, uid: i64, cmd: ChatCommand) -> anyhow::Result<()>;

    /// 关闭聊天
    async fn close_chat(&self, uid: i64, chat_id: i64) -> anyhow::Result<()>;

    /// 删除聊天
    async fn del_chat(&self, uid: i64, chat_id: i64) -> anyhow::Result<()>;

    /// 聊天设置
    async fn set_chat_setting(&self, uid: i64, cmd: ChatSettingCommand) -> anyhow::Result<()>;

    /// 聊天置顶
    async fn pin_chat(&self, uid: i64, chat_id: i64, is_pinned: bool) -> anyhow::Result<()>;

    /// 同步聊天列表
    async fn sync_chats(&self, uid: i64, offset: i64, limit: i64) -> anyhow::Result<Vec<ChatInfo>>;
}