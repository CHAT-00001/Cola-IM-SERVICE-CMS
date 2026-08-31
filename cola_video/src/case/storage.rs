// D:\rust\short-video\cola_video\src\case\storage.rs
// 🗣️ CASE - 视频存储资源解析
// 2026/8/16 Created.

////////

use anyhow::{Result, anyhow};
use port::app::ctx::AppContext;
use tracing::warn;

////////

const DEFAULT_CDN_DOMAIN: &str = "https://cdn.shortvideo.com";

/// # 1. [CASE] - 解析视频业务 CDN 域名
/// * `desc`: `UGC → Bucket → CDN`
pub async fn resolve_video_cdn_domain(
    ctx: &AppContext, // 全局上下文
    app_id: &str,     // 视频业务应用标识
) -> Result<String> {
    let bucket = match ctx.fs.bucket.get.get_bucket_by_app_id(app_id).await {
        Ok(Some(bucket)) => bucket,
        Ok(None) => {
            warn!(
                "[🤐 VIDEO CASE] - ⚠️ 存储桶不存在，使用 CDN 兜底域名: app_id={}, cdn_domain={}",
                app_id, DEFAULT_CDN_DOMAIN
            );
            return Ok(DEFAULT_CDN_DOMAIN.to_string());
        }
        Err(error) => {
            warn!(
                "[🤐 VIDEO CASE] - ⚠️ 存储桶查询失败，使用 CDN 兜底域名: app_id={}, error={}, cdn_domain={}",
                app_id, error, DEFAULT_CDN_DOMAIN
            );
            return Ok(DEFAULT_CDN_DOMAIN.to_string());
        }
    };

    match bucket.cdn_domain.filter(|domain| !domain.trim().is_empty()) {
        Some(cdn_domain) => Ok(cdn_domain),
        None => {
            warn!(
                "[🤐 VIDEO CASE] - ⚠️ 存储桶未配置 CDN 域名，使用兜底域名: app_id={}, bucket={}, cdn_domain={}",
                app_id, bucket.bucket, DEFAULT_CDN_DOMAIN
            );
            Ok(DEFAULT_CDN_DOMAIN.to_string())
        }
    }
}

//////// END
