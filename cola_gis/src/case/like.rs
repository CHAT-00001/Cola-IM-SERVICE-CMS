// cola_gis/src/case/like.rs  -- GIS - 用例层 - 点赞
// 2026-07-07 11:40

////////

use anyhow::{Result, anyhow};
use repository::cola_gis::service::like::GisLikeService;
use tracing::{info, warn};

////////

/// # [USE CASE] - 点赞 用例
pub struct LikeCase;

impl LikeCase {
    ////////

    /// # 1. [CASE] - 喜欢
    pub async fn case_add_poi_like(uid: i64, poi_id: i64, is_liked: bool) -> Result<()> {
        GisLikeService::save_like_with_update_gis_count(uid, poi_id, is_liked)
            .await
            .map_err(|e| {
                anyhow!(
                    "系统错误: 点赞处理失败 (uid: {}, poi_id: {}, err: {})",
                    uid,
                    poi_id,
                    e
                )
            })?;

        let action = if is_liked { "点赞" } else { "取消点赞" };
        info!("用户 {} {}了兴趣点 {}", uid, action, poi_id);
        Ok(())
    }

    ////////

    /// # 2. [CASE] - 不喜欢
    pub async fn case_add_poi_unlike(uid: i64, poi_id: i64, is_like: bool) -> Result<()> {
        GisLikeService::save_unlike_with_update_gis_count(uid, poi_id, is_like)
            .await
            .map_err(|e| {
                anyhow!(
                    "系统错误: 不喜欢处理失败 (uid: {}, poi_id: {}, err: {})",
                    uid,
                    poi_id,
                    e
                )
            })?;

        let action = if is_like {
            "不喜欢"
        } else {
            "取消不喜欢"
        };
        info!("用户 {} {}了兴趣点 {}", uid, action, poi_id);
        Ok(())
    }
}

//////// END
