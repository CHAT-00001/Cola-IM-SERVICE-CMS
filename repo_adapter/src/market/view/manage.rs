// repo_adapter/src/market/view/manage.rs
// 🔌 适配器 - MARKET - 商品浏览 - 管理
// 2026/8/8 12:00

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::market::info::goods::view::GoodsViewInfo;
use port::market::view::manage::GoodsViewManagePort;

////////

/// # [MANAGE ADAPTER] - 管理
/// * `desc`: `商品浏览管理服务适配器`
pub struct GoodsViewManageAdapter;

////////

#[async_trait]
impl GoodsViewManagePort for GoodsViewManageAdapter {
    //

    ////////

    /// # [ADAPTER] - 管理员列表
    async fn admin_get_views_infos(
        &self,
        _uid: i64,
        _user_id: Option<i64>,
        _video_id: Option<i64>,
        _start_time: Option<i64>,
        _end_time: Option<i64>,
        _status_code: i16,
        _limit: i64,
        _offset: i64,
    ) -> Result<(Vec<GoodsViewInfo>), u64> {
        todo!()
    }
}

//////// END
