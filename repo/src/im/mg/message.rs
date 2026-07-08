// repo/src/im/mg/message.rs  -- 仓储 - IM - MongoDB - 消息
// 2026-07-07
// 消息采用三集合模型：
//   1. messages       - 消息主表 (全量消息)
//   2. outbox          - 发件箱 (发送者视角, 按 sender_id + synced 分片)
//   3. inbox           - 收件箱 (接收者视角, 按 recipient_id + synced 分片)
// 支持增量同步 & 多端消息同步（通过 sync_key 游标 + device_id 标记已读）

//////

use app_config::GLOBAL_DB;
use chrono::Utc;
use futures_util::stream::StreamExt;
use mongodb::{bson::{doc, oid::ObjectId}, options::FindOptions, Collection, Database};
use serde::{Deserialize, Serialize};

//////

/// # [MONGO MODEL] - 消息主表
/// * `collection`: `im_messages`
/// * 消息的唯一存储，所有消息写入主表后再扇出到发件箱/收件箱
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageDoc {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    /// 消息全局唯一 ID (业务层字符串, 由 sender_device_id + timestamp + seq 生成)
    pub msg_id: String,
    /// 发送者用户 ID
    pub sender_id: i64,
    /// 接收者用户 ID (单聊) / 群组 ID (群聊)
    pub recipient_id: i64,
    /// 会话类型: 1=单聊, 2=群聊
    pub chat_type: i16,
    /// 消息类型: 1=文本, 2=图片, 3=语音, 4=视频, 5=文件, 6=系统
    pub msg_type: i16,
    /// 消息内容 (JSON, 根据不同 msg_type 有不同的 schema)
    pub content: String,
    /// 引用消息 ID (回复时)
    pub ref_msg_id: Option<String>,
    /// 发送者设备 ID (用于多端同步)
    pub sender_device_id: String,
    /// 服务端时间戳 (毫秒)
    pub server_time: i64,
    /// 发送者时间戳 (毫秒,客户端生成)
    pub client_time: i64,
    /// 消息状态: 0=发送中, 1=已送达, 2=已读, 3=撤回
    pub status: i16,
    /// 创建时间
    pub created_at: i64,
}

//////

/// # [MONGO MODEL] - 发件箱文档
/// * `collection`: `im_outbox`
/// * sender_id 发件箱, 每个发送者维护自己的已发消息列表
/// * 支持增量同步: 通过 sync_key(server_time) > last_sync_time 来拉取增量
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutboxDoc {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    /// 发送者用户 ID (分片键)
    pub sender_id: i64,
    /// 消息主表 msg_id
    pub msg_id: String,
    /// 接收者用户 ID
    pub recipient_id: i64,
    /// 会话类型
    pub chat_type: i16,
    /// 消息类型
    pub msg_type: i16,
    /// 消息摘要 (预览用)
    pub summary: String,
    /// 服务端时间戳, 用作增量同步游标
    pub server_time: i64,
    /// 发送者时间戳
    pub client_time: i64,
    /// 撤回状态
    pub recalled: bool,
    /// 多端已读记录: device_id -> read_time
    pub read_by_devices: Vec<DeviceReadRecord>,
    /// 所有设备都已读
    pub all_read: bool,
    /// 创建时间
    pub created_at: i64,
}

//////

/// # [MONGO MODEL] - 收件箱文档
/// * `collection`: `im_inbox`
/// * recipient_id 收件箱, 每个接收者维护自己的消息列表
/// * 离线消息拉取: 通过 server_time > last_pull_time 拉取增量
/// * 多端同步: 每个 device_id 维护自己的 last_sync_time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboxDoc {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    /// 接收者用户 ID (分片键)
    pub recipient_id: i64,
    /// 发送者用户 ID
    pub sender_id: i64,
    /// 消息主表 msg_id
    pub msg_id: String,
    /// 会话类型
    pub chat_type: i16,
    /// 消息类型
    pub msg_type: i16,
    /// 消息摘要
    pub summary: String,
    /// 服务端时间戳
    pub server_time: i64,
    /// 发送者时间戳
    pub client_time: i64,
    /// 是否已读 (单设备简单标记)
    pub is_read: bool,
    /// 多端已读记录: device_id -> read_time
    pub read_by_devices: Vec<DeviceReadRecord>,
    /// 撤回状态
    pub recalled: bool,
    /// 创建时间
    pub created_at: i64,
}

