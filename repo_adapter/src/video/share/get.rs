// repo_adapter/src/cola_video/share/get.rs
// 🔌 插头 - 可乐视频 - 分享 - 获取
// 2026/8/6 18:57 Created.

////////


// repo_adapter/src/cola_user/ban/del.rs
// 🔌 适配器 - 可乐用户 - 浏览 - 删除服务
// 2026/8/7 05:56 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_user::port::view::del::ViewDelPort;

////////

/// # [DEL SERVICE] - 删除
/// * `desc`: `用户浏览删除服务`
pub struct ViewDelService;

// 构造实现
#[async_trait]
impl ViewDelPort for ViewDelService {
    //

    ////////

    /// # 1. [SERVICE] - 单个
    /// * `desc`: `单个软删除`
    async fn single_soft_del(&self, uid: i64, id: i64) -> Result<(u16)> {
        todo!()
    }

    /// # 2. [SERVICE] - 批量
    /// * `desc`: `批量软删除`
    async fn batch_soft_del(&self, uid: i64, ids: Vec<i64>) -> Result<(u16)> {
        todo!()
    }
}

//////// END
