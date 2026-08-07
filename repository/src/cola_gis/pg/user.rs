// repository/src/cola_gis/pg/cola_user.rs  -- 仓储 - GIS - pg - 用户
// 2026/7/6 14:01

////////

use crate::pg_pool;
use cola_data::cola_gis::entity::user::VideoUserEntity;
use sqlx;

////////

/// # [USER REPOSITORY] 用户 - 仓储
pub struct UserRepo;

impl UserRepo {
    // 💡

    ////////

    /// # 1. [REPOSITORY] - 保存或者更新权限
    pub async fn save_or_update_user_perm(uid: i64, user_type: i16, status: i16, comment_perm: i16, danmaku_perm: i16, collect_perm: i16, download_perm: i16) -> Result<(), sqlx::Error> {
        let pool = pg_pool();
        let query = r#"
            INSERT INTO cola_gis.gis_user (uid, user_type, status, comment_perm, danmaku_perm, collect_perm, download_perm, addtime, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, EXTRACT(EPOCH FROM NOW())::BIGINT, NOW())
            ON CONFLICT (uid) DO UPDATE SET
                user_type = EXCLUDED.user_type, status = EXCLUDED.status,
                comment_perm = EXCLUDED.comment_perm, danmaku_perm = EXCLUDED.danmaku_perm,
                collect_perm = EXCLUDED.collect_perm, download_perm = EXCLUDED.download_perm,
                updated_at = NOW()"#;
        sqlx::query(query).bind(uid).bind(user_type).bind(status)
            .bind(comment_perm).bind(danmaku_perm).bind(collect_perm).bind(download_perm)
            .execute(&pool).await?;
        Ok(())
    }

    ////////

    /// # 2. [REPOSITORY] - 查找用户权限
    pub async fn find_user_perm(uid: i64) -> Result<Option<VideoUserEntity>, sqlx::Error> {
        let pool = pg_pool();
        sqlx::query_as::<_, VideoUserEntity>("SELECT * FROM cola_gis.gis_user WHERE uid = $1 LIMIT 1")
            .bind(uid).fetch_optional(&pool).await
    }

    ////////

    /// # 3. [REPOSITORY] - 更新用户计数
    pub async fn update_user_count(uid: i64, publish_delta: i32, liked_delta: i32, total_favorited_delta: i32, collected_delta: i32, following_delta: i32, follower_delta: i32) -> Result<(), sqlx::Error> {
        let pool = pg_pool();
        let query = r#"
            UPDATE cola_gis.gis_user SET
                publish_count = GREATEST(0, publish_count + $2),
                liked_count = GREATEST(0, liked_count + $3),
                total_favorited_count = GREATEST(0, total_favorited_count + $4),
                collected_count = GREATEST(0, collected_count + $5),
                following_count = GREATEST(0, following_count + $6),
                follower_count = GREATEST(0, follower_count + $7),
                updated_at = NOW()
            WHERE uid = $1"#;
        sqlx::query(query).bind(uid).bind(publish_delta).bind(liked_delta)
            .bind(total_favorited_delta).bind(collected_delta).bind(following_delta).bind(follower_delta)
            .execute(&pool).await?;
        Ok(())
    }
}

//////// END


