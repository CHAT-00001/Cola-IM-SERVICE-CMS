// repository/src/video/pg/video/add.rs -- 仓储 - VIDEO - PG - 视频 - 发布仓储
// 2026/6/10 19:52

////////

use crate::pg_pool;
use chrono::Utc;
use cola_data::cola_video::command::video::edit::VideoUpdateCommand;
use cola_data::cola_video::command::video::new::VideoNewCommand;
use cola_data::cola_video::command::video::permission::VideoUpdatePermissionCommand;
use cola_data::cola_video::entity::video::video::{VIDEO_COLUMNS, VideoEntity};
use sqlx::{self, Postgres, QueryBuilder};

////////

/// # [ADD REPOSITORY] - 发布
/// * `desc`: `用户发布视频仓储`
pub struct VideoAddRepository;

// 构造实现
impl VideoAddRepository {
    //

    ////////

    /// # 1. [REPOSITORY] - 💾 保存
    /// * `desc`: `用户发布视频落库`
    pub async fn pg_save_video_by_uid(
        uid: i64,
        cmd: VideoNewCommand,
        real_video_id: i64,   // 👈 1. 外部注入真实的视频ID（如雪花ID）
        visibility_perm: i16, // 👈 2. 接收风控计算后的可见性权限（若没有可直接用 cmd 里的）
    ) -> Result<VideoEntity, sqlx::Error> {
        let pool = pg_pool();

        // 💡 核心：先通过 into_entity 转换为完整的实体，统筹默认值、初始计数、时间戳等
        // 如果 Command 里的 visibility_perm 需要被外部参数覆盖，可以在转换后手动赋新值
        let mut entity = cmd.into_entity(uid, real_video_id);
        if visibility_perm > 0 {
            entity.visibility_perm = visibility_perm;
        }

        // 📋 根据 VideoEntity 的全量字段补齐 INSERT 语句（请根据数据库实际表结构增减列名）
        let query = format!(
            "INSERT INTO cola_video.video (
            id, _sn, uid, music_id, title, description, thumbnail, thumb, href, is_4k,
            views, done_views, likes, dislike, comments, danmakus, steps, collects, shares, recommends,
            width, height, bit, addtime, lat, lng,
            visibility_perm, comment_perm, danmaku_perm, collect_perm, download_perm,
            sync_at, status
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9,
            $10, $11, $12, $13, $14, $15, $16, $17, $18, $19,
            $20, $21, $22, $23, $24, $25,
            $26, $27, $28, $29, $30, $31,
            $31, 1
        ) \
         RETURNING {}",
            VIDEO_COLUMNS
        );