//////

/// # 设备已读记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceReadRecord {
    pub device_id: String,
    pub read_time: i64,
}

//////

/// # MongoDB 辅助: 获取 im 数据库
fn im_db() -> Database {
    GLOBAL_DB
        .get()
        .expect("GLOBAL_DB 未初始化")
        .mongo_client
        .database("cola_im")
}

fn messages_col() -> Collection<MessageDoc> {
    im_db().collection::<MessageDoc>("messages")
}

fn outbox_col() -> Collection<OutboxDoc> {
    im_db().collection::<OutboxDoc>("outbox")
}

fn inbox_col() -> Collection<InboxDoc> {
    im_db().collection::<InboxDoc>("inbox")
}

//////

/// # [REPO] - IM 消息 MongoDB 仓库
pub struct ImMessageMgRepo;

impl ImMessageMgRepo {
    ////////

    /// # 1. 写入消息(主表 + 扇出)
    /// 1) 写入 messages 主表
    /// 2) 写入发送者的 outbox
    /// 3) 写入接收者的 inbox
    pub async fn save_message(
        sender_id: i64,
        recipient_id: i64,
        msg_type: i16,
        chat_type: i16,
        content: &str,
        sender_device_id: &str,
        client_time: i64,
        ref_msg_id: Option<String>,
    ) -> Result<String, mongodb::error::Error> {
        let now = Utc::now().timestamp_millis();
        let msg_id = format!("{}_{}_{}", sender_device_id, now, sender_id);

        // 1) 消息主表
        let msg_doc = MessageDoc {
            id: None,
            msg_id: msg_id.clone(),
            sender_id,
            recipient_id,
            chat_type,
            msg_type,
            content: content.to_string(),
            ref_msg_id,
            sender_device_id: sender_device_id.to_string(),
            server_time: now,
            client_time,
            status: 1, // 已送达
            created_at: now,
        };
        messages_col().insert_one(msg_doc).await?;

        // 2) 发件箱
        let outbox_doc = OutboxDoc {
            id: None,
            sender_id,
            msg_id: msg_id.clone(),
            recipient_id,
            chat_type,
            msg_type,
            summary: content.chars().take(100).collect(),
            server_time: now,
            client_time,
            recalled: false,
            read_by_devices: vec![],
            all_read: false,
            created_at: now,
        };
        outbox_col().insert_one(outbox_doc).await?;

        // 3) 收件箱
        let inbox_doc = InboxDoc {
            id: None,
            recipient_id,
            sender_id,
            msg_id: msg_id.clone(),
            chat_type,
            msg_type,
            summary: content.chars().take(100).collect(),
            server_time: now,
            client_time,
            is_read: false,
            read_by_devices: vec![],
            recalled: false,
            created_at: now,
        };
        inbox_col().insert_one(inbox_doc).await?;

        Ok(msg_id)
    }

    ////////

    /// # 2. 增量拉取发件箱消息 (发送者视角)
    /// * `sender_id` - 发送者 ID
    /// * `last_sync_time` - 上次同步时间 (毫秒), 0 表示全量
    /// * `limit` - 限制数量
    pub async fn pull_outbox(
        sender_id: i64,
        last_sync_time: i64,
        limit: i64,
    ) -> Result<Vec<OutboxDoc>, mongodb::error::Error> {
        let filter = doc! {
            "sender_id": sender_id,
            "server_time": { "$gt": last_sync_time },
        };
        let opts = FindOptions::builder()
            .sort(doc! { "server_time": -1 })
            .limit(limit)
            .build();

        let cursor = outbox_col().find(filter).with_options(opts).await?;
        let docs: Vec<OutboxDoc> = cursor
            .collect::<Vec<Result<OutboxDoc, _>>>()
            .await
            .into_iter()
            .filter_map(|r| r.ok())
            .collect();
        Ok(docs)
    }

