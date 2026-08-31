// repository/src/cola_live/pg/category_repo.rs
// 仓储 - LIVE - pg - 直播分类
// 2026/8/20 21:14 Created.

////////

use crate::pg_pool;
use anyhow::{Context, Result, anyhow};
use cola_data::cola_live::command::class::LiveClassCommand;
use cola_data::cola_live::entity::cate::class::LiveClassEntity;

////////

/// # 1. [REPOSITORY] - 直播分类仓储
pub struct LiveCategoryRepo;

impl LiveCategoryRepo {
    /// # 1. [REPOSITORY] - 创建直播分类
    /// * `desc`: `写入直播分类并返回完整实体`
    pub async fn create(uid: i64, command: LiveClassCommand) -> Result<LiveClassEntity> {
        if uid <= 0 || command.name.trim().is_empty() {
            return Err(anyhow!("操作者和分类名称不能为空"));
        }
        let pool = pg_pool();
        sqlx::query_as::<_, LiveClassEntity>(r#"INSERT INTO cola_live.live_class
            (uid, name, name_en, icon, action_uid, sort, is_hot, is_recommended, status, add_time, upd_time, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $1, COALESCE($5, 9999), COALESCE($6, 0), COALESCE($7, 0), 1, EXTRACT(EPOCH FROM NOW())::BIGINT, EXTRACT(EPOCH FROM NOW())::BIGINT, NOW(), NOW())
            RETURNING id, uid, name, name_en, icon, action_uid, sort, is_hot, is_recommended, status, add_time, upd_time, created_at, updated_at"#)
            .bind(uid).bind(command.name.trim()).bind(command.name_en.unwrap_or_default()).bind(command.icon)
            .bind(command.sort).bind(command.is_hot).bind(command.is_recommended).fetch_one(&pool).await.context("创建直播分类失败")
    }

    /// # 2. [REPOSITORY] - 修改直播分类
    /// * `desc`: `更新分类内容`
    pub async fn edit(uid: i64, command: LiveClassCommand) -> Result<LiveClassEntity> {
        if uid <= 0 || command.id <= 0 || command.name.trim().is_empty() {
            return Err(anyhow!("操作者、分类ID和分类名称不能为空"));
        }
        let pool = pg_pool();
        sqlx::query_as::<_, LiveClassEntity>(r#"UPDATE cola_live.live_class SET name=$2, name_en=$3, icon=$4, action_uid=$1, sort=COALESCE($5,sort), is_hot=COALESCE($6,is_hot), is_recommended=COALESCE($7,is_recommended), upd_time=EXTRACT(EPOCH FROM NOW())::BIGINT, updated_at=NOW() WHERE id=$8 RETURNING id, uid, name, name_en, icon, action_uid, sort, is_hot, is_recommended, status, add_time, upd_time, created_at, updated_at"#)
            .bind(uid).bind(command.name.trim()).bind(command.name_en.unwrap_or_default()).bind(command.icon).bind(command.sort).bind(command.is_hot).bind(command.is_recommended).bind(command.id)
            .fetch_optional(&pool).await.context("修改直播分类失败")?.ok_or_else(|| anyhow!("直播分类不存在"))
    }

    /// # 3. [REPOSITORY] - 修改分类状态
    pub async fn change_status(uid: i64, id: i64, status: i16) -> Result<LiveClassEntity> {
        if uid <= 0 || id <= 0 || !matches!(status, 0 | 1) {
            return Err(anyhow!("分类ID或状态参数错误"));
        }
        let pool = pg_pool();
        sqlx::query_as::<_, LiveClassEntity>(r#"UPDATE cola_live.live_class SET status=$2, action_uid=$1, upd_time=EXTRACT(EPOCH FROM NOW())::BIGINT, updated_at=NOW() WHERE id=$3 RETURNING id, uid, name, name_en, icon, action_uid, sort, is_hot, is_recommended, status, add_time, upd_time, created_at, updated_at"#)
            .bind(uid).bind(status).bind(id).fetch_optional(&pool).await.context("修改直播分类状态失败")?.ok_or_else(|| anyhow!("直播分类不存在"))
    }

    /// # 4. [REPOSITORY] - 删除直播分类
    pub async fn delete(id: i64) -> Result<u64> {
        if id <= 0 {
            return Err(anyhow!("分类ID必须大于0"));
        }
        let pool = pg_pool();
        Ok(sqlx::query("DELETE FROM cola_live.live_class WHERE id=$1")
            .bind(id)
            .execute(&pool)
            .await
            .context("删除直播分类失败")?
            .rows_affected())
    }

    /// # 5. [REPOSITORY] - 查询直播分类
    pub async fn get(id: i64) -> Result<Option<LiveClassEntity>> {
        let pool = pg_pool();
        Ok(sqlx::query_as::<_, LiveClassEntity>("SELECT id, uid, name, name_en, icon, action_uid, sort, is_hot, is_recommended, status, add_time, upd_time, created_at, updated_at FROM cola_live.live_class WHERE id=$1").bind(id).fetch_optional(&pool).await.context("查询直播分类失败")?)
    }

    /// # 6. [REPOSITORY] - 查询直播分类列表
    pub async fn list(
        status: Option<i16>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<LiveClassEntity>> {
        let pool = pg_pool();
        Ok(sqlx::query_as::<_, LiveClassEntity>("SELECT id, uid, name, name_en, icon, action_uid, sort, is_hot, is_recommended, status, add_time, upd_time, created_at, updated_at FROM cola_live.live_class WHERE ($1::SMALLINT IS NULL OR status=$1) ORDER BY sort ASC, id ASC LIMIT $2 OFFSET $3").bind(status).bind(limit).bind(offset).fetch_all(&pool).await.context("查询直播分类列表失败")?)
    }
}

//////// END
