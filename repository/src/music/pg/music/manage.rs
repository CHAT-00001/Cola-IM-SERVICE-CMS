// repository/src/music/pg/music/manage.rs -- 仓储 - MUSIC - pg - 音乐 - 管理员仓储
// 2026/8/3 20:35 Created.

////////

use crate::pg_pool;
use cola_data::music::entity::music::{MUSIC_COLUMNS, MusicEntity};
use sqlx;

////////

/// [MANAGE REPOSITORY] - 管理
pub struct MusicManageRepo;

impl MusicManageRepo {
    //

    ////////

    /// # 1. [REPO] - 管理列表
    /// * `desc`: `管理员视角列表 - 无视权限 / 状态`
    pub async fn find_manage_list(
        uid: Option<i64>,        // 作者 ID
        keyword: Option<String>, // 关键词
        status: i16,             // 状态码
        limit: i64,              // 数量
        offset: i64,             // 页码
    ) -> Result<Vec<MusicEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_music.music WHERE status = 1 ORDER BY add_time DESC LIMIT $1 OFFSET $2",
            MUSIC_COLUMNS
        );
        sqlx::query_as::<_, MusicEntity>(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }
}

//////// END