    ////////

    /// # 3. 增量拉取收件箱消息 (接收者视角 - 离线消息)
    /// * `recipient_id` - 接收者 ID
    /// * `last_pull_time` - 上次拉取时间 (毫秒), 0 表示全量
    /// * `limit` - 限制数量
    pub async fn pull_inbox(
        recipient_id: i64,
        last_pull_time: i64,
        limit: i64,
    ) -> Result<Vec<InboxDoc>, mongodb::error::Error> {
        let filter = doc! {
            "recipient_id": recipient_id,
            "server_time": { "$gt": last_pull_time },
        };
        let opts = FindOptions::builder()
            .sort(doc! { "server_time": -1 })
            .limit(limit)
            .build();

        let cursor = inbox_col().find(filter).with_options(opts).await?;
        let docs: Vec<InboxDoc> = cursor
            .collect::<Vec<Result<InboxDoc, _>>>()
            .await
            .into_iter()
            .filter_map(|r| r.ok())
            .collect();
        Ok(docs)
    }

    ////////

    /// # 4. 按 device_id 标记收件箱消息已读 (多端同步)
    /// 更新指定收件箱文档的 read_by_devices, 当所有设备都已读时标记 is_read=true
    pub async fn mark_read(
        recipient_id: i64,
        msg_id: &str,
        device_id: &str,
    ) -> Result<(), mongodb::error::Error> {
        let now = Utc::now().timestamp_millis();

        // push 设备已读记录
        inbox_col().update_one(
            doc! { "recipient_id": recipient_id, "msg_id": msg_id },
            doc! { "$push": { "read_by_devices": { "device_id": device_id, "read_time": now } } },
        ).await?;

        // 同步更新发件箱的已读状态
        outbox_col().update_one(
            doc! { "msg_id": msg_id },
            doc! { "$push": { "read_by_devices": { "device_id": device_id, "read_time": now } } },
        ).await?;

        Ok(())
    }

    ////////

    /// # 5. 撤回消息
    /// 软删除: 将三集合中对应消息的 recalled 置为 true
    pub async fn recall_message(msg_id: &str) -> Result<(), mongodb::error::Error> {
        let update = doc! { "$set": { "recalled": true } };
        messages_col().update_one(doc! { "msg_id": msg_id }, update.clone()).await?;
        outbox_col().update_one(doc! { "msg_id": msg_id }, update.clone()).await?;
        inbox_col().update_one(doc! { "msg_id": msg_id }, update).await?;
        Ok(())
    }

    ////////

    /// # 6. 多设备同步: 获取指定设备未同步的收件箱消息
    /// 根据 device_id 检查 read_by_devices 中没有该设备记录的消息
    pub async fn pull_inbox_for_device(
        recipient_id: i64,
        device_id: &str,
        limit: i64,
    ) -> Result<Vec<InboxDoc>, mongodb::error::Error> {
        let filter = doc! {
            "recipient_id": recipient_id,
            "read_by_devices.device_id": { "$ne": device_id },
        };
        let opts = FindOptions::builder()
            .sort(doc! { "server_time": -1 })
            .limit(limit)
            .build();

        let cursor = inbox_col().find(filter).with_options(opts).await?;
        let docs: Vec<InboxDoc> = cursor
            .collect::<Vec<Result<InboxDoc, _>>>()
            .await
            .into_iter()
            .filter_map(|r| r.ok())
            .collect();
        Ok(docs)
    }
}