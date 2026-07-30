// /collect.rs  --
// 2026/5/20 20:45 by wx: cestbon10080

////////

// repository/src/pg/video/pg/danmaku.rs  -- 仓储 - 短视频 - PG - 弹幕
// 2026/6/8 16:57

////////

use crate::pg_pool;
use cola_data::video::command::collect::CollectCommand;
use cola_data::video::command::danmaku::DanmakuCommand;
use cola_data::video::entity::danmaku::DanmakuEntity;
use sqlx::{self, Postgres, QueryBuilder};
////////

// 数据表原始字段
const DANMAKU_COLUMNS: &str = r#"
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
    pub entity: DanmakuEntity,
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

/// # [COLLECT REPO] - 收藏 仓储
pub struct CollectRepo;

// 构造函数
impl CollectRepo {
    ////////

    /// # 1. [REPOSITORY] - 添加收藏记录 (防重复)
    /// * `uid`: 用户ID
    /// * `video_id`: 视频ID
    pub async fn save_collect_by_video_id(
        uid: i64,
        video_id: i64,
        cmd: &CollectCommand,
    ) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();

        let now = chrono::Utc::now();
        let timestamp = now.timestamp();
        let datetime = now.naive_utc();

        // 1. 修正：VALUES 必须对应 5 个参数
        let query = "
        INSERT INTO video_collect (user_id, video_id, remark, add_time, created_at)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (user_id, video_id) DO NOTHING
    ";

        // 2. 修正：确保 bind 的顺序与 $1 到 $5 完全一致
        let result = sqlx::query(query)
            .bind(uid)
            .bind(video_id)
            .bind(&cmd.remark) // 对应 $3
            .bind(timestamp)   // 对应 $4
            .bind(datetime)    // 对应 $5
            .execute(&pool)
            .await?;

