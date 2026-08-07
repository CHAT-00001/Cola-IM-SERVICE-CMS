// repository/src/cola_video/pg/video/get.rs
// 仓储 - 🎥 可乐视频 - pg - 视频 - 获取
// 2026/8/2 13:01 Created.

////////

use crate::pg_pool;
use cola_data::cola_video::entity::video::video::{VIDEO_COLUMNS, VideoEntity};

////////

/// # 搜索排序规则枚举（新增：最新发布）
#[derive(Debug, Clone, Copy)]
pub enum SearchOrder {
    Distance,  // 距离最近 (默认)
    MostViews, // 播放量最多
    MostLikes, // 点赞量最多
    Latest,    // 最新发布
}

/// # [REPOSITORY] - 🎥 视频 IDs 仓储
pub struct VideoGetRepo;

impl VideoGetRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - ▶ 🆔 单个
    /// `desc`: `根据视频ID 单个查找记录`
    pub async fn find_an_single_by_id(
        id: i64, // 视频 ID
    ) -> Result<Option<VideoEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_video.cola_video WHERE id = $1 AND status = 1 LIMIT 1",
            VIDEO_COLUMNS
        );

        sqlx::query_as::<_, VideoEntity>(&query)
            .bind(id)
            .fetch_optional(&pool)
            .await
    }

    ////////

    /// # 2. [REPOSITORY] - ▶ 🆔 批量
    /// `desc`; ` 根据视频IDs 批量查找记录`
    pub async fn find_list_batch_by_ids(
        ids: &[i64], // 视频 IDs
    ) -> Result<Vec<VideoEntity>, sqlx::Error> {
        if ids.is_empty() {
            return Ok(vec![]);
        }

        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_video.cola_video WHERE id = ANY($1) AND status = 1",
            VIDEO_COLUMNS
        );

        sqlx::query_as::<_, VideoEntity>(&query)
            .bind(ids)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 3. [REPOSITORY] - ▶ 👤 用户的
    /// `desc`: `根据用户ID查找记录`
    pub async fn find_new_list_by_user_id(
        user_id: i64, // 用户 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> Result<Vec<VideoEntity>, sqlx::Error> {
        let pool = pg_pool();

        // 使用参数化查询，避免 SQL 注入
        let query = format!(
            "SELECT {} FROM cola_video.cola_video WHERE uid = $1 AND status = 1 OFFSET $2 LIMIT $3",
            VIDEO_COLUMNS
        );

        sqlx::query_as::<_, VideoEntity>(&query)
            .bind(user_id) // 用户 ID
            .bind(offset)
            .bind(limit)
            .fetch_all(&pool) // 使用 fetch_all 获取多条记录
            .await
    }

    ////////

    /// # 4. [REPOSITORY] - 🎥 👤 根据用户IDs查找对象
    /// * `desc`: `关注的人/朋友/某个用户 复用`
    /// * `condition`: `⚠️ 仅限目标用户ID数量小于1000的, 否则有性能问题`
    pub async fn find_list_by_uids(
        uids: Option<Vec<i64>>,
        keyword: Option<String>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoEntity>, sqlx::Error> {
        let pool = pg_pool();

        // 1. 构建基础 SQL 和参数列表
        let mut sql = format!(
            "SELECT {} FROM cola_video.cola_video WHERE status = 1",
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
