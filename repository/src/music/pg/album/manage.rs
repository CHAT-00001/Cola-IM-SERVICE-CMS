// repository/src/music/pg/album/manage.rs
// 仓储 - MUSIC - pg - 专辑 - 管理仓储
// 2026/8/21 19:20 Created.

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
    /// * `desc`: `管理员视角 - 音乐专辑列表`
    /// * `condition`: `无视权限 / 状态`
    pub async fn find_music_album_manage_list(
        uid: Option<i64>,        // 作者 ID
        keyword: Option<String>, // 关键词
        status: i16,             // 状态码
        limit: i64,              // 数量
        offset: i64,             // 页码
    ) -> Result<Vec<MusicEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_music.album WHERE status = 1 ORDER BY add_time DESC LIMIT $1 OFFSET $2",
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
