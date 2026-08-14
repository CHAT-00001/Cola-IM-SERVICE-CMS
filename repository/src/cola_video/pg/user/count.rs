// repository/src/cola_video/pg/user/count
// 仓储 - new - pg - 用户 - count_repo 计数仓储
// 2026/8/2 11:14 Created.

////////

use crate::pg_pool;
use cola_data::cola_video::entity::user::user::VideoUserEntity;

////////

pub struct UserCountRepo;

impl UserCountRepo {
    // 💡

    ////////

    /// # [REPOSITORY] - 保存或更新用户的短视频权限及基础状态
    /// * 场景：开通创作者权限、后台禁言、调整评论/弹幕门槛
    /// * 机制：使用 PG 的 UPSERT (ON CONFLICT) 语法，自动完成“不存在则创建，存在则更新权限”
    pub async fn save_or_update_user_perm(
        uid: i64,
        user_type: i16,
        status: i16,
        comment_perm: i16,
        danmaku_perm: i16,
        collect_perm: i16,
        download_perm: i16,
    ) -> Result<(), sqlx::Error> {
        let pool = pg_pool();

        let query = r#"
            INSERT INTO cola_video.video_user (
                uid, user_type, status, comment_perm, danmaku_perm, collect_perm, download_perm, addtime, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, EXTRACT(EPOCH FROM NOW())::BIGINT, NOW())
            ON CONFLICT (uid)
            DO UPDATE SET
                user_type = EXCLUDED.user_type,
                status = EXCLUDED.status,
                comment_perm = EXCLUDED.comment_perm,
                danmaku_perm = EXCLUDED.danmaku_perm,
                collect_perm = EXCLUDED.collect_perm,
                download_perm = EXCLUDED.download_perm,
                updated_at = NOW()
        "#;

        sqlx::query(query)
            .bind(uid)
            .bind(user_type)
            .bind(status)
            .bind(comment_perm)
            .bind(danmaku_perm)
            .bind(collect_perm)
            .bind(download_perm)
            .execute(&pool)
            .await?;

        Ok(())
    }

    ////////

    /// # [REPOSITORY] - 查找用户的短视频权限及完整实体
    /// * 场景：网关后、应用层逻辑开始前，用于卡发布/评论/弹幕权限以及查看账号是否被封禁
    pub async fn find_user_perm(uid: i64) -> Result<Option<VideoUserEntity>, sqlx::Error> {
        let pool = pg_pool();

        let query = "SELECT * FROM cola_video.video_user WHERE uid = $1 LIMIT 1";

        sqlx::query_as::<_, VideoUserEntity>(query)
            .bind(uid)
            .fetch_optional(&pool)
            .await
    }

    ////////

    /// # [REPOSITORY] - 更新用户的短视频计数（原子增量更新）
    /// * 场景：发布视频、被点赞、涨粉、关注他人时，由 Service 层联动调用
    /// * 警告：严禁传入绝对值覆盖！必须传入变化量（增量），例如加 1 传 `1`，扣减传 `-1`
    pub async fn update_user_count(
        uid: i64,
        publish_delta: i32,         // 发布视频变化量
        liked_delta: i32,           // 该用户点赞别人变化量
        total_favorited_delta: i32, // 该用户产出视频被赞变化量（获赞数）
        collected_delta: i32,       // 收藏变化量
        following_delta: i32,       // 关注人数变化量
        follower_delta: i32,        // 粉丝人数变化量
    ) -> Result<(), sqlx::Error> {
        let pool = pg_pool();

        let query = r#"
            UPDATE cola_video.video_user
            SET
                publish_count = publish_count + $2,
                liked_count = liked_count + $3,
                total_favorited_count = total_favorited_count + $4,
                collected_count = collected_count + $5,
                following_count = following_count + $6,
                follower_count = follower_count + $7,
                updated_at = NOW()
            WHERE uid = $1
        "#;

        sqlx::query(query)
            .bind(uid)
            .bind(publish_delta)
            .bind(liked_delta)
            .bind(total_favorited_delta)
            .bind(collected_delta)
            .bind(following_delta)
            .bind(follower_delta)
            .execute(&pool)
            .await?;

        Ok(())
    }

    ////////

    /// # [REPOSITORY] - 查找用户的短视频计数
    /// * 场景：渲染创作者个人中心主页、或者是前台查看他人主页时的数据展示
    pub async fn find_user_count(uid: i64) -> Result<Option<VideoUserEntity>, sqlx::Error> {
        let pool = pg_pool();

        // 既然最终都是映射成完整的 VideoUserEntity，直接执行精准查询即可
        let query = "SELECT * FROM cola_video.video_user WHERE uid = $1 LIMIT 1";

        sqlx::query_as::<_, VideoUserEntity>(query)
            .bind(uid)
            .fetch_optional(&pool)
            .await
    }

    /// # 7002. [REPOSITORY] - 查找用户浏览过的视频IDs
    /// * `user_id`: 用户 ID
    /// * `offset`: 分页偏移量
    /// * `limit`: 每页数量
    pub async fn find_user_visited_video_ids_paginated(
        user_id: i64,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<i64>, sqlx::Error> {
        let pool = pg_pool();

        let query = r#"
        SELECT video_id
        FROM user_visit_history
        WHERE user_id = $1
        ORDER BY visited_at DESC, id DESC
        LIMIT $2 OFFSET $3
    "#;

        let video_ids: Vec<i64> = sqlx::query_scalar(query)
            .bind(user_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await?;

        Ok(video_ids)
    }

    ////////

    /// # 7002. [REPOSITORY] - 统计用户浏览过的视频总数
    /// * `user_id` 用户 ID
    pub async fn count_user_visited_videos(user_id: i64) -> Result<i64, sqlx::Error> {
        let pool = pg_pool();

        let query = r#"
        SELECT COUNT(*)
        FROM user_visit_history
        WHERE user_id = $1
    "#;

        let count: i64 = sqlx::query_scalar(query)
            .bind(user_id)
            .fetch_one(&pool)
            .await?;

        Ok(count)
    }

    /// # 7002. [REPOSITORY] - 查找用户浏览过的视频IDs
    /// * `user_id`: 用户 ID
    /// * `offset`: 分页偏移量
    /// * `limit`: 每页数量
    pub async fn find_user_liked_list(
        user_id: i64,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<i64>, sqlx::Error> {
        let pool = pg_pool();

        let query = r#"
        SELECT video_id
        FROM user_visit_history
        WHERE user_id = $1
        ORDER BY visited_at DESC, id DESC
        LIMIT $2 OFFSET $3
    "#;

        let video_ids: Vec<i64> = sqlx::query_scalar(query)
            .bind(user_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await?;

        Ok(video_ids)
    }

    ////////
}

//////// END
