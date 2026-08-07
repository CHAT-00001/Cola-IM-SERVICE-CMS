// repo_adapter/src/cola_im/message.rs  -- 适配器 - IM - 消息
// 2026-07-07 12:01

////////

use async_trait::async_trait;
use cola_data::cola_im::port::message::MessageRepo;
use cola_data::cola_im::command::message::MessageCommand;
use cola_data::cola_im::info::message::MessageInfo;
use repository::cola_im::service::message::ImMessageService;

////////


/// # [ADAPTER] - 消息 适配器
pub struct MessagePortAdapter;

#[async_trait]
impl MessageRepo for MessagePortAdapter {
    async fn send_message(&self, _uid: i64, cmd: MessageCommand) -> anyhow::Result<()> {
        ImMessageService::send_message(_uid, &cmd.content).await
    }

    async fn sync_messages(&self, uid: i64, sync_time: i64, limit: i64) -> anyhow::Result<Vec<MessageInfo>> {
        ImMessageService::sync_messages(uid, sync_time, limit).await
    }

    async fn del_message(&self, uid: i64, msg_id: i64) -> anyhow::Result<()> {
        ImMessageService::del_message(uid, msg_id).await
    }

    async fn collect_message(&self, _uid: i64, _msg_id: i64) -> anyhow::Result<()> {
        Ok(())
    }

    async fn del_messages_by_chat(&self, _uid: i64, _chat_id: i64) -> anyhow::Result<()> {
        Ok(())
    }

    async fn del_messages_by_card(&self, _uid: i64, _card_id: i64) -> anyhow::Result<()> {
        Ok(())
    }

    async fn del_messages_by_group(&self, _uid: i64, _group_id: i64) -> anyhow::Result<()> {
        Ok(())
    }
}

//////// END