        sqlx::query_as::<_, VideoEntity>(&query)
            .bind(entity.id) // $1: id (real_video_id)
            .bind(entity.uid) // $2: uid
            .bind(entity.music_id) // $3: music_id
            .bind(entity.title) // $4: title
            .bind(entity.description) // $5: description
            .bind(entity.thumbnail) // $6: thumbnail (cover_url)
            .bind(entity.thumb) // $7: thumb
            .bind(entity.href) // $8: href
            .bind(entity.is_4k) // $9: is_4k
            .bind(entity.views) // $10: views (默认1)
            .bind(entity.done_views) // $11: done_views
            .bind(entity.likes) // $12: likes
            .bind(entity.dislike) // $13: dislike
            .bind(entity.comments) // $14: comments
            .bind(entity.danmakus) // $15: danmakus
            .bind(entity.steps) // $16: steps
            .bind(entity.collects) // $17: collects
            .bind(entity.shares) // $18: shares
            .bind(entity.recommends) // $19: recommends
            .bind(entity.width) // $20: width
            .bind(entity.height) // $21: height
            .bind(entity.bit) // $22: bit
            .bind(entity.addtime) // $23: addtime (秒级时间戳)
            .bind(entity.lat) // $24: lat
            .bind(entity.lng) // $25: lng
            .bind(entity.visibility_perm) // $26: visibility_perm
            .bind(entity.comment_perm) // $27: comment_perm
            .bind(entity.danmaku_perm) // $28: danmaku_perm
            .bind(entity.collect_perm) // $29: collect_perm
            .bind(entity.download_perm) // $30: download_perm
            .bind(entity.sync_at) // $31: sync_at
            .fetch_one(&pool)
            .await
    }

    ////////

    /// # 2. [REPOSITORY] - 编辑内容
    /// * `desc`: `用户编辑视频落库`
    pub async fn update_content_by_video_id(
        video_id: i64,
        cmd: VideoUpdateCommand,
    ) -> Result<VideoEntity, sqlx::Error> {
        let pool = pg_pool();
        let now = Utc::now();

        let query = format!(
            "UPDATE cola_video.video \
         SET title = $2, description = $3, thumbnail = $4, updated_at = $5 \
         WHERE video_id = $1 \
         RETURNING {}",
            VIDEO_COLUMNS
        );

        sqlx::query_as::<_, VideoEntity>(&query)
            .bind(video_id)
            .bind(cmd.title)
            .bind(cmd.description)
            .bind(cmd.cover_url)
            .bind(now)
            .fetch_one(&pool)
            .await
    }

    ////////

    /// # 3. [REPOSITORY] - 修改权限
    /// * `desc`: `用户修改视频权限落库`
    pub async fn update_permission_by_video_id(
        video_id: i64,
        cmd: VideoUpdatePermissionCommand,
    ) -> Result<VideoEntity, sqlx::Error> {
        let pool = pg_pool();
        let now = Utc::now();

        let query = format!(
            "UPDATE cola_video.video \
         SET visibility_perm = $2, comment_perm = $3, danmaku_perm = $4, collect_perm = $5, download_perm = $6, updated_at = $7 \
         WHERE video_id = $1 \
         RETURNING {}",
            VIDEO_COLUMNS
        );

        sqlx::query_as::<_, VideoEntity>(&query)
            .bind(video_id)
            .bind(cmd.visibility_perm)
            .bind(cmd.comment_perm)
            .bind(cmd.danmaku_perm)
            .bind(cmd.collect_perm)
            .bind(cmd.download_perm)
            .bind(now)
            .fetch_one(&pool)
            .await
    }

    ////////

    /// # 4. [REPOSITORY] - 单个软删除
    /// * `desc` :用户删除视频落库
    pub async fn pg_delete_video_by_id(video_id: i64) -> Result<VideoEntity, sqlx::Error> {
        let pool = pg_pool();

        let query = r#"
        UPDATE cola_video.video
        SET
            is_del = 1,
            del_time = EXTRACT(EPOCH FROM NOW())::BIGINT,
            deleted_at = NOW()
        WHERE video_id = $1 AND is_del = 0
        RETURNING video_id, title, description, href, visibility, status,
                  is_del, del_time, deleted_at, created_at, updated_at
    "#;

        sqlx::query_as::<_, VideoEntity>(query)
            .bind(video_id)
            .fetch_one(&pool)
            .await
    }

    ////////

    /// # 4. [REPOSITORY] - 批量软删除
    /// * `desc` :用户删除视频落库
    pub async fn pg_delete_video_by_ids(video_ids: Vec<i64>) -> Result<VideoEntity, sqlx::Error> {
        let pool = pg_pool();

        let query = r#"
        UPDATE cola_video.video
        SET
            is_del = 1,
            del_time = EXTRACT(EPOCH FROM NOW())::BIGINT,
            deleted_at = NOW()
        WHERE video_id = $1 AND is_del = 0
        RETURNING video_id, title, description, href, visibility, status,
                  is_del, del_time, deleted_at, created_at, updated_at
    "#;

        sqlx::query_as::<_, VideoEntity>(query)
            .bind(video_ids)
            .fetch_one(&pool)
            .await
    }

    ////////

    /// # 12. [REPOSITORY] - 同步更新视频弹幕数量（减指定数量）
    /// * `video_id`: 视频 ID
    /// * `count`: 删除的弹幕数量
    /// * 返回更新后的弹幕数量
    pub async fn sync_decrement_danmaku_count_by_num(
        video_id: i64,
        count: i64,
    ) -> Result<i64, sqlx::Error> {
        let pool = pg_pool();

        let query = r#"
        UPDATE cola_video.video
        SET danmaku_count = GREATEST(danmaku_count - $1, 0),
            updated_at = NOW()
        WHERE id = $2
        RETURNING danmaku_count
    "#;

        let danmaku_count: i64 = sqlx::query_scalar(query)
            .bind(count)
            .bind(video_id)
            .fetch_one(&pool)
            .await?;

        Ok(danmaku_count)
    }

    ////////

    /// # 8. [REPOSITORY] - 查找某个用户发布的视频列表
    pub async fn find_new_list_by_user_id(
        user_id: i64,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<VideoEntity>, sqlx::Error> {
        let pool = pg_pool();

        // 使用参数化查询，避免 SQL 注入
        let query = format!(
            "SELECT {} FROM cola_video.video WHERE uid = $1 AND status = 1 OFFSET $2 LIMIT $3",
            VIDEO_COLUMNS
        );

        sqlx::query_as::<_, VideoEntity>(&query)
            .bind(user_id)
            .bind(offset)
            .bind(limit)
            .fetch_all(&pool) // 使用 fetch_all 获取多条记录
            .await
    }
}

//////// END
