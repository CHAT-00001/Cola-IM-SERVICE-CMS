// /add.rs  --
// 2026/6/10 19:52

////////

// repository/src/pg/video/video -- 仓储 - 短视频 - PG - video
// 2026/5/20 by wx: cestbon10080

////////

use crate::pg_pool;
use cola_data::video::command::video::VideoCommand;
use cola_data::video::entity::video::VideoEntity;
use sqlx::{self, Postgres, QueryBuilder};

////////

// 数据表原始字段
const VIDEO_COLUMNS: &str = r#"
    id, uuid, show_id, user_id, title, title_at_uids, description, desc_at_uids,
    thumb, thumb_s, href, href_w, original_url, tags, lat, lng, duration,
    width, height, fps, bit, views, likes, steps, collects, comments,
    done_play_qty, visibility, allow_comment, allow_danmaku, shares,
    is_public, status, music_id, goods_id, addtime, created_at, updated_at
"#;

// 局部辅助结构体：用来承接带有“动态计算距离”的数据库返回行
#[derive(Debug, sqlx::FromRow)]
pub struct VideoHomeRow {
    #[sqlx(flatten)] // 自动把标准字段映射进 Entity
    pub entity: VideoEntity,
    #[sqlx(default)]
    pub distance: Option<f64>, // 承接动态计算的距离
}

/// # 搜索排序规则枚举（新增：最新发布）
#[derive(Debug, Clone, Copy)]
pub enum SearchOrder {
    Distance,  // 距离最近 (默认)
    MostViews, // 播放量最多
    MostLikes, // 点赞量最多
    Latest,    // 最新发布
}

/// 视频底层仓储驱动 - 纯静态命名空间外壳
pub struct AddRepository;

impl AddRepository {
    // * --------
    // * --------

    ////////

    ////////

    /// # 1. [REPOSITORY] - 保存
    /// * `desc`: 用户发布视频落库
    pub async fn pg_save_video_by_uid(
        uid: i64,
        cmd: VideoCommand,
        visibility: i16,
    ) -> Result<VideoEntity, sqlx::Error> {
        let pool = pg_pool();

        let query = format!(
            "INSERT INTO video (user_id, title, description, href, visibility, status) \
             VALUES ($1, $2, $3, $4, $5, 1) \
             RETURNING {}",
            VIDEO_COLUMNS
        );

        sqlx::query_as::<_, VideoEntity>(&query)
            .bind(uid)
            .bind(cmd.title)
            .bind(cmd.description) // 👈 简介字段安全入库
            .bind(cmd.href)
            .bind(visibility) // 👈 风控计算后的可见性状态
            .fetch_one(&pool)
            .await
    }

    ////////

    /// # 2. [REPOSITORY] - 编辑
    /// * `desc` :用户编辑视频落库
    pub async fn pg_update_video_by_id(
        video_id: i64,
        cmd: VideoCommand,
        visibility: i16,
    ) -> Result<VideoEntity, sqlx::Error> {
        let pool = pg_pool();

        let query = format!(
            "INSERT INTO video (video_id, title, description, href, visibility, status) \
             VALUES ($1, $2, $3, $4, $5, 1) \
             RETURNING {}",
            VIDEO_COLUMNS
        );

        sqlx::query_as::<_, VideoEntity>(&query)
            .bind(video_id)
            .bind(cmd.title)
            .bind(cmd.description) // 👈 简介字段安全入库
            .bind(cmd.href)
            .bind(visibility) // 👈 风控计算后的可见性状态
            .fetch_one(&pool)
            .await
    }

    ////////

    /// # 4. [REPOSITORY] - 单个软删除
    /// * `desc` :用户删除视频落库
    pub async fn pg_delete_video_by_id(video_id: i64) -> Result<VideoEntity, sqlx::Error> {
        let pool = pg_pool();

        let query = r#"
        UPDATE video
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
        UPDATE video
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
        UPDATE video
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
            "SELECT {} FROM video WHERE uid = $1 AND status = 1 OFFSET $2 LIMIT $3",
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
