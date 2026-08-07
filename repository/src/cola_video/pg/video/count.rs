// repository/src/cola_video/pg/video/count.rs
// 仓储 - ▶ 可乐视频 - pg - 视频 - 计数
// 2026/6/10 05:41

////////

use crate::pg_pool;
use sqlx::{self};

////////

/// # [REPOSITORY] - 计数仓储
/// * `desc`: `▶ 可乐视频 - 🛢 视频计数仓储`
pub struct VideoCountRepo;

// 构造实现
impl VideoCountRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - 更新浏览量
    // `描述` 浏览量是递增,不可以-1, 调用一次就+1
    pub async fn pg_update_video_views(video_id: i64) -> Result<(), sqlx::Error> {
        let pool = pg_pool();
        let query = r#"
            INSERT INTO cola_video.video_count (video_id, views)
            VALUES ($1, 1)
            ON CONFLICT (video_id)
            DO UPDATE SET views = cola_video.video_count.views + 1
        "#;
        sqlx::query(query).bind(video_id).execute(&pool).await?;
        Ok(())
    }

    ////////

    /// # 2. [REPOSITORY] - 更新评论数量
    /// * `desc`: 传入正数加计数，传入负数减计数（如 +1 / -1），利用 GREATEST 锁死下限为 0
    pub async fn pg_update_video_comments(
        video_id: i64,  // 视频 ID
        increment: i16, // 变化量：+1 代表新增，-1 代表删除
    ) -> Result<(), sqlx::Error> {
        let pool = pg_pool();

        // 🌟 核心设计说明：
        // 1. INSERT 时：如果是新视频初始化计数，直接用 GREATEST(0, 增量) 防止第一笔就是负数。
        // 2. UPDATE 时：旧计数 + 增量，再用 GREATEST(0, ...) 锁死。如果 0 + (-1)，结果依然是 0。
        let query = r#"
        INSERT INTO cola_video.video_count (video_id, comments)
        VALUES ($1, GREATEST(0, $2::INT))
        ON CONFLICT (video_id)
        DO UPDATE SET comments = GREATEST(0, cola_video.video_count.comments + $2::INT)
    "#;

        sqlx::query(query)
            .bind(video_id)
            .bind(increment as i32) // 💡 显式转成 i32 绑定，避免 PostgreSQL 对 i16(smallint) 的类型推导产生歧义
            .execute(&pool)
            .await?;

        Ok(())
    }

    ////////

    /// # 3. [REPOSITORY] - 更新视频点赞数量
    pub async fn pg_update_video_likes(
        video_id: i64,
        increment: i16, // 🔒 经过上层拦截后，这里进来的数字绝对安全
    ) -> Result<(), sqlx::Error> {
        let pool = pg_pool();

        let query = r#"
        INSERT INTO cola_video.video_count (video_id, likes)
        VALUES ($1, GREATEST(0, $2::INT))
        ON CONFLICT (video_id)
        DO UPDATE SET likes = GREATEST(0, cola_video.video_count.likes + $2::INT)
    "#;

        sqlx::query(query)
            .bind(video_id)
            .bind(increment as i32)
            .execute(&pool)
            .await?;

        Ok(())
    }

    ////////

    /// # 3. [REPOSITORY] - 更新视频不喜欢数量
    pub async fn pg_update_video_unlikes(
        video_id: i64,
        increment: i16, // 🔒 经过上层拦截后，这里进来的数字绝对安全
    ) -> Result<(), sqlx::Error> {
        let pool = pg_pool();

        let query = r#"
        INSERT INTO cola_video.video_count (video_id, unlikes)
        VALUES ($1, GREATEST(0, $2::INT))
        ON CONFLICT (video_id)
        DO UPDATE SET likes = GREATEST(0, cola_video.video_count.unlikes + $2::INT)
    "#;

        sqlx::query(query)
            .bind(video_id)
            .bind(increment as i32)
            .execute(&pool)
            .await?;

        Ok(())
    }

    ////////

    /// # 4. [REPOSITORY] - 更新收藏数量
    /// * `desc`: 传入正数加收藏，传入负数减收藏（如 +1 / -1），利用 GREATEST 锁死下限为 0
    pub async fn pg_update_video_collects(
        video_id: i64,  // 视频 ID
        increment: i16, // 变化量：+1 代表收藏，-1 代表取消收藏
    ) -> Result<(), sqlx::Error> {
        let pool = pg_pool();

        let query = r#"
        INSERT INTO cola_video.video_count (video_id, collects)
        VALUES ($1, GREATEST(0, $2::INT))
        ON CONFLICT (video_id)
        DO UPDATE SET collects = GREATEST(0, cola_video.video_count.collects + $2::INT)
    "#;

        sqlx::query(query)
            .bind(video_id)
            .bind(increment as i32)
            .execute(&pool)
            .await?;

        Ok(())
    }

    ////////

    /// # 5. [REPOSITORY] - 更新推荐数量
    /// * `desc`: 传入正数加推荐，传入负数减推荐（如 +1 / -1），利用 GREATEST 锁死下限为 0
    pub async fn pg_update_video_recommend(
        video_id: i64,  // 视频 ID
        increment: i16, // 变化量：+1 代表增加推荐，-1 代表减少推荐
    ) -> Result<(), sqlx::Error> {
        let pool = pg_pool();

        let query = r#"
        INSERT INTO cola_video.video_count (video_id, recommend)
        VALUES ($1, GREATEST(0, $2::INT))
        ON CONFLICT (video_id)
        DO UPDATE SET recommend = GREATEST(0, cola_video.video_count.recommend + $2::INT)
    "#;

        sqlx::query(query)
            .bind(video_id)
            .bind(increment as i32)
            .execute(&pool)
            .await?;

        Ok(())
    }

    ////////

    /// # 6. [REPOSITORY] - 更新分享数量
    /// * `描述` 数量不需要+1,只能递增
    pub async fn pg_update_video_shares(video_id: i64) -> Result<(), sqlx::Error> {
        let pool = pg_pool();
        let query = r#"
            INSERT INTO cola_video.video_count (video_id, shares)
            VALUES ($1, 1)
            ON CONFLICT (video_id)
            DO UPDATE SET shares = cola_video.video_count.shares + 1
        "#;
        sqlx::query(query).bind(video_id).execute(&pool).await?;
        Ok(())
    }

    ////////

    /// # 7. [REPOSITORY] - 更新销售数量
    /// * `描述` 数量不需要+1,只能递增
    pub async fn pg_update_video_buys(video_id: i64) -> Result<(), sqlx::Error> {
        let pool = pg_pool();
        let query = r#"
            INSERT INTO cola_video.video_count (video_id, shalls)
            VALUES ($1, 1)
            ON CONFLICT (video_id)
            DO UPDATE SET shares = cola_video.video_count.shares + 1
        "#;
        sqlx::query(query).bind(video_id).execute(&pool).await?;
        Ok(())
    }


    ////////
}

//////// END
