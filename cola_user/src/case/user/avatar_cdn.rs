// cola_user/src/case/user/avatar_cdn.rs -- 用户头像 CDN 解析
// 2026/9/23 Created.

////////

use port::app::ctx::AppContext;
use tracing::warn;

////////

const DEFAULT_CDN_DOMAIN: &str = "https://cdn1.damawei.com";
pub const USER_AVATAR_BUCKET_APP_ID: &str = "user-avatar";

pub fn default_avatar_cdn_domain() -> &'static str {
    DEFAULT_CDN_DOMAIN
}

/// # [CASE] - 获取头像 CDN 域名
pub async fn resolve_avatar_cdn_domain(ctx: &AppContext) -> String {
    match ctx
        .fs
        .bucket
        .get
        .get_bucket_by_app_id(USER_AVATAR_BUCKET_APP_ID)
        .await
    {
        Ok(Some(bucket)) => bucket
            .cdn_domain
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| DEFAULT_CDN_DOMAIN.to_string()),
        Ok(None) => DEFAULT_CDN_DOMAIN.to_string(),
        Err(error) => {
            warn!(
                "[🤐 USER CASE] - ❌️ 头像 bucket 查询失败，使用默认 CDN: {}",
                error
            );
            DEFAULT_CDN_DOMAIN.to_string()
        }
    }
}

/// # [HELPER] - 拼接头像 CDN 地址
pub fn resolve_avatar_url(path: &str, cdn_domain: &str) -> String {
    let default_prefix = format!("{}/", DEFAULT_CDN_DOMAIN);
    if let Some(relative_path) = path.strip_prefix(&default_prefix) {
        return format!(
            "{}/{}",
            cdn_domain.trim().trim_end_matches('/'),
            relative_path
        );
    }
    if path.is_empty()
        || path.starts_with("http://")
        || path.starts_with("https://")
        || path.starts_with("//")
    {
        return path.to_string();
    }
    let domain = if cdn_domain.trim().is_empty() {
        DEFAULT_CDN_DOMAIN
    } else {
        cdn_domain.trim()
    };
    format!(
        "{}/{}",
        domain.trim_end_matches('/'),
        path.trim_start_matches('/')
    )
}

////////

#[cfg(test)]
mod tests {
    use super::resolve_avatar_url;

    #[test]
    fn resolves_avatar_paths_without_double_slashes() {
        assert_eq!(resolve_avatar_url("", "https://cdn.example"), "");
        assert_eq!(
            resolve_avatar_url("/a.png", "https://cdn.example/"),
            "https://cdn.example/a.png"
        );
        assert_eq!(
            resolve_avatar_url("a.png", "https://cdn.example/"),
            "https://cdn.example/a.png"
        );
        assert_eq!(
            resolve_avatar_url("https://origin/a.png", "https://cdn.example"),
            "https://origin/a.png"
        );
        assert_eq!(
            resolve_avatar_url("http://origin/a.png", "https://cdn.example"),
            "http://origin/a.png"
        );
        assert_eq!(
            resolve_avatar_url("https://cdn1.damawei.com/a.png", "https://custom.example/"),
            "https://custom.example/a.png"
        );
    }
}

//////// END
