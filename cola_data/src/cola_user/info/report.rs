// cola_data/src/cola_user/info/report.rs
// 可乐数据中心 - 用户 - info - 举报内容信息
// 2026/8/5 20:35 Created.

////////

use crate::cola_user::entity::report::UserReportContentEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

////////

/// # [INFO] - 用户 举报内容信息
/// * `desc`: `安全的举报内容元信息`
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserReportContentInfo {
    pub id: i64,                            // ID
    pub _id: Option<String>,                // UUID v4
    pub uid: i64,                           // 举报人用户ID
    pub target_type: i16,                   // 举报目标类型: 1.用户 2.视频 3.评论等
    pub target_id: i64,                     // 举报目标ID
    pub name: Option<String>,               // 目标快照名称
    pub content: Option<String>,            // 举报内容
    pub images: Option<Vec<String>>,        // 举报凭证图片
    pub review_reason: Option<String>,      // 审核意见/驳回原因
    pub review_status: i16,                 // 审核状态码: 0.待审核 1.审核中 2.已通过 3.已驳回
    pub is_starred: bool,                   // 是否标星/加急
    pub status: i16,                        // 状态码: 0.无效 1.有效
    pub add_time: i64,                      // 添加时间 (兼容旧版PHP)
    pub created_at: Option<DateTime<Utc>>,  // 创建时间
    pub reviewed_at: Option<DateTime<Utc>>, // 审核时间
}

/// # [BUILD] - 构造举报内容信息
impl UserReportContentInfo {
    /// 1. 专门用于返回“举报记录不存在”的空对象
    pub fn empty() -> Self {
        Self {
            id: 0,
            _id: None,
            uid: 0,
            target_type: 0,
            target_id: 0,
            name: Some("举报记录不存在".to_string()),
            content: None,
            images: None,
            review_reason: None,
            review_status: 0,
            is_starred: false,
            status: 0,
            add_time: 0,
            created_at: None,
            reviewed_at: None,
        }
    }

    /// 2. 纯粹的从数据库实体转换（过滤掉敏感内部字段）
    pub fn from_entity(entity: UserReportContentEntity) -> Self {
        Self {
            id: entity.id,
            _id: entity._id,
            uid: entity.uid,
            target_type: entity.target_type,
            target_id: entity.target_id,
            name: entity.name,
            content: entity.content,
            images: entity.images,
            review_reason: entity.review_reason,
            review_status: entity.review_status,
            is_starred: entity.is_starred,
            status: entity.status,
            add_time: entity.add_time,
            created_at: entity.created_at,
            reviewed_at: entity.reviewed_at,
        }
    }
}

//////// END
