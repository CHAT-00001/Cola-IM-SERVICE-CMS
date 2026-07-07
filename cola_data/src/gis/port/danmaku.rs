// /danmaku.rs  -- 弹幕 服务 端口
// 2026/7/7

//////

use crate::gis::command::danmaku::PoiDanmakuCommand;
use crate::gis::info::danmaku::PoiDanmakuInfo;

//////

/// # [PORT] - 弹幕
#[async_trait::async_trait]
pub trait DanmakuRepo: Send + Sync {

    ////////

    /// # 1. [PORT] - 保存弹幕记录
    async fn save_danmaku_record(
        &self,
        uid: i64,
        poi_id: i64,
        cmd: PoiDanmakuCommand,
    ) -> anyhow::Result<()>;

    ////////

    /// # 2. [PORT] - 编辑弹幕
    async fn edit_danmaku_record(
        &self,
        uid: i64,
        danmaku_id: i64,
        cmd: PoiDanmakuCommand,
    ) -> anyhow::Result<()>;

    ////////

    /// # 3. [PORT] - 删除弹幕
    async fn del_danmaku_record(
        &self,
        uid: i64,
        danmaku_id: i64,
    ) -> anyhow::Result<()>;

    ////////

    /// # 4. [PORT] - 批量删除弹幕
    async fn del_danmakus_record(
        &self,
        uid: i64,
        danmaku_ids: Vec<i64>,
    ) -> anyhow::Result<()>;

    ////////

    /// # 5. [PORT] - 获取兴趣点ID的弹幕
    async fn get_danmaku_by_poi_id(
        &self,
        uid: i64,
        poi_id: i64,
        play_time: i32,
        qty: i32,
    ) -> anyhow::Result<(Vec<PoiDanmakuInfo>, i64)>;

    ////////

    /// # 6. [PORT] - 获取用户ID的弹幕
    async fn get_danmaku_by_user_id(
        &self,
        uid: i64,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<(Vec<PoiDanmakuInfo>, i64)>;
}