// repository/src/video/pg/file/file.rs  --
// 仓储 - VIDEO - pg - file - 评论
// 2026/6/8 16:55

////////

use crate::pg_pool;
use cola_data::cola_video::command::comment::CommentCommand;
use cola_data::cola_video::entity::comment::VideoCommentEntity;
use sqlx::{self, Postgres, QueryBuilder};

////////

/// # [CHECK REPOSITORY] - 评论 检查
/// * `DESC`: `VIDEO` - `检查评论状态`
pub struct VideoCommentCheckRepo;

impl VideoCommentCheckRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - 健康
    /// * `desc`: 检查评论的健康分（根据评论 ID）
    pub async fn find_health(id: i64, // 评论 ID
    ) -> Result<Vec<i16>, sqlx::Error> {
        let pool = pg_pool();

        let query = "
            SELECT health
            FROM cola_video.comments
            WHERE id = $1
        ";

        sqlx::query_scalar::<_, i16>(query)
            .bind(id)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 2. [REPOSITORY] - 状态
    /// * `desc`: 检查评论的状态码（根据评论 ID）
    pub async fn find_status(id: i64, // 评论 ID
    ) -> Result<Vec<i16>, sqlx::Error> {
        let pool = pg_pool();

        let query = "
            SELECT status
            FROM cola_video.comments
            WHERE id = $1
        ";

        sqlx::query_scalar::<_, i16>(query)
            .bind(id)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 3. [REPOSITORY] - 归属
    /// * `desc`: 检查用户是否持有评论所有权
    pub async fn find_owner(
        id: i64,  // 评论 ID
        uid: i64, // 用户 ID
    ) -> Result<Vec<bool>, sqlx::Error> {
        let pool = pg_pool();

        let query = "
            SELECT (COUNT(1) > 0)
            FROM cola_video.comments
            WHERE id = $1 AND uid = $2
        ";

        sqlx::query_scalar::<_, bool>(query)
            .bind(id)
            .bind(uid)
            .fetch_all(&pool)
            .await
    }

    /// # 4. [REPOSITORY] - 是否存在
    /// `desc`: 检查评论是否存在且状态正常
    pub async fn exists_active(id: i64) -> Result<bool, sqlx::Error> {
        let pool = pg_pool();
        let query = "
            SELECT EXISTS(
                SELECT 1 FROM cola_video.comments
                WHERE id = $1 AND status = 1
            )
        ";
        sqlx::query_scalar::<_, bool>(query)
            .bind(id)
            .fetch_one(&pool)
            .await
    }
}

//////// END
