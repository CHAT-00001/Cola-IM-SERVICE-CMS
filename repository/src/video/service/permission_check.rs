// repository/src/video/service//permission_update  -- 仓储 - video - 服务 - 权限检查服务
// 2026/6/8 23:33

////////

use anyhow::Result;
use crate::pg_pool; // 👈 引入你项目里通用的数据库连接池获取函数

////////


/// # [SERVICE] - 短视频权限服务
pub struct VideoPermissionsCheckService;


// 构造函数
impl VideoPermissionsCheckService {

    ////////

    /// # 1. [SERVICE] - 检查视频发布权限
    pub async fn check_video_publish_perm(
        uid: i64,
    ) -> Result<(), sqlx::Error> {
        let pool = pg_pool();

        // 
        let sql = r#"
            SELECT "publish_perm"
            FROM "video_user
            SET publish_perm = COALESCE(views, 0) + $2
            WHERE id = $1
        "#;

        sqlx::query(sql)
            .bind(uid)
            .execute(&pool)
            .await?;

        Ok(())
    }

    ////////

    /// # 2. [SERVICE] - 检查视频编辑权限
    pub async fn check_video_edit_perm(
        uid: i64,
        delta: i32, // 👈 增加或减少的数量（发片传 1，删片传 -1）
    ) -> Result<(), sqlx::Error> {
        let pool = pg_pool();

        //
        let sql = r#"
            SELECT "publish_perm"
            FROM "video_user
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

    /// # 1. [SERVICE] - 检查视频浏览权限
    pub async fn check_video_visibility_perm(
        uid: i64,
        delta: i32, // 👈 增加或减少的数量（发片传 1，删片传 -1）
    ) -> Result<(), sqlx::Error> {
        let pool = pg_pool();

        // 对应你 UserEntity 中的 views（发布数/作品数）字段进行原子加减
        let sql = r#"
            SELECT "visibility_perm"
            FROM "video
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

    /// # 2. [SERVICE] - 检查视频评论权限
    pub async fn check_video_comment_perm(
        user_id: i64,
    ) -> Result<i16, sqlx::Error> {
        let pool = pg_pool();

        // 1:1 匹配你在 UserEntity 中定义的物理字段 perm_id
        let sql = r#"
            SELECT perm_id
            FROM "video"
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

    /// # 3. [SERVICE] - 检查视频弹幕权限
    pub async fn check_video_danmaku_perm(
        uid: i64,
        new_perm_id: i16, // 👈 传入修改后的权限 ID（如 0:禁用，1:常规，2:创作者）
    ) -> Result<(), sqlx::Error> {
        let pool = pg_pool();

        let sql = r#"
            SELECT "danmaku_perm"
            FROM "video"
            SET perm_id = $2, updated_at = NOW()
            WHERE danmaku_perm = $1
        "#;

        sqlx::query(sql)
            .bind(uid)
            .bind(new_perm_id)
            .execute(&pool)
            .await?;

        Ok(())
    }

    ////////

    /// # 4. [SERVICE] - 检查视频收藏权限
    pub async fn check_video_collect_perm(
        uid: i64,
        new_perm_id: i16, // 👈 传入修改后的权限 ID（如 0:禁用，1:常规，2:创作者）
    ) -> Result<(), sqlx::Error> {
        let pool = pg_pool();

        let sql = r#"
            UPDATE "video"
            SET perm_id = $2, updated_at = NOW()
            WHERE collect_perm = $1
        "#;

        sqlx::query(sql)
            .bind(uid)
            .bind(new_perm_id)
            .execute(&pool)
            .await?;

        Ok(())
    }


    ////////

    /// # 5. [SERVICE] - 检查视频下载权限
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
    pub async fn check_video_download_perm(
        uid: i64,
        video_id: i64,
        comment_perm: i16,
    ) -> Result<(), sqlx::Error> {
        let pool = pg_pool();

        let sql = r#"
        SELECT "download_perm"
        FROM "video"
        SET comment_perm = $3, updated_at = NOW()
        WHERE id = $2
            AND user_id = $1
            AND download_perm = $1
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

    /// # 6. [SERVICE] - 检查视频购买权限
    pub async fn check_video_buy_perm(
        uid: i64,
        video_id: i64,
        danmaku_perm: i16,
    ) -> Result<(), sqlx::Error> {
        let pool = pg_pool();

        let sql = r#"
        SELECT "buy_perm"
        FORM "video"
        SET danmaku_perm = $3, updated_at = NOW()
        WHERE id = $2
            AND video_id = $1
            AND buy_perm = $1
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
}

//////// END