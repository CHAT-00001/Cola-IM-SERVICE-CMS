// repo_adapter/src/gis/like.rs
// 2026-07-07

////////

use async_trait::async_trait;
use cola_data::gis::port::like::LikeRepo;
use repository::gis::service::like::GisLikeService;

////////

/// # [LIKE PORT] - 点赞 端口 插头
pub struct LikePortAdapter;

////////

#[async_trait]
impl LikeRepo for LikePortAdapter {

    ////////

    /// # 1. [PORT] - 保存点赞记录 + 更新点赞数量
    async fn like_poi(
        &self,
        uid: i64,
        poi_id: i64,
        is_liked: bool,
    ) -> anyhow::Result<()> {
        GisLikeService::save_like_with_update_gis_count(uid, poi_id, is_liked)
            .await
            .map_err(|e| anyhow::anyhow!("like_poi failed: {}", e))?;
        Ok(())
    }

    ////////

    /// # 2. [PORT] - 保存不喜欢记录 + 更新不喜欢数量
    async fn unlike_poi(
        &self,
        uid: i64,
        poi_id: i64,
        is_unliked: bool,
    ) -> anyhow::Result<()> {
        GisLikeService::save_unlike_with_update_gis_count(uid, poi_id, is_unliked)
            .await
            .map_err(|e| anyhow::anyhow!("unlike_poi failed: {}", e))?;
        Ok(())
    }
}

//////// END