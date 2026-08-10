// /manage.rs
// 
// 2026/8/10 00:06 Created.

////////


// repository/src/cola_video/pg/dislike/manage.rs
// 仓储 - ▶ 视频 - pg - 不喜欢 - 管理员 仓储
// 2026/8/10 00:04 Created.

////////

use crate::pg_pool;
use cola_data::cola_video::entity::like::dislike::VideoDislikeEntity;
use sqlx::{self, Postgres, QueryBuilder};

////////

/// [MANAGE REPOSITORY] - 管理员 repository
/// * `desc`: `▶ 视频 - 管理员列表仓储`
pub struct VideoLikeManageRepo;

impl VideoLikeManageRepo {
    //

    ////////

    /// # 9. [REPOSITORY] - 管理员列表
    pub async fn find_admin_dislike_record_list(
        user_id: Option<i64>,    // 用户 ID
        video_id: Option<i64>,   // 视频 ID
        start_time: Option<i64>, // 开始时间
        end_time: Option<i64>,   // 结束时间
        status_code: i16,        // 状态码
        limit: i64,              // 数量
        offset: i64,             // 页码
    ) -> Result<Vec<VideoDislikeEntity>, sqlx::Error> {
        let pool = pg_pool();

        // 初始化 SQL 查询构建器，带上基础固定条件
        let mut query_builder: QueryBuilder<Postgres> =
            QueryBuilder::new("SELECT * FROM cola_video.dislike WHERE status = ");

        query_builder.push_bind(status_code);
        query_builder.push(" AND (is_deleted = false OR is_deleted IS NULL)");

        // 动态拼接：用户 ID 可选参数
        if let Some(uid) = user_id {
            query_builder.push(" AND uid = ");
            query_builder.push_bind(uid);
        }

        // 动态拼接：视频 ID 可选参数
        if let Some(vid) = video_id {
            query_builder.push(" AND video_id = ");
            query_builder.push_bind(vid);
        }

        // 动态拼接：开始时间可选参数
        if let Some(start) = start_time {
            query_builder.push(" AND add_time >= ");
            query_builder.push_bind(start);
        }

        // 动态拼接：结束时间可选参数
        if let Some(end) = end_time {
            query_builder.push(" AND add_time <= ");
            query_builder.push_bind(end);
        }

        // 排序与分页
        query_builder.push(" ORDER BY add_time DESC");
        query_builder.push(" LIMIT ");
        query_builder.push_bind(limit);
        query_builder.push(" OFFSET ");
        query_builder.push_bind(offset);

        // 构建目标实体查询
        let query = query_builder.build_query_as::<VideoDislikeEntity>();

        // 执行查询并加上错误日志打印
        query.fetch_all(&pool).await.map_err(|e| {
            tracing::error!(
                error = ?e,
                "VideoLikeManageRepo::find_admin_dislike_record_list query failed"
            );
            e
        })
    }
}

//////// END
