// repo_adapter/src/market/cate/stat.rs
// 🔌 适配器 -MARKET - 商品分类 - 统计
// 2026/8/6 19:20 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::market::cate::stat::CateStatPort;

////////

/// # [STAT ADAPTER] - 统计
/// * `desc`: `商品分类统计服务`
pub struct CateStatAdapter;

// 构造实现
#[async_trait]
impl CateStatPort for CateStatAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 统计用户的视频数量
    async fn stat_count_by_user_id(&self, uid: i64, user_id: i64) -> Result<(u64)> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 统计所有视频数量
    async fn stat_count(&self) -> Result<(u64)> {
        todo!()
    }
}

//////// END
