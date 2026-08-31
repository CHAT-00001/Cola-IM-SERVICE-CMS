// repository/src/video/pg/video/list.rs -- 仓储 - VIDEO - PG - 视频 - 列表仓储
// 2026/8/2 12:58 Created.

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

////////

/// # [LIST REPOSITORY] -  列表
/// * `desc`: `视频前台列表仓储`
pub struct VideoListRepo;

// 构造实现
impl VideoListRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - ▶ 最新
    /// * `desc`: `获取最新的记录列表`
    pub async fn find_new_list(
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> Result<Vec<VideoEntity>, sqlx::Error> {
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
    /// * `desc`: `获取最热的记录列表`
    pub async fn find_hot_list(
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> Result<Vec<VideoEntity>, sqlx::Error> {
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

    /// # 3. [REPOSITORY] - ▶  随机
    /// * `desc`: `(使用 PostgreSQL 数据库内置随机引擎)`
    pub async fn find_recommend_list(
        limit: i64,  // 数量
        offset: i64, // 页码
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

    /// # 4. [REPOSITORY] - ▶ 附近
    /// * `desc`: `获取附近的记录列表`
    pub async fn find_nearby_list(
        lat: f64,    // 经度
        lng: f64,    // 纬度
        limit: i64,  // 数量
        offset: i64, // 页码
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

    /// # 6. [REPOSITORY] - ▶ 分类
    /// * `desc`: `根据分类 ID 获取最新的记录列表`
    pub async fn find_category_list(
        category_id: i16, // 分类 ID
        limit: i64,       // 数量
        offset: i64,      // 页码
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

    /// # 8. [REPOSITORY] - ▶ 精选
    /// * `desc`: `获取精选的记录列表`
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

    /// # 9. [REPOSITORY] - ▶ 搜索
    /// * `desc`: `关键词 (超级强化版：时间筛选 + 多维可选排序 + 距离计算)`
    pub async fn search_keyword_list(
        keyword: String,               // 关键词
        lat: f64,                      // 纬度
        lng: f64,                      // 经度
        start_time: Option<i64>,       // 开始时间
        end_time: Option<i64>,         // 结束时间
        order_by: Option<SearchOrder>, // 工单
        limit: i64,                    // 数量
        offset: i64,                   // 页码
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
}

//////// END
