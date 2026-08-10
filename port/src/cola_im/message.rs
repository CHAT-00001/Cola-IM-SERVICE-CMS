// port/src/cola_im/message.rs
// ⏩️ 端口 - 可乐IM - 消息
// 2026-07-07 14:10 Created.

////////

use cola_data::cola_im::command::message::MessageCommand;
use cola_data::cola_im::info::message::MessageInfo;

////////

#[async_trait::async_trait]
pub trait MessageRepo: Send + Sync {
    /// 发送消息
    async fn send_message(&self, uid: i64, cmd: MessageCommand) -> anyhow::Result<()>;

    /// 增量拉取离线消息
    async fn sync_messages(&self, uid: i64, sync_time: i64, limit: i64) -> anyhow::Result<Vec<MessageInfo>>;

    /// 删除消息
    async fn del_message(&self, uid: i64, msg_id: i64) -> anyhow::Result<()>;

    /// 收藏消息
    async fn collect_message(&self, uid: i64, msg_id: i64) -> anyhow::Result<()>;

    /// 批量删除（按聊天ID）
    async fn del_messages_by_chat(&self, uid: i64, chat_id: i64) -> anyhow::Result<()>;

    /// 批量删除（按用户ID）
    async fn del_messages_by_card(&self, uid: i64, card_id: i64) -> anyhow::Result<()>;

    /// 批量删除（按群组ID）
    async fn del_messages_by_group(&self, uid: i64, group_id: i64) -> anyhow::Result<()>;
}