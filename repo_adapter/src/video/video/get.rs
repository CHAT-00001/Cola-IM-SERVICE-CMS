// repo_adapter/src/video/video/get.rs -- 适配器 - VIDEO - 视频内容 - 获取
// 2026/8/6 19:19 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::entity::video::video::VideoEntity;
use cola_data::cola_video::info::video::VideoInfo;
use port::cola_video::video::get::VideoGetPort;
use redis::AsyncCommands;
use repository::video::pg::video::get::VideoGetRepo;
use std::collections::{HashMap, HashSet};
////////

/// # [GET ADAPTER] - 视频内容获取适配器
/// * `desc`: `COLA VIDEO - Get Video Infos Adapter`
pub struct VideoGetAdapter;

const VIDEO_INFO_CACHE_TTL: u64 = 48 * 60 * 60;

fn cache_key(video_id: i64) -> String {
    format!("new:info:{}", video_id)
}

async fn get_cache(video_id: i64) -> Result<Option<VideoInfo>> {
    let db = app_config::GLOBAL_DB
        .get()
        .ok_or_else(|| anyhow::anyhow!("GLOBAL_DB 未初始化"))?;
    let mut conn = db.redis_conn.clone();
    let key = cache_key(video_id);
    let value: Option<String> = conn.get(key).await?;

    value
        .map(|json| serde_json::from_str(&json).map_err(Into::into))
        .transpose()
}

async fn set_cache(info: &VideoInfo) -> Result<()> {
    let db = app_config::GLOBAL_DB
        .get()
        .ok_or_else(|| anyhow::anyhow!("GLOBAL_DB 未初始化"))?;
    let mut conn = db.redis_conn.clone();
    let key = cache_key(info.id);
    let json = serde_json::to_string(info)?;
    let _: () = conn.set_ex(key, json, VIDEO_INFO_CACHE_TTL).await?;
    Ok(())
}

async fn cache_video_infos_by_ids(ids: &[i64]) -> Result<(HashMap<i64, VideoInfo>, Vec<i64>)> {
    let mut hit_map: HashMap<i64, VideoInfo> = HashMap::new();
    let mut miss_ids: Vec<i64> = Vec::new();
    let mut seen_miss: HashSet<i64> = HashSet::new();

    for &video_id in ids {
        if video_id <= 0 {
            continue;
        }

        match get_cache(video_id).await {
            Ok(Some(info)) => {
                tracing::info!("[🔌 ADAPTER] - ⚡️ 视频缓存命中: video_id={}", video_id);
                hit_map.insert(video_id, info);
            }
            Ok(None) => {
                if seen_miss.insert(video_id) {
                    miss_ids.push(video_id);
                }
            }
            Err(error) => {
                tracing::warn!(
                    "[🤐 ADAPTER] - ❌️ 视频缓存读取失败, video_id={}, error={}",
                    video_id,
                    error
                );
                if seen_miss.insert(video_id) {
                    miss_ids.push(video_id);
                }
            }
        }
    }

    Ok((hit_map, miss_ids))
}

// 构造实现
#[async_trait]
impl VideoGetPort for VideoGetAdapter {
    /// # 1. [ADAPTER] - 单个获取视频信息
    /// * `desc`: `先查缓存，未命中则查仓储并回填缓存`
    async fn get_video_info_by_id(&self, _uid: i64, video_id: i64) -> Result<VideoInfo> {
        if video_id <= 0 {
            return Ok(VideoInfo::empty());
        }

        if let Ok(Some(info)) = get_cache(video_id).await {
            tracing::info!("[🔌 ADAPTER] - ✅️ 视频详情缓存命中: video_id={}", video_id);
            return Ok(info);
        }

        let entity = VideoGetRepo::find_video_entity_by_id(video_id).await?;
        let Some(entity) = entity else {
            tracing::info!("[🔌 ADAPTER] - ✅️ 视频不存在: video_id={}", video_id);
            return Ok(VideoInfo::empty());
        };

        let info = VideoInfo::from_entity(entity);
        if let Err(error) = set_cache(&info).await {
            tracing::warn!(
                "[🤐 ADAPTER] - ❌️ 视频详情缓存回填失败: video_id={}, error={}",
                video_id,
                error
            );
        }

        tracing::info!("[🔌 ADAPTER] - ✅️ 视频详情查询成功: video_id={}", video_id);
        Ok(info)
    }

    /// # 2. [ADAPTER] - 批量获取视频信息
    /// * `desc`: `缓存未命中 ID 收集后一次性查库，避免 N+1`
    async fn get_video_infos_by_ids(
        &self,
        _uid: i64,
        video_ids: Vec<i64>,
    ) -> Result<Vec<VideoInfo>> {
        if video_ids.is_empty() {
            return Ok(vec![]);
        }

        let (mut info_map, miss_ids) = cache_video_infos_by_ids(&video_ids).await?;

        if !miss_ids.is_empty() {
            let db_entities: Vec<VideoEntity> =
                VideoGetRepo::find_video_entities_by_ids(&miss_ids).await?;
            for entity in db_entities {
                let info = VideoInfo::from_entity(entity);
                if let Err(error) = set_cache(&info).await {
                    tracing::warn!(
                        "[🤐 ADAPTER] - ❌️ 视频批量缓存回填失败: video_id={}, error={}",
                        info.id,
                        error
                    );
                }
                info_map.insert(info.id, info);
            }
        }

        let total = video_ids.iter().filter(|video_id| **video_id > 0).count();
        let hit_count = info_map.len();

        let infos: Vec<VideoInfo> = video_ids
            .into_iter()
            .filter(|video_id| *video_id > 0)
            .filter_map(|video_id| info_map.get(&video_id).cloned())
            .collect();

        tracing::info!(
            "[🔌 ADAPTER] - ✅️ 视频批量查询完成: total={}, hit={}, miss={}",
            total,
            hit_count,
            miss_ids.len()
        );

        Ok(infos)
    }

    async fn get_my_list(
        &self,
        uid: i64,
        keyword: Option<String>,
        limit: i64,
        offset: i64,
        is_liked: bool,
    ) -> Result<(Vec<VideoInfo>)> {
        todo!()
    }

    async fn get_he_list(
        &self,
        uid: i64,
        user_id: i64,
        keyword: Option<String>,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<VideoInfo>)> {
        todo!()
    }
}

//////// END
