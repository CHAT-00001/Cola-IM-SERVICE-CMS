// port/src/fs/bucket/list.rs
// ⏩️ 端口 - FS - 存储桶 - 列表
// 2026/8/5 02:06 Created.

////////

use cola_data::cola_fs::info::bucket::BucketInfo;

////////

/// # [LIST SERVICE] - 列表
/// * `desc`: `获取视频评论列表服务端口`
#[async_trait::async_trait]
pub trait BucketListPort: Send + Sync {
    /// # 1. [PORT] - 管理员分页查询存储桶
    /// * `desc`: `查询全部存储桶，支持 app_id 和 keyword 条件`
    async fn admin_find_page(
        &self,
        app_id: Option<&str>,  // 应用 ID
        keyword: Option<&str>, // 搜索关键词
        limit: i64,            // 数量
        offset: i64,           // 偏移
    ) -> anyhow::Result<(Vec<BucketInfo>, i64)>;
}

//////// END
