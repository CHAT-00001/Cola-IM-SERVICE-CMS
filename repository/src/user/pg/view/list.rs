// repository/src/pg/user/pg/view/list.rs
// 仓储 - 可乐用户 - pg - 浏览 - 列表
// 2026/6/29 03:54 Created.

////////

use crate::pg_pool;
use cola_data::cola_user::entity::view::{USER_VIEW_COLUMNS, UserViewEntity};
use sqlx;

////////

/// # [LIST REPOSITORY] - 列表
/// * `desc`: `用户浏览列表仓储`
pub struct UserViewListRepo;

// 构造函数
impl UserViewListRepo {
    ////////

    /// # 1. [REPOSITORY] - 用户的
    /// * `desc`: `根据用户ID获取浏览记录列表`
    pub async fn find_new_list_by_user_id(
        user_id: i64, // 目标用户ID
        limit: i64,   // 数量
        offset: i64,  // 偏移量
    ) -> Result<Vec<UserViewEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM \"user\".\"view\" WHERE uid = $1 ORDER BY id DESC LIMIT $2 OFFSET $3",
            USER_VIEW_COLUMNS
        );

        sqlx::query_as::<_, UserViewEntity>(&query)
            .bind(user_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 2. [REPOSITORY] - 视频的
    /// * `desc`: `根据视频ID获取浏览记录列表`
    pub async fn find_new_list_by_video_id(
        video_id: i64, // 目标视频ID
        limit: i64,    // 数量
        offset: i64,   // 偏移量
    ) -> Result<Vec<UserViewEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM \"user\".\"view\" WHERE video_id = $1 ORDER BY id DESC LIMIT $2 OFFSET $3",
            USER_VIEW_COLUMNS
        );

        sqlx::query_as::<_, UserViewEntity>(&query)
            .bind(video_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }
}

//////// END