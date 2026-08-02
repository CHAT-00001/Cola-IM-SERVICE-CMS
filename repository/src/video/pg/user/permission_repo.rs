// repository/src/video/pg/user/permission_repo.rs
// 仓储 - new - repository - 权限数据库操作
// 2026/8/1 14:29 创建

////////

use sqlx::Error;
use crate::pg_pool;


////////


/// # [REPO] - 视频权限数据库操作
pub struct VideoPermissionRepo;


impl VideoPermissionRepo {
    //

    ////////

    /// # 1. 获取发布权限
    pub async fn get_publish_perm(
        uid: i64,
    ) -> Result<bool, Error> {

        let pool = pg_pool();

        let sql = r#"
            SELECT publish_perm
            FROM cola_video.video_user
            WHERE uid = $1
            LIMIT 1
        "#;


        let (perm,): (bool,) =
            sqlx::query_as(sql)
                .bind(uid)
                .fetch_one(&pool)
                .await?;


        Ok(perm)
    }


    ////////

    /// # 2. 获取编辑权限
    pub async fn get_edit_perm(
        uid: i64,
    ) -> Result<bool, Error> {

        let pool = pg_pool();


        let sql = r#"
            SELECT edit_perm
            FROM cola_video.video_user
            WHERE uid = $1
            LIMIT 1
        "#;


        let (perm,): (bool,) =
            sqlx::query_as(sql)
                .bind(uid)
                .fetch_one(&pool)
                .await?;


        Ok(perm)
    }


    ////////

    /// # 3. 获取浏览权限
    pub async fn get_visibility_perm(
        uid: i64,
    ) -> Result<i16, Error> {

        let pool = pg_pool();


        let sql = r#"
            SELECT visibility_perm
            FROM cola_video.video_user
            WHERE uid = $1
            LIMIT 1
        "#;


        let (perm,): (i16,) =
            sqlx::query_as(sql)
                .bind(uid)
                .fetch_one(&pool)
                .await?;


        Ok(perm)
    }


    ////////

    /// # 4. 获取评论权限
    pub async fn get_comment_perm(
        uid: i64,
    ) -> Result<i16, Error> {

        let pool = pg_pool();


        let sql = r#"
            SELECT comment_perm
            FROM cola_video.video_user
            WHERE uid = $1
            LIMIT 1
        "#;


        let (perm,): (i16,) =
            sqlx::query_as(sql)
                .bind(uid)
                .fetch_one(&pool)
                .await?;


        Ok(perm)
    }


    ////////

    /// # 5. 获取下载权限
    pub async fn get_download_perm(
        uid: i64,
        video_id: i64,
    ) -> Result<i16, Error> {

        let pool = pg_pool();


        let sql = r#"
            SELECT download_perm
            FROM cola_video.video_user
            WHERE uid = $1
            AND video_id = $2
            LIMIT 1
        "#;


        let (perm,): (i16,) =
            sqlx::query_as(sql)
                .bind(uid)
                .bind(video_id)
                .fetch_one(&pool)
                .await?;


        Ok(perm)
    }


    ////////

    /// # 6. 获取购买权限
    pub async fn get_buy_perm(
        uid: i64,
        video_id: i64,
    ) -> Result<i16, Error> {

        let pool = pg_pool();


        let sql = r#"
            SELECT buy_perm
            FROM cola_video.video_user
            WHERE uid = $1
            AND video_id = $2
            LIMIT 1
        "#;


        let (perm,): (i16,) =
            sqlx::query_as(sql)
                .bind(uid)
                .bind(video_id)
                .fetch_one(&pool)
                .await?;


        Ok(perm)
    }

}

//////// END