// repo_adapter/src/user/view/add.rs
// 🔌 适配器 - 可乐用户 - 浏览 - 获取服务
// 2026/8/6 04:18 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::video::info::video::VideoInfo;
use cola_data::video::port::video::get::VideoGetPort;

////////

/// # [GET SERVICE] - 获取
/// * `desc`: `用户浏览获取服务`
pub struct ViewGetService;

// 构造实现
#[async_trait]
impl VideoGetPort for ViewGetService {
    //

    ////////

    /// # 1. [SERVICE] - 我的
    /// * `desc`: `单个软删除`
    async fn get_my_list(
        &self,
        uid: i64,
        keyword: Option<String>,
        limit: i64,
        offset: i64,
        is_liked: bool,
    ) -> Result<(Vec<VideoInfo>)> {
        todo!()
    }

    async fn get_he_list(
        &self,
        uid: i64,
        keyword: Option<String>,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<VideoInfo>)> {
        todo!()
    }

    async fn get_nearby_list(
        &self,
        lat: f64,
        lng: f64,
        range: f64,
        offset: i64,
        limit: i64,
    ) -> Result<(Vec<VideoInfo>)> {
        todo!()
    }
}

//////// END
