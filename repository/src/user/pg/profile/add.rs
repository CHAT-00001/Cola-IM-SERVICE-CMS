// repository/src/user/pg/profile/add.rs
// 仓储 - USER - pg - profile - 资料名片 添加
// 2026/8/3 12:56 Created.
// 2026/8/6 实现：资料名片 CRUD 仓储

////////

use crate::pg_pool;
use cola_data::user::command::profile::ProfileCommand;
use cola_data::user::entity::profile::UserProfileEntity;
use sqlx;

////////

/// # [PROFILE REPO] - 资料名片仓储
pub struct ProfileAddRepo;

impl ProfileAddRepo {

    ////////

    /// # 1. [REPOSITORY] - 保存/更新资料名片
    /// * `desc`: UPSERT 模式，存在则更新，不存在则创建
    pub async fn pg_upsert_profile(
        cmd: &ProfileCommand,
    ) -> Result<i64, sqlx::Error> {
        let pool = pg_pool();
        let now = chrono::Utc::now().timestamp();

        let id: i64 = sqlx::query_scalar(
            r#"
            INSERT INTO cola_user.profile (
                user_id, user_nickname, avatar, bg_img, signature,
                birthday, sex, email, phone, sns_url,
                status, create_time, created_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, 1, $11, NOW())
            ON CONFLICT (user_id)
            DO UPDATE SET
                user_nickname = EXCLUDED.user_nickname,
                avatar = EXCLUDED.avatar,
                bg_img = EXCLUDED.bg_img,
                signature = EXCLUDED.signature,
                birthday = EXCLUDED.birthday,
                sex = EXCLUDED.sex,
                email = EXCLUDED.email,
                phone = EXCLUDED.phone,
                sns_url = EXCLUDED.sns_url,
                updated_at = NOW()
            RETURNING id
            "#,
        )
        .bind(cmd.user_id)
        .bind(&cmd.nickname)
        .bind(&cmd.avatar)
        .bind(&cmd.bg_img)
        .bind(&cmd.signature)
        .bind(cmd.birthday)
        .bind(cmd.sex)
        .bind(&cmd.email)
        .bind(&cmd.phone)
        .bind(&cmd.sns_url)
        .bind(now)
        .fetch_one(&pool)
        .await?;

        Ok(id)
    }

    ////////

    /// # 2. [REPOSITORY] - 按用户ID查询资料名片
    /// * `desc`: 单条查询，用于个人主页展示
    pub async fn pg_find_by_user_id(
        user_id: i64,
    ) -> Result<Option<UserProfileEntity>, sqlx::Error> {
        use cola_data::user::entity::profile::USER_PROFILE_COLUMNS;
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_user.profile WHERE user_id = $1 AND status = 1 LIMIT 1",
            USER_PROFILE_COLUMNS
        );
        sqlx::query_as::<_, UserProfileEntity>(&query)
            .bind(user_id)
            .fetch_optional(&pool)
            .await
    }
}

//////// END