// repository/src/new/service//permission_update  -- 仓储 - new - 服务 - 权限
// 2026/6/8 23:33

////////

use anyhow::Result;
use crate::pg_pool; // 👈 引入你项目里通用的数据库连接池获取函数

////////


/// # [SERVICE] - 短视频权限服务
pub struct PermissionsChangeService;


// 构造函数
impl PermissionsChangeService {

    ////////

    /// # 1. [SERVICE] - 更新用户权限
    pub async fn update_user_permission(
        uid: i64,
        delta: i32, // 👈 增加或减少的数量（发片传 1，删片传 -1）
    ) -> Result<(), sqlx::Error> {
        let pool = pg_pool();

        // 对应你 UserEntity 中的 views（发布数/作品数）字段进行原子加减
        let sql = r#"
            UPDATE "user"
            SET views = COALESCE(views, 0) + $2
            WHERE id = $1
        "#;

        sqlx::query(sql)
            .bind(uid)
            .bind(delta)
            .execute(&pool)
            .await?;

        Ok(())
    }

    ////////

    /// # 2. [SERVICE] - 检查用户视频权限
    pub async fn check_video_permission(
        user_id: i64,
    ) -> Result<i16, sqlx::Error> {
        let pool = pg_pool();

        // 1:1 匹配你在 UserEntity 中定义的物理字段 perm_id
        let sql = r#"
            SELECT perm_id
            FROM "video_permission"
            WHERE id = $1
            LIMIT 1
        "#;

        let perm_id: (i16,) = sqlx::query_as(sql)
            .bind(user_id)
            .fetch_one(&pool)
            .await?;

        Ok(perm_id.0)
    }

    ////////

    /// # 3. [SERVICE] - 修改用户视频权限
    /// * 场景：后台管理员对博主实施禁言、关闭发布短视频权限等惩罚
    pub async fn update_user_video_perm(
        uid: i64,
        new_perm_id: i16, // 👈 传入修改后的权限 ID（如 0:禁用，1:常规，2:创作者）
    ) -> Result<(), sqlx::Error> {
        let pool = pg_pool();

        let sql = r#"
            UPDATE "user"
            SET perm_id = $2, updated_at = NOW()
            WHERE id = $1
        "#;

        sqlx::query(sql)
            .bind(uid)
            .bind(new_perm_id)
            .execute(&pool)
            .await?;

        Ok(())
    }


    ////////

    /// # 10. [SERVICE] - 修改视频浏览权限
    pub async fn update_video_visite_perm(
        uid: i64,
        new_perm_id: i16, // 👈 传入修改后的权限 ID（如 0:禁用，1:常规，2:创作者）
    ) -> Result<(), sqlx::Error> {
        let pool = pg_pool();

        let sql = r#"
            UPDATE "user"
            SET perm_id = $2, updated_at = NOW()
            WHERE id = $1
        "#;

        sqlx::query(sql)
            .bind(uid)
            .bind(new_perm_id)
            .execute(&pool)
            .await?;

        Ok(())
    }


    ////////

    /// # 11. [SERVICE] - 修改视频评论权限
    /// * `uid` - 用户ID
    /// * `video_id` - 视频ID
    /// * `comment_perm` - 评论权限值 (范围: 1-5)
    ///
    /// 权限说明:
    /// - 1: 仅作者自己
    /// - 2: 仅朋友
    /// - 3: 仅己方关注
    /// - 4: 仅粉丝
    /// - 5: 所有人
    pub async fn update_video_comment_perm(
        uid: i64,
        video_id: i64,
        comment_perm: i16,
    ) -> Result<(), sqlx::Error> {
        let pool = pg_pool();

        let sql = r#"
        UPDATE "video_permission"
        SET comment_perm = $3, updated_at = NOW()
        WHERE id = $2
            AND user_id = $1
            AND $3 BETWEEN 1 AND 5
    "#;

        let result = sqlx::query(sql)
            .bind(uid)
            .bind(video_id)
            .bind(comment_perm)
            .execute(&pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(())
    }


    ////////

    /// # 12. [SERVICE] - 修改视频弹幕权限
    pub async fn update_video_danmaku_perm(
        uid: i64,
        video_id: i64,
        danmaku_perm: i16,
    ) -> Result<(), sqlx::Error> {
        let pool = pg_pool();

        let sql = r#"
        UPDATE "video_permission"
        SET danmaku_perm = $3, updated_at = NOW()
        WHERE id = $2
            AND user_id = $1
            AND $3 BETWEEN 1 AND 5
    "#;

        let result = sqlx::query(sql)
            .bind(uid)
            .bind(video_id)
            .bind(danmaku_perm)
            .execute(&pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(())
    }

    ////////

    /// # 13. [SERVICE] - 修改视频收藏权限
    pub async fn update_video_collect_perm(
        uid: i64,
        video_id: i64,
        collect_perm: i16,
    ) -> Result<(), sqlx::Error> {
        let pool = pg_pool();

        let sql = r#"
        UPDATE "video_permission"
        SET collect_perm = $3, updated_at = NOW()
        WHERE id = $2
            AND user_id = $1
            AND $3 BETWEEN 1 AND 5
    "#;

        let result = sqlx::query(sql)
            .bind(uid)
            .bind(video_id)
            .bind(collect_perm)
            .execute(&pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(())
    }

    ////////

    /// # 14. [SERVICE] - 修改视频下载权限
    pub async fn update_video_download_perm(
        uid: i64,
        video_id: i64,
        download_perm: i16,
    ) -> Result<(), sqlx::Error> {
        let pool = pg_pool();

        let sql = r#"
        UPDATE "video_permission"
        SET download_perm = $3, updated_at = NOW()
        WHERE id = $2
            AND user_id = $1
            AND $3 BETWEEN 1 AND 5
    "#;

        let result = sqlx::query(sql)
            .bind(uid)
            .bind(video_id)
            .bind(download_perm)
            .execute(&pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(())
    }

    ////////

    /// # 15. [SERVICE] - 修改视频购买权限
    /// * 场景：后台管理员对博主实施禁言、关闭发布短视频权限等惩罚
    pub async fn update_video_buy_perm(
        uid: i64,
        video_id: i64,
        buy_perm: i16,
    ) -> Result<(), sqlx::Error> {
        let pool = pg_pool();

        let sql = r#"
        UPDATE "video_permission"
        SET buy_perm = $3, updated_at = NOW()
        WHERE id = $2
            AND user_id = $1
            AND $3 BETWEEN 1 AND 5
    "#;

        let result = sqlx::query(sql)
            .bind(uid)
            .bind(video_id)
            .bind(buy_perm)
            .execute(&pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(())
    }

    ////////
}

//////// END