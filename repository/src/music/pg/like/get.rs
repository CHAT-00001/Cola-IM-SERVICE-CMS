// repository/src/music/pg/like/get.rs -- 仓储 - MUSIC - PG - 点赞 -  获取仓储
// 🛢️ 仓储 - MUSIC - 最喜欢 - 获取
// 2026/8/23 03:10 Created.

////////

use crate::pg_pool;

////////

pub struct MusicLikeGetRepo;

impl MusicLikeGetRepo {
    /// # 1. [REPOSITORY] - 获取用户最喜欢音乐 IDs
    /// * `desc`: `只返回有效关系，按操作时间倒序分页`
    pub async fn find_music_ids_by_user_id(
        user_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<i64>, sqlx::Error> {
        sqlx::query_scalar(
            "SELECT music_id FROM cola_music.favorites WHERE uid = $1 AND status = 1 AND is_deleted = false ORDER BY add_time DESC LIMIT $2 OFFSET $3",
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&pg_pool())
        .await
    }
}

//////// END