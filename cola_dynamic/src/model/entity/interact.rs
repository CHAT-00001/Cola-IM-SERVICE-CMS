// repo/src/dynamic/gateway/interact.rs  -- 动态 - 交互
// 2026/3/31 07:56 by wx: cestbon10080

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::{Uuid, uuid};

////////

/// ## ENTITY - 浏览实体映射
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ViewEntity {
    pub id: i64,                           // id
    pub uid: Option<String>,               // 作者id
    pub dynamic_id: Option<i64>,             // 视频 id
    pub status: i16,                       // 状态: 0. 失效 1. 有效
    pub addtime: Option<i32>,              // 创建时间（兼容PHP旧版）
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
}

/// ## ENTITY - 点赞实体映射
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LikeEntity {
    pub id: i64,                           // id
    pub uid: Option<String>,               // 作者id
    pub dynamic_id: Option<i64>,             // 视频 id
    pub status: i16,                       // 状态: 0. 失效 1. 有效
    pub addtime: Option<i32>,              // 创建时间（兼容PHP旧版）
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
}

/// ## ENTITY - 收藏实体映射
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CollectEntity {
    pub id: i64,                           // id
    pub uid: Option<String>,               // 作者id
    pub dynamic_id: Option<i64>,             // 视频 id
    pub status: i16,                       // 状态: 0. 失效 1. 有效
    pub addtime: Option<i32>,              // 创建时间（兼容PHP旧版）
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
}

/// ## ENTITY - 分享实体映射
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SharesEntity {
    pub id: i64,                           // id
    pub uid: Option<String>,               // 作者id
    pub dynamic_id: Option<i64>,             // 视频 id
    pub status: i16,                       // 状态: 0. 失效 1. 有效
    pub addtime: Option<i32>,              // 创建时间（兼容PHP旧版）
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
}

/// ## ENTITY - 踩实体映射
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct StepEntity {
    pub id: i64,                           // id
    pub uid: Option<String>,               // 作者id
    pub dynamic_id: Option<i64>,             // 视频 id
    pub status: i16,                       // 状态: 0. 失效 1. 有效
    pub addtime: Option<i32>,              // 创建时间（兼容PHP旧版）
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
}

/// ## ENTITY - 举报实体映射
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ReportEntity {
    pub id: i64,                           // id
    pub uid: Option<String>,               // 作者id
    pub dynamic_id: Option<i64>,             // 视频 id
    pub status: i16,                       // 状态: 0. 失效 1. 有效
    pub addtime: Option<i32>,              // 创建时间（兼容PHP旧版）
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
}

/// ## ENTITY - 上热门实体映射
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PushEntity {
    pub id: i64,                           // id
    pub uid: Option<String>,               // 作者id
    pub dynamic_id: Option<i64>,             // 视频 id
    pub status: i16,                       // 状态: 0. 失效 1. 有效
    pub addtime: Option<i32>,              // 创建时间（兼容PHP旧版）
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
}

/// ## ENTITY - 推荐实体映射
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RecommendEntity {
    pub id: i64,                           // id
    pub uid: Option<String>,               // 作者id
    pub dynamic_id: Option<i64>,             // 视频 id
    pub status: i16,                       // 状态: 0. 失效 1. 有效
    pub addtime: Option<i32>,              // 创建时间（兼容PHP旧版）
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
}
