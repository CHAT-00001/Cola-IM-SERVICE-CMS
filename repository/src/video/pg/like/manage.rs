// repository/src/cola_video/pg/like/manage.rs
// 仓储 - ▶ 视频 - pg - 点赞 - 管理员 仓储
// 2026/8/2 14:49 Created.

////////

use crate::pg_pool;
use cola_data::cola_video::entity::like::like::VideoLikeEntity;
use sqlx::{self, Postgres, QueryBuilder};

////////

/// [MANAGE REPOSITORY] - 管理员 repository
/// * `desc`: `▶ 视频 - 管理员列表仓储`
pub struct VideoLikeManageRepo;

impl VideoLikeManageRepo {
    //

    ////////

    /// # 9. [REPOSITORY] - 管理员列表
    pub async fn find_admin_list(
        user_id: Option<i64>,    // 用户 ID
        video_id: Option<i64>,   // 视频 ID
        start_time: Option<i64>, // 开始时间
        end_time: Option<i64>,   // 结束时间
        status_code: i16,        // 状态码
        limit: i64,              // 数量
        offset: i64,             // 页码
    ) -> Result<(Vec<VideoLikeEntity>, u64), sqlx::Error> {
        let pool = pg_pool();

        // 1. 查询总条数 (Count)
        let mut count_builder: QueryBuilder<Postgres> =
            QueryBuilder::new("SELECT COUNT(*) FROM cola_video.like WHERE status = ");
        count_builder.push_bind(status_code);
        count_builder.push(" AND (is_deleted = false OR is_deleted IS NULL)");

        if let Some(uid) = user_id {
            count_builder.push(" AND uid = ");
            count_builder.push_bind(uid);
        }
        if let Some(vid) = video_id {
            count_builder.push(" AND video_id = ");
            count_builder.push_bind(vid);
        }
        if let Some(start) = start_time {
            count_builder.push(" AND addtime >= ");
            count_builder.push_bind(start);
        }
        if let Some(end) = end_time {
            count_builder.push(" AND addtime <= ");
            count_builder.push_bind(end);
        }

        let total: i64 = count_builder
            .build_query_scalar()
            .fetch_one(&pool)
            .await
            .map_err(|e| {
                tracing::error!(
                    error = ?e,
                    "VideoLikeManageRepo::find_admin_list count query failed"
                );
                e
            })?;

        // 2. 查询分页列表数据 (List)
        let mut query_builder: QueryBuilder<Postgres> =
            QueryBuilder::new("SELECT * FROM cola_video.like WHERE status = ");
        query_builder.push_bind(status_code);
        query_builder.push(" AND (is_deleted = false OR is_deleted IS NULL)");

        if let Some(uid) = user_id {
            query_builder.push(" AND uid = ");
            query_builder.push_bind(uid);
        }
        if let Some(vid) = video_id {
            query_builder.push(" AND video_id = ");
            query_builder.push_bind(vid);
        }
        if let Some(start) = start_time {
            query_builder.push(" AND addtime >= ");
            query_builder.push_bind(start);
        }
        if let Some(end) = end_time {
            query_builder.push(" AND addtime <= ");
            query_builder.push_bind(end);
        }

        // 排序与分页
        query_builder.push(" ORDER BY addtime DESC");
        query_builder.push(" LIMIT ");
        query_builder.push_bind(limit);
        query_builder.push(" OFFSET ");
        query_builder.push_bind(offset);

        let entities = query_builder
            .build_query_as::<VideoLikeEntity>()
            .fetch_all(&pool)
            .await
            .map_err(|e| {
                tracing::error!(
                    error = ?e,
                    "VideoLikeManageRepo::find_admin_list query failed"
                );
                e
            })?;

        Ok((entities, total as u64))
    }
}

//////// END
