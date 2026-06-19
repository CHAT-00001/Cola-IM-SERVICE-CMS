// bridge/src/video/adapter/video  -- 桥接 - 短视 - 适配器 - 首页
// 2026/5/20 01:50 corrected by code-alignment

use app_core::video::port::home::VideoHomePort;
use async_trait::async_trait;
use data::video::entity::video::VideoEntity;
use repo::video::pg::video::VideoHomeRepo;
use std::sync::Arc;

/// # [BRIDGE] - 短视频首页服务端口的直通实现适配器
pub struct VideoAdapter {
    repo: Arc<VideoHomeRepo>,
}

impl VideoAdapter {
    pub fn new(repo: Arc<VideoHomeRepo>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl VideoHomePort for VideoAdapter {
    /// # 1. [Adapter] - 最新
    async fn find_new_list(&self, limit: i64, offset: i64) -> Result<Vec<VideoEntity>, String> {
        // 💡 修正 1：方法名改为底层真实的 find_newest
        self.repo
            .find_new_list(limit, offset)
            .await
            .map_err(|e| format!("Bridge 转发最新列表失败: {}", e))
    }

    /// # 2. [Adapter] - 热门
    async fn find_hot_list(&self, limit: i64, offset: i64) -> Result<Vec<VideoEntity>, String> {
        // 💡 修正：方法名改为 find_hottest，做 i64 转型
        self.repo
            .find_hot_list(limit, offset)
            .await
            .map_err(|e| format!("Bridge 转发热门列表失败: {}", e))
    }

    /// # 3. [Adapter] - 随机 (暂用最新兜底，或在 SQL 中加 RANDOM())
    async fn find_random_list(&self, limit: i64, offset: i64) -> Result<Vec<VideoEntity>, String> {
        self.repo
            .find_recommend_list(limit, offset)
            .await
            .map_err(|e| format!("Bridge 转发随机列表失败: {}", e))
    }

    /// # 4. [Adapter] - 附近（同城）
    async fn find_nearby_list(
        &self,
        lat: f64,
        lng: f64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoEntity>, String> {
        // 💡 修正：底层返回的是 Vec<VideoHomeRow>，需要把内部的 handler 提取并映射出来
        let rows = self
            .repo
            .find_nearby_list(lat, lng, limit, offset)
            .await
            .map_err(|e| format!("Bridge 转发附近列表失败: {}", e))?;

        // 剥离外壳，只把 Core 关心的 VideoEntity 传回去
        let entities = rows.into_iter().map(|row| row.entity).collect();
        Ok(entities)
    }

    /// # 5. [Adapter] - 分类 (频道)
    async fn find_category_list(
        &self,
        categor_id: i16,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoEntity>, String> {
        // 💡 修正：对接底层的 find_featured 业务方法
        self.repo
            .find_category_list(categor_id, limit, offset)
            .await
            .map_err(|e| format!("Bridge 转发精选列表失败: {}", e))
    }

    /// # 6. [Adapter] - 精选 (已对齐 Port 的最新更名)
    async fn find_featured_list(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoEntity>, String> {
        // 💡 修正：对接底层的 find_featured 业务方法
        self.repo
            .find_featured_list(limit, offset)
            .await
            .map_err(|e| format!("Bridge 转发精选列表失败: {}", e))
    }

    /// # 7. [Adapter] - 关键字搜索
    async fn find_keyword_list(
        &self,
        keyword: String,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoEntity>, String> {
        // 💡 修正：底层的 search_keyword 还需要 lat 和 lng 传入。
        // 由于前端首页单搜索框未传坐标，这里先固定传 0.0, 0.0，或者你可以修改 Port 结构引入坐标
        let rows = self
            .repo
            .search_keyword_list(
                &keyword,
                0.0,  // 纬度
                0.0,  // 经度
                None, // start_time: 暂无时间筛选
                None, // end_time: 暂无时间筛选
                None, // order_by: 暂无特定排序，底层默认会走 Distance
                limit, offset,
            )
            .await
            .map_err(|e| format!("Bridge 转发关键词搜索失败: {}", e))?;

        // 同样做一次剥离外壳映射
        let entities = rows.into_iter().map(|row| row.entity).collect();
        Ok(entities)
    }
}
