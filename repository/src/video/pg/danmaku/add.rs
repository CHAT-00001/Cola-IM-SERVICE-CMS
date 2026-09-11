// repository/src/pg/danmaku/add.rs  -- 仓储层 - PG - 视频 - 弹幕 - 发布仓储
// 2026/6/8 16:57

////////

use crate::pg_pool;
use cola_data::cola_video::command::danmaku::DanmakuCommand;
use cola_data::cola_video::entity::danmaku::{DanmakuEntity, VIDEO_DANMAKU_COLUMNS};
use sqlx::{self, Postgres, QueryBuilder};

////////

/// # [ADD REPOSITORY] - 视频弹幕发布仓储
pub struct DanmakuAddRepo;

impl DanmakuAddRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - 保存弹幕
    pub async fn save_danmaku_by_video_id(
        uid: i64,
        video_id: i64, // 视频 ID
        cmd: DanmakuCommand,
        visibility: i16,
    ) -> Result<DanmakuEntity, sqlx::Error> {
        let pool = pg_pool();
        let mut entity = cmd.into_entity(uid, video_id);
        entity.visibility = visibility;

        let query = format!(
            "INSERT INTO cola_video.danmaku (id, _id, user_id, video_id, channel_id, content, likes, dislikes, collects, visibility, color, mode, play_time, duration, status, send_time, sync_time, is_deleted, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19) RETURNING {}",
            VIDEO_DANMAKU_COLUMNS
        );

        sqlx::query_as::<_, DanmakuEntity>(&query)
            .bind(entity.id)
            .bind(entity._id)
            .bind(entity.user_id)
            .bind(entity.video_id)
            .bind(entity.channel_id)
            .bind(entity.content)
            .bind(entity.likes)
            .bind(entity.dislikes)
            .bind(entity.collects)
            .bind(entity.visibility)
            .bind(entity.color)
            .bind(entity.mode)
            .bind(entity.play_time)
            .bind(entity.duration)
            .bind(entity.status)
            .bind(entity.send_time)
            .bind(entity.sync_time)
            .bind(entity.is_deleted)
            .bind(entity.created_at)
            .fetch_one(&pool)
            .await
    }

    ////////

    /// # 2. [REPOSITORY] - 按视频ID批量删除弹幕
    /// * params: 视频ID
    /// * 视频被删除时触发：删除该视频下的所有弹幕
    pub async fn del_danmaku_by_video_id(video_id: i64) -> Result<u64, sqlx::Error> {
        // 返回删除的弹幕数量

        let pool = pg_pool();

        // 删除指定视频下的所有弹幕
        let query = "DELETE FROM cola_video.danmaku WHERE video_id = $1";

        let result = sqlx::query(query).bind(video_id).execute(&pool).await?;

        Ok(result.rows_affected()) // 返回被删除的行数
    }

    /// # 4. [REPOSITORY] - 根据弹幕ID删除一条弹幕
    /// * `danmaku_id`: 弹幕 ID
    /// * ``
    pub async fn user_del_danmaku_by_video_id(danmaku_id: i64) -> Result<u64, sqlx::Error> {
        // 返回删除的弹幕数量

        let pool = pg_pool();

        // 删除指定视频下的所有弹幕
        let query = "DELETE FROM cola_video.danmaku WHERE id = $1";

        let result = sqlx::query(query).bind(danmaku_id).execute(&pool).await?;

        Ok(result.rows_affected()) // 返回被删除的行数
    }

    ////////

    /// # 11. [REPOSITORY] - 根据用户ID批量更新弹幕状态
    /// * 用户被封禁时,其UGC全部状态state=0,不可被公开
    /// * params: 视频ID
    /// * 视频被删除时触发：删除该视频下的所有弹幕
    pub async fn update_danmaku_status_by_user_id(user_id: i64) -> Result<u64, sqlx::Error> {
        // 返回删除的弹幕数量

        let pool = pg_pool();

        // 删除指定视频下的所有弹幕
        let query = "DELETE FROM cola_video.danmaku WHERE user_id = $1";

        let result = sqlx::query(query).bind(user_id).execute(&pool).await?;

        Ok(result.rows_affected()) // 返回被删除的行数
    }
}

//////// END
