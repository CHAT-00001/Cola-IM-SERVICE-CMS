// repository/src/cola_video/pg/file/list.rs
// 🛢 仓储 - ▶ 可乐视频 - pg - 评论记录 - 前台列表
// 2026/8/8 00:46 Created.

////////

use crate::pg_pool;
use cola_data::cola_video::entity::comment::{VIDEO_COMMENT_COLUMNS, VideoCommentEntity};
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
pub struct VideoCommentListRepo;

// 构造实现
impl VideoCommentListRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - ▶ 最新
    /// * `desc`: `获取最新的评论记录列表`
    pub async fn find_new_list(
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> Result<Vec<VideoCommentEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_video.file WHERE status = 1 ORDER BY addtime DESC LIMIT $1 OFFSET $2",
            VIDEO_COMMENT_COLUMNS
        );

        sqlx::query_as::<_, VideoCommentEntity>(&query)
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
    ) -> Result<Vec<VideoCommentEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_video.file WHERE status = 1 ORDER BY likes DESC, views DESC, addtime DESC LIMIT $1 OFFSET $2",
            VIDEO_COMMENT_COLUMNS
        );

        sqlx::query_as::<_, VideoCommentEntity>(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 3. [REPOSITORY] - ▶ 附近
    /// * `desc`: `获取附近的记录列表`
    pub async fn find_nearby_list(
        lat: f64,    // 经度
        lng: f64,    // 纬度
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> Result<Vec<VideoCommentEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {}, SQRT(POW(lat - $1, 2) + POW(lng - $2, 2)) AS distance
             FROM cola_video.comments
             WHERE status = 1
             ORDER BY distance ASC
             LIMIT $3 OFFSET $4",
            VIDEO_COMMENT_COLUMNS
        );

        sqlx::query_as::<_, VideoCommentEntity>(&query)
            .bind(lat)
            .bind(lng)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 4. [REPOSITORY] - ▶ 置顶
    /// * `desc`: `获取置顶的记录列表`
    pub async fn find_pinned_list(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoCommentEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_video.comments WHERE status = 1 ORDER BY likes DESC, addtime DESC LIMIT $1 OFFSET $2",
            VIDEO_COMMENT_COLUMNS
        );

        sqlx::query_as::<_, VideoCommentEntity>(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 5. [REPOSITORY] - ▶ 目标
    /// * `desc`: `获取目标评论并高亮,回复评论时使用`
    pub async fn find_focus_list(
        id: i64, // 目标评论 ID
    ) -> Result<Vec<VideoCommentEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_video.comments WHERE status = 1 AND id = $1 ORDER BY likes DESC",
            VIDEO_COMMENT_COLUMNS
        );

        sqlx::query_as::<_, VideoCommentEntity>(&query)
            .bind(id) // 评论 ID
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 6. [REPOSITORY] - 按照 用户ID 命中评论列表
    /// * `desc`: `根据 user_id 命中表中的 uid 获取评论记录列表`
    pub async fn find_list_by_user_id(
        user_id: i64, // 用户 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> Result<Vec<VideoCommentEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM \"cola_video\".\"file\" WHERE uid = $1 AND status = 1 ORDER BY add_time DESC LIMIT $2 OFFSET $3",
            VIDEO_COMMENT_COLUMNS
        );

        sqlx::query_as::<_, VideoCommentEntity>(&query)
            .bind(user_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
            .map_err(|e| {
                eprintln!(
                    "[DB ERROR] find_list_by_user_id | SQL: {} | user_id: {} | limit: {} | offset: {} | err: {:?}",
                    query, user_id, limit, offset, e
                );
                e
            })
    }

    ////////

    /// # 7. [REPOSITORY] - 按照 视频ID 命中评论列表
    /// * `desc`: `根据 video_id 命中表中的 video_id 获取评论记录列表`
    pub async fn find_list_by_video_id(
        video_id: i64, // 视频 ID
        limit: i64,    // 数量
        offset: i64,   // 页码
    ) -> Result<Vec<VideoCommentEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM \"cola_video\".\"file\" WHERE video_id = $1 AND status = 1 ORDER BY add_time DESC LIMIT $2 OFFSET $3",
            VIDEO_COMMENT_COLUMNS
        );

        sqlx::query_as::<_, VideoCommentEntity>(&query)
            .bind(video_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
            .map_err(|e| {
                eprintln!(
                    "[DB ERROR] find_list_by_video_id | SQL: {} | video_id: {} | limit: {} | offset: {} | err: {:?}",
                    query, video_id, limit, offset, e
                );
                e
            })
    }
}

//////// END
