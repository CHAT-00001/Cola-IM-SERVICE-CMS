// service/src/cola_gis/like.rs
// 服务 - GIS - 点赞 - 模块

////////

use anyhow::Result;
use repository::cola_gis::pg::count::CountRepo;
use repository::cola_gis::pg::poi_like::LikeRepo;

////////

pub struct GisLikeService;

impl GisLikeService {
    pub async fn save_like_with_update_gis_count(
        uid: i64,
        gis_id: i64,
        is_like: bool,
    ) -> Result<bool> {
        let increment = if is_like { 1 } else { -1 };
        LikeRepo::pg_save_gis_like(uid, gis_id, is_like)
            .await
            .map_err(|e| anyhow::anyhow!("保存点赞记录失败: {}", e))?;
        CountRepo::pg_update_gis_likes(gis_id, increment)
            .await
            .map_err(|e| anyhow::anyhow!("更新点赞计数失败: {}", e))?;
        Ok(true)
    }

    pub async fn save_unlike_with_update_gis_count(
        uid: i64,
        gis_id: i64,
        is_unlike: bool,
    ) -> Result<bool> {
        let increment = if is_unlike { 1 } else { -1 };
        LikeRepo::pg_save_gis_unlike(uid, gis_id, is_unlike)
            .await
            .map_err(|e| anyhow::anyhow!("保存踩记录失败: {}", e))?;
        CountRepo::pg_update_gis_likes(gis_id, increment)
            .await
            .map_err(|e| anyhow::anyhow!("更新点赞计数失败: {}", e))?;
        Ok(true)
    }

    pub async fn get_user_like_ids(
        user_id: i64,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<i64>> {
        let ids = LikeRepo::find_like_record_by_user_id(user_id, limit, offset)
            .await
            .map_err(|e| anyhow::anyhow!("获取用户点赞ID列表失败: {}", e))?;
        Ok(ids)
    }
}

//////// END