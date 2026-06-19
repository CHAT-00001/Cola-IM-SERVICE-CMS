// cola_data/src/music/info/music  -- 构建音乐信息
// 2026/5/19 18:17 by wx: cestbon10080

////////

use chrono::Utc;
use serde::{Deserialize, Serialize};

////////

/// # [INFO] - 音乐信息
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MusicInfo {
    pub id: u64,                   // 音乐 ID
    pub uuid: Option<String>,      // UUID
    pub user_id: Option<i64>,      // 用户 ID
    pub actor: Option<String>,     // 作者名称
    pub name: String,              // 名称（母语）
    pub name_en: Option<String>,   // 英语名称
    pub cover_url: Option<String>, // 封面
    pub duration: u32,             // 音乐时长（单位：秒）
    pub release_time: String,      // 发行时间（格式化后的字符串）
    pub href: String,              // 文件位置
    pub add_time: i64,             // 创建时间（兼容PHP旧版毫秒/秒戳，建议统一）
    pub sync_time: i64,            // 同步时间（服务器生成，用于客户端乐观锁校验）
}

////////
impl MusicInfo {

    /// 【创建场景】构造标准的 MusicInfo 并处理前端兜底逻辑
    /// 适用于首次创建音乐时，生成初始的服务器时间与乐观锁版本
    pub fn new(
        id: u64,
        uuid: Option<String>,
        user_id: Option<i64>,
        actor: Option<String>,
        name: String,
        name_en: Option<String>,
        cover_url: Option<String>,
        duration: u32,
        release_time: Option<String>,
        href: String,
    ) -> Self {
        let now = Utc::now();

        // 【避坑提示】如果乐观锁对并发要求高，推荐用 timestamp_millis() 毫秒戳
        // 这里沿用你的秒级戳，但统一了赋值逻辑
        let server_now_time = now.timestamp();

        // 1. 处理作者名称：无 clone 消耗所有权
        let final_actor = actor
            .filter(|a| !a.trim().is_empty())
            .or_else(|| Some(format!("{}创作的原声", name)));

        // 2. 处理发行时间
        let final_release_time = release_time
            .filter(|t| !t.trim().is_empty())
            .unwrap_or_else(|| now.format("%Y-%m").to_string());

        Self {
            id,
            uuid,
            user_id,
            actor: final_actor,
            name,
            name_en,
            cover_url,
            duration,
            release_time: final_release_time,
            href,
            add_time: server_now_time,
            sync_time: server_now_time, // 初始同步时间与创建时间一致
        }
    }

    /// 【查询/浏览场景】从数据库纯净的元数据转换为前端 VO，同时应用文案兜底
    /// 能够严格还原数据库中的真实 `sync_time`，确保客户端乐观锁校验逻辑不失效
    pub fn from_entity(
        id: u64,
        uuid: Option<String>,
        user_id: Option<i64>,
        actor: Option<String>,
        name: String,
        name_en: Option<String>,
        cover_url: Option<String>,
        duration: u32,
        release_time: Option<String>,
        href: String,
        add_time: i64,  // 传入数据库存储的真实创建时间
        sync_time: i64, // 传入数据库存储的真实同步时间（乐观锁版本戳）
    ) -> Self {
        let now = Utc::now();

        // 1. 浏览时动态兜底作者名
        let final_actor = actor
            .filter(|a| !a.trim().is_empty())
            .or_else(|| Some(format!("{}创作的原声", name)));

        // 2. 浏览时动态兜底发行时间
        let final_release_time = release_time
            .filter(|t| !t.trim().is_empty())
            .unwrap_or_else(|| now.format("%Y-%m").to_string());

        Self {
            id,
            uuid,
            user_id,
            actor: final_actor,
            name,
            name_en,
            cover_url,
            duration,
            release_time: final_release_time,
            href,
            add_time,   // 严格透传历史数据
            sync_time,  // 严格透传历史数据，供客户端做乐观锁比对
        }
    }

    /// 【异常兜底场景】当 Repo 未命中、视频关联的音乐被删除或下架时，返回安全的空响应
    pub fn empty() -> Self {
        let now_timestamp = Utc::now().timestamp();

        Self {
            id: 0,                               // 关键：前端通过 id == 0 识别“该音乐已失效/不存在”
            uuid: None,                          // 未命中时无有效 UUID
            user_id: None,                       // 未命中时无有效用户
            actor: Some("未知作者".to_string()),    // 保证展示层有得显示
            name: "音乐不存在".to_string(),        // 显式告知前端
            name_en: None,
            cover_url: None,                     // 空图片，防止前端请求无效资源
            duration: 0,                         // 时长置 0
            release_time: "0000-00".to_string(), // 明显的空时间格式
            href: "".to_string(),                // 文件路径为空，防止前端播放报错
            add_time: now_timestamp,
            sync_time: now_timestamp,
        }
    }
}

// * --------
//////// END