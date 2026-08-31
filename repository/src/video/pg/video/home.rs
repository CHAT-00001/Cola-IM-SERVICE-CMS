// repository/src/video/pg/video/home.rs -- 仓储 - VIDEO - PG - 视频 - 主页仓储
// 2026/5/20 10:40

////////

use crate::pg_pool;
use cola_data::cola_video::command::video::edit::VideoUpdateCommand;
use cola_data::cola_video::command::video::new::VideoNewCommand;
use cola_data::cola_video::entity::video::video::{VIDEO_COLUMNS, VideoEntity};
use sqlx::{self, Postgres, QueryBuilder};

////////

/// # 搜索排序规则枚举（新增：最新发布）
#[derive(Debug, Clone, Copy)]
pub enum SearchOrder {
    Distance,  // 距离最近 (默认)
    MostViews, // 播放量最多
    MostLikes, // 点赞量最多
    Latest,    // 最新发布
}

/// # [REPOSITORY] - ▶ 视频 仓储
pub struct VideoRepo;

impl VideoRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - ▶ 最新
    pub async fn find_new_list(limit: i64, offset: i64) -> Result<Vec<VideoEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_video.video WHERE status = 1 ORDER BY addtime DESC LIMIT $1 OFFSET $2",
            VIDEO_COLUMNS
        );

        sqlx::query_as::<_, VideoEntity>(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
            .map_err(|e| {
                // 打印完整数据库错误、SQL、参数
                eprintln!(
                    "[DB ERROR] find_new_list | SQL: {} | limit: {} | offset: {} | err: {:?}",
                    query, limit, offset, e
                );
                e
            })
    }

    ////////

    /// # 2. [REPOSITORY] - ▶ 热门
    pub async fn find_hot_list(limit: i64, offset: i64) -> Result<Vec<VideoEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_video.video WHERE status = 1 ORDER BY likes DESC, views DESC, addtime DESC LIMIT $1 OFFSET $2",
            VIDEO_COLUMNS
        );

        sqlx::query_as::<_, VideoEntity>(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 3. [REPOSITORY] - ▶ 随机
    /// * (使用 PostgreSQL 数据库内置随机引擎)
    pub async fn find_recommend_list(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_video.video WHERE status = 1 ORDER BY RANDOM() LIMIT $1 OFFSET $2",
            VIDEO_COLUMNS
        );

        sqlx::query_as::<_, VideoEntity>(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 4. [REPOSITORY] - ▶ 📍 附近(同城)
    /// * 使用lat和lng参数
    pub async fn find_nearby_list(
        lat: f64,
        lng: f64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {}, SQRT(POW(lat - $1, 2) + POW(lng - $2, 2)) AS distance
             FROM cola_video.video
             WHERE status = 1
             ORDER BY distance ASC
             LIMIT $3 OFFSET $4",
            VIDEO_COLUMNS
        );

        sqlx::query_as::<_, VideoEntity>(&query)
            .bind(lat)
            .bind(lng)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 5. [REPOSITORY] - ▶ 🍠 分类（频道）视频列表
    pub async fn find_category_list(
        category_id: i16,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_video.video WHERE status = 1 AND category_id = $1 ORDER BY likes DESC, addtime DESC LIMIT $2 OFFSET $3",
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

    /// # 6. [REPOSITORY] - ▶ ⭐ 精选
    pub async fn find_featured_list(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_video.video WHERE status = 1 ORDER BY likes DESC, addtime DESC LIMIT $1 OFFSET $2",
            VIDEO_COLUMNS
        );

        sqlx::query_as::<_, VideoEntity>(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 7. [REPOSITORY] - ▶ 🔎 搜索关键词 (超级强化版：时间筛选 + 多维可选排序 + 距离计算)
    pub async fn search_keyword_list(
        keyword: String,
        lat: f64,
        lng: f64,
        start_time: Option<i64>,
        end_time: Option<i64>,
        order_by: Option<SearchOrder>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoEntity>, sqlx::Error> {
        let pool = pg_pool();
        let mut sql = format!(
            "SELECT {}, SQRT(POW(lat - $1, 2) + POW(lng - $2, 2)) AS distance FROM cola_video.video WHERE status = 1",
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
        let mut query = sqlx::query_as::<_, VideoEntity>(&sql).bind(lat).bind(lng);

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

    /// # 8. [REPOSITORY] - ▶ 🆔 根据唯一 ID 查找单个视频详情
    pub async fn find_by_id(id: i64) -> Result<Option<VideoEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_video.video WHERE id = $1 AND status = 1 LIMIT 1",
            VIDEO_COLUMNS
        );

        sqlx::query_as::<_, VideoEntity>(&query)
            .bind(id)
            .fetch_optional(&pool)
            .await
    }

    ////////

    /// # 9. [REPOSITORY] - ▶ 🆔 🆔 根据 IDs 集合批量查找视频列表 (保持高性能)
    pub async fn find_by_ids(ids: &[i64]) -> Result<Vec<VideoEntity>, sqlx::Error> {
        if ids.is_empty() {
            return Ok(vec![]);
        }

        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_video.video WHERE id = ANY($1) AND status = 1",
            VIDEO_COLUMNS
        );

        sqlx::query_as::<_, VideoEntity>(&query)
            .bind(ids)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 10. [REPOSITORY] - ▶ ✅️ 保存视频
    /// * 场景：用户发布视频落库
    pub async fn save_video_by_uid(
        uid: i64,
        cmd: VideoNewCommand,
        visibility: i16,
    ) -> Result<VideoEntity, sqlx::Error> {
        let pool = pg_pool();

        let query = format!(
            "INSERT INTO cola_video.video (uid, title, description, href, visibility, status) \
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

    /// # 13. [REPOSITORY] - ▶ 👤 查找某个用户发布的视频列表
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

    ////////

    /// # [REPOSITORY] - 👤 根据用户ID和关键词分页查找视频列表
    pub async fn find_list_by_uid(
        uid: Option<i64>,
        keyword: Option<String>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoEntity>, sqlx::Error> {
        let pool = pg_pool();

        // 使用 QueryBuilder 优雅且安全地拼接动态 SQL
        let mut query_builder: sqlx::QueryBuilder<Postgres> = sqlx::QueryBuilder::new(format!(
            "SELECT {} FROM cola_video.video WHERE status = 1",
            VIDEO_COLUMNS
        ));

        // 1. 动态拼接 uid 条件
        if let Some(user_id) = uid {
            query_builder.push(" AND uid = ");
            query_builder.push_bind(user_id);
        }

        // 2. 动态拼接 keyword 条件
        if let Some(ref kw) = keyword {
            if !kw.is_empty() {
                query_builder.push(" AND (title ILIKE ");
                query_builder.push_bind(format!("%{}%", kw));
                query_builder.push(" OR description ILIKE ");
                query_builder.push_bind(format!("%{}%", kw));
                query_builder.push(")");
            }
        }

        // 3. 拼接排序与分页
        query_builder.push(" ORDER BY addtime DESC LIMIT ");
        query_builder.push_bind(limit);
        query_builder.push(" OFFSET ");
        query_builder.push_bind(offset);

        // 4. 构建并执行
        query_builder
            .build_query_as::<VideoEntity>()
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 15. [REPOSITORY] - ▶ 👤 根据用户IDs查找对象
    /// * 关注的人/朋友/某个用户 复用
    pub async fn find_list_by_uids(
        uids: Option<Vec<i64>>,
        keyword: Option<String>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoEntity>, sqlx::Error> {
        let pool = pg_pool();

        // 1. 构建基础 SQL 和参数列表
        let mut sql = format!(
            "SELECT {} FROM cola_video.video WHERE status = 1",
            VIDEO_COLUMNS
        );

        // 2. 动态拼接条件
        if let Some(ref ids) = uids {
            if !ids.is_empty() {
                sql.push_str(" AND uid = ANY($1)");
            }
        }

        if let Some(ref kw) = keyword {
            if !kw.is_empty() {
                sql.push_str(" AND (title ILIKE $2 OR description ILIKE $2)");
            }
        }

        sql.push_str(" ORDER BY addtime DESC LIMIT $3 OFFSET $4");

        // 3. 执行查询
        let mut query = sqlx::query_as::<_, VideoEntity>(&sql);

        // 4. 按顺序绑定 (注意：SQL 中 $1-$4 必须对应好)
        // 这里使用 bind 链式调用，这是最简单的方法
        query = query.bind(uids.unwrap_or_default());
        query = query.bind(format!("%{}%", keyword.unwrap_or_default()));
        query = query.bind(limit);
        query = query.bind(offset);

        query.fetch_all(&pool).await
    }
}

//////// END
