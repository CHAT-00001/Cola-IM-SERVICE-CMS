// repo/src/gis/pg/danmaku.rs  -- 仓储 - GIS - pg - 兴趣点 弹幕
// 2026/7/6 14:00

////////

use crate::pg_pool;
use cola_data::gis::command::danmaku::PoiDanmakuCommand;
use cola_data::gis::entity::danmaku::PoiDanmakuEntity;
use sqlx;

////////

const DANMAKU_COLUMNS: &str = r#"
    id, uuid, show_id, user_id, title, title_at_uids, description, desc_at_uids,
    thumb, thumb_s, href, href_w, original_url, tags, lat, lng, duration,
    width, height, fps, bit, views, likes, steps, collects, comments,
    done_play_qty, visibility, allow_comment, allow_danmaku, shares,
    is_public, status, music_id, goods_id, addtime, created_at, updated_at
"#;

////////

/// # [REPOSITORY] - 兴趣点 弹幕 仓储
pub struct PoiDanmakuRepo;

// 构造
impl PoiDanmakuRepo {
    ////////

    /// # 1. [REPOSITORY] - 查找最新的列表
    pub async fn find_danmaku_by_gis_id(
        gis_id: i64,
        play_time: i32,
        time_window: i32,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PoiDanmakuEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_gis.poi_danmaku WHERE gis_id = $1 AND status = 1 AND visibility >= 5 AND play_time BETWEEN $2 - $3 AND $2 + $3 ORDER BY play_time ASC, created_at DESC LIMIT $4 OFFSET $5",
            DANMAKU_COLUMNS
        );
        sqlx::query_as::<_, PoiDanmakuEntity>(&query)
            .bind(gis_id)
            .bind(play_time)
            .bind(time_window)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 3. [REPOSITORY] - 保存新弹幕
    pub async fn save_danmaku_by_gis_id(
        uid: i64,
        gis_id: i64,
        cmd: PoiDanmakuCommand,
        visibility: i16,
    ) -> Result<PoiDanmakuEntity, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "INSERT INTO cola_gis.poi_danmaku (user_id, gis_id, content, play_time, color, visibility, status) \
             VALUES ($1, $2, $3, $4, $5, $6, 1) RETURNING {}",
            DANMAKU_COLUMNS
        );
        sqlx::query_as::<_, PoiDanmakuEntity>(&query)
            .bind(uid)
            .bind(gis_id)
            .bind(cmd.content)
            .bind(cmd.play_time)
            .bind(cmd.color)
            .bind(visibility)
            .fetch_one(&pool)
            .await
    }

    ////////

    /// # 4. [REPOSITORY] - 用户删除弹幕
    pub async fn del_danmaku_by_gis_id(gis_id: i64) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();
        let result = sqlx::query("DELETE FROM cola_gis.poi_danmaku WHERE gis_id = $1")
            .bind(gis_id)
            .execute(&pool)
            .await?;
        Ok(result.rows_affected())
    }

    ////////

    /// # 5. [REPOSITORY] - 同步删除弹幕
    pub async fn user_del_danmaku_by_gis_id(danmaku_id: i64) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();
        let result = sqlx::query("DELETE FROM cola_gis.poi_danmaku WHERE id = $1")
            .bind(danmaku_id)
            .execute(&pool)
            .await?;
        Ok(result.rows_affected())
    }
}

//////// END
