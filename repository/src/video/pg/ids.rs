// repository/src/pg/new/ids.rs --
// 仓储 - VIDEO -  pg - ids
// 2026-05-20 14:20

////////

use crate::pg_pool;
use sqlx::{self, PgPool};
use cola_data::video::entity::video::video::VideoEntity;


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

/// 视频底层仓储驱动
pub struct VideoIdsRepo {
    pool: PgPool,
}

/// # 搜索排序规则枚举（新增：最新发布）
#[derive(Debug, Clone, Copy)]
pub enum SearchOrder {
    Distance,  // 距离最近 (默认)
    MostViews, // 播放量最多
    MostLikes, // 点赞量最多
    Latest,    // 最新发布 👈 新增
}

impl VideoIdsRepo {
    /// 初始化存储实例，注入连接池
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    ////////

    /// # 1. [REPOSITORY] - 浏览记录ids
    /// * `user_id` - 用户ID
    /// * `ids` - 需要检查的视频ID列表
    /// 返回已访问的视频ID列表
    pub async fn find_video_visite_ids(
        user_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<i64>, sqlx::Error> {
        let pool = pg_pool();

        let query = r#"
            SELECT id
            FROM video_visite
            WHERE user_id = $1
                AND status = 1
            ORDER BY add_time DESC
            LIMIT $2
            OFFSET $3
        "#;

        let ids: Vec<i64> = sqlx::query_scalar(query)
            .bind(user_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await?;

        Ok(ids)
    }

    ////////

    /// # 2. [REPOSITORY] - 点赞记录ids
    /// * `user_id` - 用户ID
    /// * `ids` - 需要检查的视频ID列表
    /// 返回已访问的视频ID列表
    pub async fn find_video_like_ids(
        user_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<i64>, sqlx::Error> {
        let pool = pg_pool();

        let query = r#"
            SELECT id
            FROM video_like
            WHERE user_id = $1
                AND status = 1
            ORDER BY add_time DESC
            LIMIT $2
            OFFSET $3
        "#;

        let ids: Vec<i64> = sqlx::query_scalar(query)
            .bind(user_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await?;

        Ok(ids)
    }

    ////////

    /// # 3. [REPOSITORY] - 收藏记录ids
    /// * `user_id` - 用户ID
    /// * `ids` - 需要检查的视频ID列表
    /// 返回已访问的视频ID列表
    pub async fn find_video_collect_ids(
        user_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<i64>, sqlx::Error> {
        let pool = pg_pool();

        let query = r#"
            SELECT id
            FROM video_collect
            WHERE user_id = $1
                AND status = 1
            ORDER BY add_time DESC
            LIMIT $2
            OFFSET $3
        "#;

        let ids: Vec<i64> = sqlx::query_scalar(query)
            .bind(user_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await?;

        Ok(ids)
    }

    ////////

    /// # 4. [REPOSITORY] - 推荐记录ids
    /// * `user_id` - 用户ID
    /// * `ids` - 需要检查的视频ID列表
    /// 返回已访问的视频ID列表
    pub async fn find_video_recommend_ids(
        user_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<i64>, sqlx::Error> {
        let pool = pg_pool();

        let query = r#"
            SELECT id
            FROM video_recommend
            WHERE user_id = $1
                AND status = 1
            ORDER BY add_time DESC
            LIMIT $2
            OFFSET $3
        "#;

        let ids: Vec<i64> = sqlx::query_scalar(query)
            .bind(user_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await?;

        Ok(ids)
    }

    ////////



    /// # 8. [REPOSITORY] - 根据唯一 ID 查找单个视频详情
    /// * 场景：短视频详情页、分享落地页、单个视频状态校验
    pub async fn find_by_id(&self, id: Vec<i64>) -> Result<Option<VideoEntity>, sqlx::Error> {
        let pool = pg_pool(); // 👈 抽出来
        let query = format!(
            "SELECT {} FROM new WHERE id = $1 AND status = 1 LIMIT 1",
            VIDEO_COLUMNS
        );

        sqlx::query_as::<_, VideoEntity>(&query)
            .bind(id)
            .fetch_optional(&pool)
            .await
    }

    ////////

    /// # 9. [REPOSITORY] - 根据 IDs 集合批量查找视频列表 (保持高性能)
    /// * 场景：用户点赞历史列表、收藏夹批量补全、网关层批量解析缓存未命中数据
    pub async fn find_by_ids(
        &self,
        ids: &[i64], // 👈 借用 i64 数组切片，免去所有权克隆损耗
    ) -> Result<Vec<VideoEntity>, sqlx::Error> {
        let pool = pg_pool(); // 👈 抽出来
        // 如果上层传了个空数组，直接返回空结果，避免白跑一趟数据库
        if ids.is_empty() {
            return Ok(vec![]);
        }

        // 💡 工业级 PG 秘籍：在 PostgreSQL 中，使用 = ANY($1) 的性能和可读性远超动态拼接 IN ($1, $2, $3...)
        let query = format!(
            "SELECT {} FROM new WHERE id = ANY($1) AND status = 1",
            VIDEO_COLUMNS
        );

        sqlx::query_as::<_, VideoEntity>(&query)
            .bind(ids) // 👈 sqlx 完美支持直接绑定 &[i64] 作为 PG 的大数组，零拷贝性能极佳
            .fetch_all(&pool)
            .await
    }
}

//////// END
