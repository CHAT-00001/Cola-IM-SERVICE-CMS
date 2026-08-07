// repository/src/cola_im/service/message.rs  -- 仓储 - IM - Service - 消息
// 2026-07-07

////////

use cola_data::cola_im::info::message::MessageInfo;
use repository::cola_im::pg::message::ImMessageRepo;

////////

/// # [MESSAGE SERVICE] - 消息
/// * `desc`: `消息服务`
pub struct ImMessageService;

impl ImMessageService {
    pub async fn send_message(uid: i64, content: &str) -> anyhow::Result<()> {
        let entity = ImMessageRepo::save_message(uid, content).await?;
        let _ = MessageInfo::from_entity(entity);
        Ok(())
    }

    pub async fn sync_messages(uid: i64, sync_time: i64, limit: i64) -> anyhow::Result<Vec<MessageInfo>> {
        let entities = ImMessageRepo::find_messages_by_uid(uid, sync_time, limit).await?;
        Ok(entities.into_iter().map(MessageInfo::from_entity).collect())
    }

    pub async fn del_message(uid: i64, msg_id: i64) -> anyhow::Result<()> {
        ImMessageRepo::soft_delete_message(uid, msg_id).await?;
        Ok(())
    }
}

//////// END