        Ok(result.rows_affected())
    }

    ////////

    /// # 2. [REPOSITORY] - 删除收藏记录
    /// * `uid`      : 用户 ID
    /// * `video_id` : 视频 ID
    pub async fn delete_collect_by_video_id(
        uid: i64,
        video_id: i64,
    ) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();

        // 1. 使用 DELETE 语句，并通过 WHERE 锁定唯一记录
        let query = "
        DELETE FROM video_collect
        WHERE user_id = $1 AND video_id = $2
    ";

        // 2. 执行删除
        let result = sqlx::query(query)
            .bind(uid)
            .bind(video_id)
            .execute(&pool)
            .await?;

        // 3. 返回受影响行数 (如果返回 1 表示删除成功，0 表示记录不存在)
        Ok(result.rows_affected())
    }

    ////////

    /// # 5. [REPOSITORY] - 根据用户ID查找收藏IDs
    /// *
    pub async fn find_collect_ids_by_user_id(
        user_id: i64,
        keyword: Option<String>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<i64>, sqlx::Error> {
        let pool = pg_pool();

        // 1. 构建基础 SQL
        let mut sql = "SELECT video_id FROM video_collect WHERE user_id = $1".to_string();

        // 2. 动态拼接条件：remark 模糊匹配
        if let Some(ref kw) = keyword {
            if !kw.is_empty() {
                sql.push_str(" AND remark ILIKE $2");
            }
        }

        sql.push_str(" ORDER BY add_time DESC LIMIT $3 OFFSET $4");

        // 3. 构建查询对象 (使用 query_scalar 直接获取 i64)
        let mut query = sqlx::query_scalar::<_, i64>(&sql);

        // 4. 绑定参数
        // 注意：bind 的顺序必须严格遵循 $1, $2, $3, $4
        query = query.bind(user_id); // $1

        // 如果有 keyword，绑定 $2；如果没有，占位符 $2 在 SQL 中不会被用到，绑定逻辑需灵活处理
        if let Some(ref kw) = keyword {
            if !kw.is_empty() {
                query = query.bind(format!("%{}%", kw)); // $2
            }
        } else {
            // 如果没有关键字，为了保证索引对齐，可以绑定一个空值或处理逻辑
            // 但更好的做法是如果没关键字就删掉 SQL 里的 AND remark ILIKE $2
            // 这里提供一种简单处理方式：如果 kw 为空，绑定一个不会匹配到的值或通过 SQL 逻辑规避
            query = query.bind(""); // 兜底绑定
        }

        query = query.bind(limit); // $3
        query = query.bind(offset); // $4

        query.fetch_all(&pool).await
    }

    ////////

    /// # 2. [REPOSITORY] - 获取视频的热门弹幕
    /// * `video_id`: 视频ID
    /// * `limit`: 返回数量限制
    /// * `offset`: 分页偏移量
    /// * 按点赞数降序排序，返回该视频下最热门的弹幕
    pub async fn find_hot_danmaku_by_video_id(
        video_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<DanmakuEntity>, sqlx::Error> {
        let pool = pg_pool();

        let query = format!(
            "SELECT {}
         FROM video_danmaku
         WHERE video_id = $1
           AND status = 1
           AND visibility = 1
         ORDER BY likes DESC, created_at DESC
         LIMIT $2 OFFSET $3",
            DANMAKU_COLUMNS
        );

        sqlx::query_as::<_, DanmakuEntity>(&query)
            .bind(video_id)
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
            DANMAKU_COLUMNS
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

    /// # 5. [REPOSITORY] - 根据用户ID查找自己发布的弹幕
    /// * `user_id`: 用户ID
    /// * `limit`: 返回数量限制
    /// * `offset`: 分页偏移量
    /// * 用于用户管理面板，查看自己发布过的弹幕，不需要风控隔离
    pub async fn find_publish_list_by_user_id(
        user_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<DanmakuEntity>, sqlx::Error> {
        let pool = pg_pool();

        let query = format!(
            "SELECT {}
         FROM video_danmaku
         WHERE user_id = $1
           AND status = 1
         ORDER BY created_at DESC, likes DESC
         LIMIT $2 OFFSET $3",
            DANMAKU_COLUMNS
        );

        sqlx::query_as::<_, DanmakuEntity>(&query)
            .bind(user_id)
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
    ) -> Result<Vec<DanmakuEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM video WHERE status = 1 ORDER BY likes DESC, addtime DESC LIMIT $1 OFFSET $2",
            DANMAKU_COLUMNS
        );

        sqlx::query_as::<_, DanmakuEntity>(&query)
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
            DANMAKU_COLUMNS
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
        let mut query = sqlx::query_as::<_, VideoHomeRow>(&sql).bind(lat).bind(lng);

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
    pub async fn find_by_id(id: i64) -> Result<Option<DanmakuEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM video WHERE id = $1 AND status = 1 LIMIT 1",
            DANMAKU_COLUMNS
        );

        sqlx::query_as::<_, DanmakuEntity>(&query)
            .bind(id)
            .fetch_optional(&pool)
            .await
    }

    ////////

    /// # 9. [REPOSITORY] - 根据 IDs 集合批量查找视频列表 (保持高性能)
    pub async fn find_by_ids(ids: &[i64]) -> Result<Vec<DanmakuEntity>, sqlx::Error> {
        if ids.is_empty() {
            return Ok(vec![]);
        }

        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM video WHERE id = ANY($1) AND status = 1",
            DANMAKU_COLUMNS
        );

        sqlx::query_as::<_, DanmakuEntity>(&query)
            .bind(ids)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 10. [REPOSITORY] - 保存弹幕
    /// * params: video_id
    pub async fn save_danmaku_by_video_id(
        uid: i64,
        video_id: i64, // 视频 ID
        cmd: DanmakuCommand,
        visibility: i16,
    ) -> Result<DanmakuEntity, sqlx::Error> {
        let pool = pg_pool();

        let query = format!(
            "INSERT INTO video_danmaku (user_id, video_id, content, play_time, color, visibility, status) \
         VALUES ($1, $2, $3, $4, $5, $6, 1) \
         RETURNING {}",
            DANMAKU_COLUMNS
        );

        sqlx::query_as::<_, DanmakuEntity>(&query)
            .bind(uid)
            .bind(video_id) // 👈 绑定 video_id
            .bind(cmd.content)
            .bind(cmd.play_time)
            .bind(cmd.color)
            .bind(visibility)
            .fetch_one(&pool)
            .await
    }

    ////////

    /// # 10. [REPOSITORY] - 按视频ID批量删除弹幕
    /// * params: 视频ID
    /// * 视频被删除时触发：删除该视频下的所有弹幕
    pub async fn del_danmaku_by_video_id(video_id: i64) -> Result<u64, sqlx::Error> {
        // 返回删除的弹幕数量

        let pool = pg_pool();

        // 删除指定视频下的所有弹幕
        let query = "DELETE FROM video_danmaku WHERE video_id = $1";

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
        let query = "DELETE FROM video_danmaku WHERE id = $1";

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
        let query = "DELETE FROM video_danmaku WHERE user_id = $1";

        let result = sqlx::query(query).bind(user_id).execute(&pool).await?;

        Ok(result.rows_affected()) // 返回被删除的行数
    }

    ////////

    /// # 12. [REPOSITORY] - 点赞/取消点赞弹幕
    /// * `user_id`: 用户ID
    /// * `danmaku_id`: 弹幕ID
    /// * 如果用户已点赞则取消点赞（-1），未点赞则添加点赞（+1）
    /// * 返回当前弹幕的最新点赞数
    pub async fn update_like_danmaku(user_id: i64, danmaku_id: i64) -> Result<i64, sqlx::Error> {
        let pool = pg_pool();
        let mut tx = pool.begin().await?;

        // 1. 检查用户是否已点赞该弹幕
        let exists: Option<bool> = sqlx::query_scalar(
                "SELECT EXISTS(SELECT 1 FROM video_danmaku_like WHERE user_id = $1 AND danmaku_id = $2)"
            )
                .bind(user_id)
                .bind(danmaku_id)
                .fetch_one(&mut *tx)
                .await?;

        let delta = if exists == Some(true) {
            // 已点赞：取消点赞（删除记录）
            sqlx::query("DELETE FROM video_danmaku_like WHERE user_id = $1 AND danmaku_id = $2")
                .bind(user_id)
                .bind(danmaku_id)
                .execute(&mut *tx)
                .await?;
            -1
        } else {
            // 未点赞：添加点赞记录
            sqlx::query("INSERT INTO video_danmaku_like (user_id, danmaku_id) VALUES ($1, $2)")
                .bind(user_id)
                .bind(danmaku_id)
                .execute(&mut *tx)
                .await?;
            1
        };

        // 2. 更新弹幕表的点赞数字段
        let likes: i64 = sqlx::query_scalar(
            "UPDATE video_danmaku
         SET likes = GREATEST(likes + $1, 0),
             updated_at = NOW()
         WHERE id = $2
         RETURNING likes",
        )
        .bind(delta)
        .bind(danmaku_id)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(likes)
    }

    ////////

    /// # [REPOSITORY] - 批量获取用户对弹幕的点赞状态
    /// * `user_id`: 用户ID
    /// * `danmaku_ids`: 弹幕ID列表
    /// * 返回: HashMap<danmaku_id, is_liked>
    pub async fn get_user_liked_danmaku_map(
        user_id: i64,
        danmaku_ids: &[i64],
    ) -> Result<std::collections::HashMap<i64, bool>, sqlx::Error> {
        let pool = pg_pool();

        if danmaku_ids.is_empty() {
            return Ok(std::collections::HashMap::new());
        }

        // 使用 ANY 数组查询
        let query = r#"
        SELECT danmaku_id, true as is_liked
        FROM video_danmaku_like
        WHERE user_id = $1 AND danmaku_id = ANY($2)
    "#;

        let rows: Vec<(i64, bool)> = sqlx::query_as(query)
            .bind(user_id)
            .bind(danmaku_ids)
            .fetch_all(&pool)
            .await?;

        let liked_map: std::collections::HashMap<i64, bool> = rows.into_iter().collect();

        Ok(liked_map)
    }

    ////////
}

//////// END
