// repo/src/gis/service/like.rs -- 鏈嶅姟灞?- GIS 鐐硅禐
// 2026/7/6

use crate::gis::pg::count::CountRepo;
use crate::gis::pg::poi_like::LikeRepo;

pub struct GisLikeService;

impl GisLikeService {
    pub async fn save_like_with_update_gis_count(uid: i64, gis_id: i64, is_like: bool) -> Result<bool, sqlx::Error> {
        let increment = if is_like { 1 } else { -1 };
        LikeRepo::pg_save_gis_like(uid, gis_id, is_like).await?;
        CountRepo::pg_update_gis_likes(gis_id, increment).await?;
        Ok(true)
    }

    pub async fn save_unlike_with_update_gis_count(uid: i64, gis_id: i64, is_unlike: bool) -> Result<bool, sqlx::Error> {
        let increment = if is_unlike { 1 } else { -1 };
        LikeRepo::pg_save_gis_unlike(uid, gis_id, is_unlike).await?;
        CountRepo::pg_update_gis_likes(gis_id, increment).await?;
        Ok(true)
    }

    pub async fn get_user_like_ids(user_id: i64, offset: i64, limit: i64) -> Result<Vec<i64>, sqlx::Error> {
        LikeRepo::find_like_record_by_user_id(user_id, limit, offset).await
    }
}

