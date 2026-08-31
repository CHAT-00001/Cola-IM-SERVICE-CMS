// repo_adapter/src/cola_im/message.rs -- 🔌 适配器 - 可乐IM - 消息
// 2026-07-07 12:01

////////

use async_trait::async_trait;
use cola_data::cola_im::command::message::MessageCommand;
use cola_data::cola_im::info::message::MessageInfo;
use port::cola_im::message::MessageRepo;
use repository::cola_im::service::message::ImMessageService;

////////

/// # [MESSAGE ADAPTER] - 消息
/// * `desc`: `IM - ✉️ 消息适配器`
pub struct MessagePortAdapter;

#[async_trait]
impl MessageRepo for MessagePortAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 发送消息
    async fn send_message(&self, _uid: i64, cmd: MessageCommand) -> anyhow::Result<()> {
        ImMessageService::send_message(_uid, &cmd.content).await
    }

    ////////

    /// # 2. [ADAPTER] - 同步消息
    async fn sync_messages(
        &self,
        uid: i64,
        sync_time: i64,
        limit: i64,
    ) -> anyhow::Result<Vec<MessageInfo>> {
        ImMessageService::sync_messages(uid, sync_time, limit).await
    }

    ////////

    /// # 3. [ADAPTER] - 删除消息
    async fn del_message(&self, uid: i64, msg_id: i64) -> anyhow::Result<()> {
        ImMessageService::del_message(uid, msg_id).await
    }

    ////////

    /// # 4. [ADAPTER] - 收藏消息
    async fn collect_message(&self, _uid: i64, _msg_id: i64) -> anyhow::Result<()> {
        Ok(())
    }

    ////////

    /// # 5. [ADAPTER] - 根据会话ID删除消息
    async fn del_messages_by_chat(&self, _uid: i64, _chat_id: i64) -> anyhow::Result<()> {
        Ok(())
    }

    ////////

    /// # 6. [ADAPTER] - 根据名片ID删除消息
    async fn del_messages_by_card(&self, _uid: i64, _card_id: i64) -> anyhow::Result<()> {
        Ok(())
    }

    ////////

    /// # 7. [ADAPTER] - 根据群组删除消息
    async fn del_messages_by_group(&self, _uid: i64, _group_id: i64) -> anyhow::Result<()> {
        Ok(())
    }
}

//////// END
