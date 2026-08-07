// music/command/album/add.rs
// 数据中心 - MUSIC - command - new 新建
// 2026/8/3 21:46 Created.

////////

use crate::music::entity::album::MusicAlbumEntity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

////////

/// # [CMD] - 创建应用专辑请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMusicAlbumCmd {
    pub uid: i64,                    // 用户ID
    pub name: Option<String>,        // 名称
    pub name_zh: Option<String>,     // 中文名称
    pub description: Option<String>, // 描述
    pub cover_url: Option<String>,   // 封面url
    pub file_url: Option<String>,    // 音频文件url
    pub category_id: Option<i64>,    // 分类ID
    pub tags: Option<String>,        // 标签
    pub source: Option<String>,      // 音乐来源
    pub original: Option<bool>,      // 是否为原创
    pub copyright: Option<String>,   // 版权信息
    pub is_public: Option<bool>,     // 是否公开
}

impl CreateMusicAlbumCmd {
    //

    ////////

    /// # 1. [FROM] - 转换
    /// * `desc`: 创建命令转换为实体
    pub fn to_entity(&self, id_gen: i64) -> MusicAlbumEntity {
        let now_timestamp = chrono::Utc::now().timestamp();

        MusicAlbumEntity {
            id: id_gen,
            uuid: Some(Uuid::new_v4().to_string()),
            uid: self.uid,
            name: self.name.clone(),
            name_zh: self.name_zh.clone(),
            description: self.description.clone(),
            cover_url: self.cover_url.clone(),
            file_url: self.file_url.clone(),
            music_qty: Some(0),
            category_id: self.category_id,
            tags: self.tags.clone(),
            source: self.source.clone(),
            original: self.original,
            copyright: self.copyright.clone(),
            views: 0,
            likes: 0,
            steps: 0,
            collects: 0,
            recommends: 0,
            shares: 0,
            review_reason: None,
            status: 1, // 默认正常或草稿，视业务而定
            is_public: self.is_public,
            is_deleted: Some(false),
            use_nums: Some(0),
            add_time: Some(now_timestamp),
            upd_time: Some(now_timestamp),
            del_time: None,
            rev_time: None,
            created_at: Some(chrono::Utc::now()),
            updated_at: Some(chrono::Utc::now()),
            deleted_at: None,
            reviewed_at: None,
        }
    }
}

//////// END
