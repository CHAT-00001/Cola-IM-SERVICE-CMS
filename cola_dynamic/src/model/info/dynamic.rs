// cola_data/src/dynamic/info/dynamic.rs  -- DYNAMIC - Info - 动态信息
// 2026/5/22 20:10 by wx: cestbon10080
// * --------
// * --------

////////

use serde::{Deserialize, Serialize};
use crate::dynamic::entity::dynamic::DynamicEntity;

/// # [INFO] - 动态 - 信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicInfo {
    pub id: i64,                      // ID
    pub user_id: i64,                     // 作者id
    pub r#type: i16,                  // 类型（数据库直接存 i16）
    pub title: String,                // 标题
    pub original_url: Option<String>, // 原文URL
    pub thumb: Option<String>,        // 缩略图（兼容旧版PHP）
    pub video_thumb: Option<String>,  // 视频封面
    pub video_url: Option<String>,    // 视频URL
    pub voice_url: Option<String>,    // 语音URL
    pub length: Option<i64>,          // 语音长度
    pub address: Option<String>,      // 地址
    pub likes: i64,                   // 点赞量
    pub views: i64,                   // 浏览量
    pub comments: i64,                // 评论量
    pub collect: i64,                 // 收藏量
    pub status: i8,                   // 状态
    pub add_time: i64,                // 发布时间
}

////////

/// # [BUILD] - 构建动态信息
impl DynamicInfo {
    /// # 从数据库 Entity 转换为业务 Model
    pub fn from_entity(entity: DynamicEntity) -> Self {

        let cdn_host = "https://cdn.your-app.com";

        // 处理多图
        let full_thumb_list: Vec<String> = entity
            .thumb
            .unwrap_or_default()
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|s| format!("{}/{}", cdn_host, s))
            .collect();

        let thumb_string = if full_thumb_list.is_empty() {
            None
        } else {
            Some(full_thumb_list.join(","))
        };

        // 视频封面取第一张
        let video_thumb = thumb_string
            .as_ref()
            .and_then(|s| s.split(',').next().map(|f| f.to_string()));

        let video_url = entity
            .video_url
            .as_ref()
            .map(|u| format!("{}/{}", cdn_host, u));

        let voice_url = entity
            .voice_url
            .as_ref()
            .map(|u| format!("{}/{}", cdn_host, u));

        let title = entity.title.unwrap_or_else(|| "未命名".to_string());

        Self {
            id: entity.id,
            user_id: entity.user_id,
            r#type: entity.r#type as i16,   // 直接使用数据库类型
            title,
            original_url: entity.original_url,
            thumb: thumb_string,
            video_thumb,
            video_url,
            voice_url,
            length: entity.length,
            address: entity.address,
            likes: entity.likes,
            views: entity.views,
            comments: entity.comments,
            collect: entity.collects,
            status: entity.status as i8,
            add_time: entity.add_time,
        }
    }
}


//////// END

