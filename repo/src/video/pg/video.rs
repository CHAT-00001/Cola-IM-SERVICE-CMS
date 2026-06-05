// repo/src/pg/video/video -- 仓储 - 短视频 - PG - video
// 2026/5/20 by wx: cestbon10080

////////

use crate::pg_pool;
use sqlx::{self};
use cola_data::video::entity::video::VideoEntity;
use cola_data::video::command::video::VideoCommand;

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
pub struct VideoRepo;

impl VideoRepo {

    /// # 1. [REPOSITORY] - 查找最新的列表
    pub async fn find_new_list(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM video WHERE status = 1 ORDER BY addtime DESC LIMIT $1 OFFSET $2",
            VIDEO_COLUMNS
        );

        sqlx::query_as::<_, VideoEntity>(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 2. [REPOSITORY] - 热门
    pub async fn find_hot_list(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM video WHERE status = 1 ORDER BY likes DESC, views DESC, addtime DESC LIMIT $1 OFFSET $2",
            VIDEO_COLUMNS
        );

        sqlx::query_as::<_, VideoEntity>(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 3. [REPOSITORY] - 随机推荐列表
    /// * (使用 PostgreSQL 数据库内置随机引擎)
    pub async fn find_recommend_list(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM video WHERE status = 1 ORDER BY RANDOM() LIMIT $1 OFFSET $2",
            VIDEO_COLUMNS
        );

        sqlx::query_as::<_, VideoEntity>(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 4. [REPOSITORY] - 附近(同城)
    /// * 使用lat和lng参数
    pub async fn find_nearby_list(
        lat: f64,
        lng: f64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoHomeRow>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {}, SQRT(POW(lat - $1, 2) + POW(lng - $2, 2)) AS distance
             FROM video
             WHERE status = 1
             ORDER BY distance ASC
             LIMIT $3 OFFSET $4",
            VIDEO_COLUMNS
        );

        sqlx::query_as::<_, VideoHomeRow>(&query)
            .bind(lat)
            .bind(lng)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 5. [REPOSITORY] - 分类（频道）视频列表
    pub async fn find_category_list(
        category_id: i16,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM video WHERE status = 1 AND category_id = $1 ORDER BY likes DESC, addtime DESC LIMIT $2 OFFSET $3",
            VIDEO_COLUMNS
        );

        sqlx::query_as::<_, VideoEntity>(&query)
            .bind(category_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 6. [REPOSITORY] - 精选
    pub async fn find_featured_list(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM video WHERE status = 1 ORDER BY likes DESC, addtime DESC LIMIT $1 OFFSET $2",
            VIDEO_COLUMNS
        );

        sqlx::query_as::<_, VideoEntity>(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 7. [REPOSITORY] - 搜索关键词 (超级强化版：时间筛选 + 多维可选排序 + 距离计算)
    pub async fn search_keyword_list(
        keyword: &str,
        lat: f64,
        lng: f64,
        start_time: Option<i64>,
        end_time: Option<i64>,
        order_by: Option<SearchOrder>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoHomeRow>, sqlx::Error> {
        let pool = pg_pool();
        let mut sql = format!(
            "SELECT {}, SQRT(POW(lat - $1, 2) + POW(lng - $2, 2)) AS distance FROM video WHERE status = 1",
            VIDEO_COLUMNS
        );

        let mut param_index = 3;

        sql.push_str(&format!(" AND title LIKE ${}", param_index));
        param_index += 1;

        if start_time.is_some() {
            sql.push_str(&format!(" AND addtime >= ${}", param_index));
            param_index += 1;
        }

        if end_time.is_some() {
            sql.push_str(&format!(" AND addtime <= ${}", param_index));
            param_index += 1;
        }

        match order_by.unwrap_or(SearchOrder::Distance) {
            SearchOrder::Distance => sql.push_str(" ORDER BY distance ASC"),
            SearchOrder::MostViews => sql.push_str(" ORDER BY views DESC, distance ASC"),
            SearchOrder::MostLikes => sql.push_str(" ORDER BY likes DESC, distance ASC"),
            SearchOrder::Latest => sql.push_str(" ORDER BY addtime DESC, distance ASC"),
        }

        sql.push_str(&format!(
            " LIMIT ${} OFFSET ${}",
            param_index,
            param_index + 1
        ));

        let keyword_like = format!("%{}%", keyword);
        let mut query = sqlx::query_as::<_, VideoHomeRow>(&sql)
            .bind(lat)
            .bind(lng);

        query = query.bind(&keyword_like);

        if let Some(start) = start_time {
            query = query.bind(start);
        }

        if let Some(end) = end_time {
            query = query.bind(end);
        }

        query.bind(limit).bind(offset).fetch_all(&pool).await
    }

    ////////

    /// # 8. [REPOSITORY] - 根据唯一 ID 查找单个视频详情
    pub async fn find_by_id(id: i64) -> Result<Option<VideoEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM video WHERE id = $1 AND status = 1 LIMIT 1",
            VIDEO_COLUMNS
        );

        sqlx::query_as::<_, VideoEntity>(&query)
            .bind(id)
            .fetch_optional(&pool)
            .await
    }

    ////////

    /// # 9. [REPOSITORY] - 根据 IDs 集合批量查找视频列表 (保持高性能)
    pub async fn find_by_ids(ids: &[i64]) -> Result<Vec<VideoEntity>, sqlx::Error> {
        if ids.is_empty() {
            return Ok(vec![]);
        }

        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM video WHERE id = ANY($1) AND status = 1",
            VIDEO_COLUMNS
        );

        sqlx::query_as::<_, VideoEntity>(&query)
            .bind(ids)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 10. [REPOSITORY] - 保存视频
    /// * 场景：用户发布视频落库
    pub async fn save_video_by_uid(
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
            .bind(visibility)      // 👈 风控计算后的可见性状态
            .fetch_one(&pool)
            .await
    }
}

//////// END