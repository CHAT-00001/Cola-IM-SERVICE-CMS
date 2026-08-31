// repository/src/pg/danmaku/danmaku.rs  -- 弹幕仓储
// 2026/6/8 16:57

////////

use crate::pg_pool;
use cola_data::cola_video::command::danmaku::DanmakuCommand;
use cola_data::cola_video::entity::danmaku::{DanmakuEntity, VIDEO_DANMAKU_COLUMNS};
use sqlx::{self, Postgres, QueryBuilder};

////////

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

/// 视频底层仓储驱动 - 纯静态命名空间外壳
pub struct DanmakuRepo;

impl DanmakuRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - 根据视频ID和播放时间获取弹幕列表
    /// * `video_id`: 视频ID
    /// * `play_time`: 当前播放时间（秒）
    /// * `time_window`: 时间窗口（秒），例如获取播放时间前后5秒内的弹幕
    /// * `limit`: 返回数量限制
    /// * `offset`: 分页偏移量
    /// * 只返回 visibility >= 5 的弹幕（公众可见）
    pub async fn find_danmaku_by_video_id(
        video_id: i64,
        play_time: i32,
        time_window: i32,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<DanmakuEntity>, sqlx::Error> {
        let pool = pg_pool();

        let query = format!(
            "SELECT {}
         FROM video_danmaku
         WHERE video_id = $1
           AND status = 1
           AND visibility >= 5
           AND play_time BETWEEN $2 - $3 AND $2 + $3
         ORDER BY play_time ASC, created_at DESC
         LIMIT $4 OFFSET $5",
            VIDEO_DANMAKU_COLUMNS
        );

        sqlx::query_as::<_, DanmakuEntity>(&query)
            .bind(video_id)
            .bind(play_time)
            .bind(time_window)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 2. [REPOSITORY] - 根据用户ID他发布获取弹幕列表
    /// * `user_id`: 视频ID
    /// * `limit`: 返回数量限制
    /// * `offset`: 分页偏移量
    /// * 返回全部
    pub async fn find_danmaku_by_user_id(
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
         ORDER BY play_time ASC, created_at DESC
         LIMIT $4 OFFSET $5",
            VIDEO_DANMAKU_COLUMNS
        );

        sqlx::query_as::<_, DanmakuEntity>(&query)
            .bind(user_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 5. [REPOSITORY] - 根据用户IDs查找对象
    /// * 关注的人/朋友/某个用户 复用
    pub async fn find_list_by_uids(
        uids: Option<Vec<i64>>,
        keyword: Option<String>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<DanmakuEntity>, sqlx::Error> {
        let pool = pg_pool();

        // 1. 构建基础 SQL 和参数列表
        let mut sql = format!("SELECT {} FROM new WHERE status = 1", VIDEO_DANMAKU_COLUMNS);

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
        let mut query = sqlx::query_as::<_, DanmakuEntity>(&sql);

        // 4. 按顺序绑定 (注意：SQL 中 $1-$4 必须对应好)
        // 这里使用 bind 链式调用，这是最简单的方法
        query = query.bind(uids.unwrap_or_default());
        query = query.bind(format!("%{}%", keyword.unwrap_or_default()));
        query = query.bind(limit);
        query = query.bind(offset);

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
            VIDEO_DANMAKU_COLUMNS
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
             FROM new
             WHERE status = 1
             ORDER BY distance ASC
             LIMIT $3 OFFSET $4",
            VIDEO_DANMAKU_COLUMNS
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
            VIDEO_DANMAKU_COLUMNS
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
            "SELECT {} FROM new WHERE status = 1 ORDER BY likes DESC, addtime DESC LIMIT $1 OFFSET $2",
            VIDEO_DANMAKU_COLUMNS
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
            "SELECT {}, SQRT(POW(lat - $1, 2) + POW(lng - $2, 2)) AS distance FROM new WHERE status = 1",
            VIDEO_DANMAKU_COLUMNS
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
            "SELECT {} FROM new WHERE id = $1 AND status = 1 LIMIT 1",
            VIDEO_DANMAKU_COLUMNS
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
            "SELECT {} FROM new WHERE id = ANY($1) AND status = 1",
            VIDEO_DANMAKU_COLUMNS
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
            VIDEO_DANMAKU_COLUMNS
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
