// repo_adapter/src/basic/app/get.rs -- ADAPTER - BASIC - 应用 - 获取
// 2026/8/14 15:00 Updated.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::basic::info::core::app::AppInfo;
use port::basic::app::get::AppGetPort;
use redis::AsyncCommands;
use repository::basic::pg::app::AppRepo;
use repository::pg_pool;

////////

/// # [GET ADAPTER] - 获取
/// * `desc`: `FS - 存储桶获取适配器`
#[derive(Debug, Default, Clone)]
pub struct AppGetAdapter;

const BUCKET_CACHE_TTL: u64 = 300;

async fn get_cache(key: &str) -> Result<Option<AppInfo>> {
    let db = app_config::GLOBAL_DB
        .get()
        .ok_or_else(|| anyhow::anyhow!("GLOBAL_DB 未初始化"))?;
    let mut conn = db.redis_conn.clone();
    let value: Option<String> = conn.get(key).await?;
    value
        .map(|json| serde_json::from_str(&json).map_err(Into::into))
        .transpose()
}

async fn set_cache(key: &str, info: &AppInfo) -> Result<()> {
    let db = app_config::GLOBAL_DB
        .get()
        .ok_or_else(|| anyhow::anyhow!("GLOBAL_DB 未初始化"))?;
    let mut conn = db.redis_conn.clone();
    let json = serde_json::to_string(info)?;
    let _: () = conn.set_ex(key, json, BUCKET_CACHE_TTL).await?;
    Ok(())
}

#[async_trait]
impl AppGetPort for AppGetAdapter {
    ////////

    /// # 1. [ADAPTER] - 按 app_id 查询
    /// * `desc`: `根据应用 ID 查询存储桶配置`
    async fn get_app_by_app_id(
        &self,
        app_id: &str, // 应用 ID
    ) -> Result<Option<AppInfo>> {
        let key = format!("fs:bucket:app:{}", app_id);
        if let Ok(Some(info)) = get_cache(&key).await {
            tracing::info!("[🔌 ADAPTER] - ⚡️ Bucket缓存命中: app_id={}", app_id);
            return Ok(Some(info));
        }

        let result = AppRepo::find_by_app_id(&pg_pool(), app_id).await?;
        let info = result.map(AppInfo::from);
        if let Some(info_ref) = info.as_ref() {
            if let Err(error) = set_cache(&key, info_ref).await {
                tracing::warn!(
                    "[🔌 ADAPTER] - ⚠️ Bucket缓存写入失败: app_id={}, error={}",
                    app_id,
                    error
                );
            }
        }
        tracing::info!(
            "[🔌 ADAPTER] - ✅️ 按 app_id 查询存储桶完成: app_id={}, found={}",
            app_id,
            info.is_some()
        );
        Ok(info)
    }

    ////////

    /// # 2. [ADAPTER] - 按 ID 查询
    /// * `desc`: `根据存储桶 ID 查询`
    async fn get_app_by_id(
        &self,
        app_id: i64, // 存储桶 ID
    ) -> Result<Option<AppInfo>> {
        ////////

        // 💡 - CACHE - 缓存中获取
        let key = format!("fs:bucket:id:{}", app_id);
        if let Ok(Some(info)) = get_cache(&key).await {
            tracing::info!("[🔌 ADAPTER] - ⚡️ Bucket缓存命中: app_id={}", app_id);
            return Ok(Some(info));
        }

        ////////

        // 💡 - PG - 数据库中兜底
        let result = AppRepo::find_by_id(&pg_pool(), app_id).await?;

        ////////

        // 💡 - INFO 转换
        let info = result.map(AppInfo::from);

        // 💡 - 回填缓存
        if let Some(info_ref) = info.as_ref() {
            if let Err(error) = set_cache(&key, info_ref).await {
                tracing::warn!(
                    "[🔌 ADAPTER] - ⚠️ 应用信息缓存写入失败: app_id={}, error={}",
                    app_id,
                    error
                );
            }
        }

        ////////

        // 💡 - LOGGER
        tracing::info!(
            "[🔌 ADAPTER] - ✅️ 按 ID 查询应用完成: app_id={}, found={}",
            app_id,
            info.is_some()
        );
        Ok(info)
    }
}

/////// END
