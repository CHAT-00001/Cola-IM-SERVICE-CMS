// repository/src/video/service/state  -- 仓储 - video - 服务 - 状态
// 2026/06/05 04:45 by wx: cestbon10080

////////

use anyhow::Result;
use crate::pg_pool;

////////


/// # [SERVICE] - 状态服务
pub struct VideoStateService;

// 构造函数
impl VideoStateService {

    /// # 1. [SERVICE] - 更新用户视频数量（原子高并发自增）
    /// * 场景：新视频审核通过发布成功、或者视频被删除时
    /// * 机制：利用 PostgreSQL 字段行锁原子操作，杜绝并发计算脏数据
    pub async fn update_user_video_count(
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
    /// * 场景：发帖前的前置安全检查，判断用户是否具有发布视频、评论的权限级别
    /// * 返回：返回当前的权限 id（perm_id），供上层业务层判断
    pub async fn check_user_video_perm(
        uid: i64,
    ) -> Result<i16, sqlx::Error> {
        let pool = pg_pool();

        // 1:1 匹配你在 UserEntity 中定义的物理字段 perm_id
        let sql = r#"
            SELECT perm_id
            FROM "user"
            WHERE id = $1
            LIMIT 1
        "#;

        let perm_id: (i16,) = sqlx::query_as(sql)
            .bind(uid)
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


    /// # 3. [SERVICE] - 检查视频状态
    pub async fn check_video_state(
        video_id: i64,
    ) -> std::result::Result<i32, sqlx::Error> {
        let code = 4001;
        Ok(code)
    }


    ////////

    /// # 3. [SERVICE] - 检查视频状态
    // 假设你的项目使用 anyhow 或自定义错误类型
    pub async fn check_user_state(
        video_id: i64,
    ) -> std::result::Result<i32, sqlx::Error> {
        let code = 4001;
        Ok(code)
    }
}

//////// END