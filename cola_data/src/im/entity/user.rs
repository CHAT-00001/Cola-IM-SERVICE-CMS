// cola_data/src/new/handler/state  -- 可乐数据 - new - handler - 用户
// 2026/5/19 21:34

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 短视频 用户实体
/// * table name: video_user
#[derive(Debug, Clone, Default, FromRow, Serialize, Deserialize)]
pub struct VideoUserEntity {
    // ==== 基础身份标识 ====
    pub uid: i64,                          // 用户ID
    pub send_id: Option<String>,           // 发送 ID UUID （客户端生成）
    pub show_id: Option<String>,           // 显示 ID
    pub sync_id: Option<String>,           // 同步 ID
    pub user_type: i16,                    // 用户类型：1-普通观众, 2-创作者, 3-企业号(蓝V) 👈 新增

    // ==== 核心计数器（高频变动，建议配合 Redis 异步回写） ====
    pub publish_count: i32,                // 发布的视频数量
    pub liked_count: i32,                  // 点赞的视频数量（该用户点赞了多少个视频）
    pub total_favorited_count: i32,        // 获赞总数（该用户的视频被别人点赞的总数） 👈 新增
    pub collected_count: i32,              // 收藏的视频数量（修正原字段名为 collected_count 保持语意一致）
    pub following_count: i32,              // 关注的人数 👈 新增
    pub follower_count: i32,               // 粉丝数量 👈 新增

    // ==== 社交属性扩展 ====
    pub title_at_uids: Option<Vec<i64>>,   // 标题@的IDs (主页常用常驻@人群)

    // ==== 核心状态与风控 ====
    pub status: i16,                       // 状态：1-正常, 2-禁言, 3-封禁 👈 新增
    pub audit_msg: Option<String>,         // 账号封禁/处罚原因说明 👈 新增

    // ==== 细分业务权限控制 ====
    pub publish_perm: Option<String>,      // 发布权限控制（如: 允许发布、禁止发布、需审核后发布）
    pub visite_perm: Option<Vec<i64>>,     // 浏览权限控制（黑白名单控制）
    pub comment_perm: i16,                 // 评论权限：1-允许所有人, 2-仅粉丝, 3-全网禁言
    pub danmaku_perm: i16,                 // 弹幕权限：1-允许, 2-关闭
    pub collect_perm: i16,                 // 收藏权限：1-允许, 2-禁止
    pub download_perm: i16,                // 下载权限：1-允许他人下载原视频, 2-禁止下载

    // ==== 兼容性与时间戳 ====
    pub addtime: i64,                      // 创建时间（兼容旧版PHP）
    pub sync_at: Option<i64>,              // 同步时间
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
}

//////// END